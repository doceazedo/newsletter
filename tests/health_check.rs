use std::collections::HashMap;
use std::net::TcpListener;

use actix_web::rt::spawn;
use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use sqlx::{migrate, query, Executor, PgPool};
use uuid::Uuid;

use zero2prod::config::{get_config, DatabaseSettings};
use zero2prod::startup::create_server;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber = get_subscriber(
        "test".to_string(),
        "debug".to_string(),
        std::env::var("TEST_LOG").is_err(),
    );
    init_subscriber(subscriber);
});

struct App {
    url: String,
    db: PgPool,
}

async fn spawn_app() -> App {
    Lazy::force(&TRACING);
    let mut config = get_config();
    let db = setup_mock_database(&mut config.database).await;
    let listener =
        TcpListener::bind(format!("{}:0", &config.ip)).expect("Could not bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = create_server(listener, db.clone()).expect("Could not get server");
    let _ = spawn(server);
    App {
        url: format!("http://{}:{}", &config.ip, port),
        db,
    }
}

async fn setup_mock_database(db_settings: &mut DatabaseSettings) -> PgPool {
    let db_uri = db_settings.get_uri_without_db();
    db_settings.name = Uuid::new_v4().to_string();

    // create database
    let db = PgPool::connect(db_uri.expose_secret())
        .await
        .expect("Could not connect to database");
    db.execute(format!(r#"CREATE DATABASE "{}";"#, db_settings.name).as_str())
        .await
        .expect("Could not create mock database");

    // migrate database
    let db_uri = db_settings.get_uri();
    let db = PgPool::connect(db_uri.expose_secret())
        .await
        .expect("Could not connect to mock database");
    migrate!()
        .run(&db)
        .await
        .expect("Could not migrate mock database");

    db
}

#[actix_web::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let status = reqwest::get(format!("{}/health_check", &app.url))
        .await
        .expect("Could not make request")
        .status();
    assert!(status.is_success());
}

#[actix_web::test]
async fn subscribe_returns_200_for_valid_data() {
    let app = spawn_app().await;

    let mut body = HashMap::new();
    body.insert("email", "doce@example.com");
    body.insert("name", "Doce");

    let client = reqwest::Client::new();
    let status = client
        .post(format!("{}/subscribe", &app.url))
        .json(&body)
        .send()
        .await
        .expect("Could not make request")
        .status();
    assert!(status.is_success());

    let subscription = query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db)
        .await
        .expect("Could not fetch subscription");

    assert_eq!(subscription.email, body.get("email").unwrap().to_string());
    assert_eq!(subscription.name, body.get("name").unwrap().to_string());
}

#[actix_web::test]
async fn subscribe_returns_400_for_invalid_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("body with no email", "name", "Doce"),
        ("body with no name", "email", "doce@example.com"),
        ("empty body", "", ""),
    ];

    for (case, key, value) in test_cases {
        let mut body = HashMap::new();
        if !key.is_empty() && !value.is_empty() {
            body.insert(key, value);
        }
        let status = client
            .post(format!("{}/subscribe", &app.url))
            .json(&body)
            .send()
            .await
            .expect("Could not make request")
            .status();
        assert_eq!(
            status.as_u16(),
            400,
            "API did not return 400 when request has {}",
            case
        );
    }
}
