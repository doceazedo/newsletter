use std::collections::HashMap;
use std::net::TcpListener;

use actix_web::rt::spawn;
use sqlx::postgres::{PgPool, Postgres};
use sqlx::{query, Pool};

use zero2prod::config::get_config;
use zero2prod::startup::create_server;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Could not bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = create_server(listener).expect("Could not get server");
    let _ = spawn(server);
    format!("http://127.0.0.1:{port}")
}

async fn connect_to_database() -> Pool<Postgres> {
    let config = get_config().unwrap();
    let database_uri = config.database.get_uri();
    PgPool::connect(&database_uri).await.expect("")
}

#[actix_web::test]
async fn health_check_works() {
    let url = spawn_app();
    let status = reqwest::get(format!("{}/health_check", &url))
        .await
        .expect("Could not make request")
        .status();
    assert!(status.is_success());
}

#[actix_web::test]
async fn subscribe_returns_200_for_valid_data() {
    let url = spawn_app();

    let mut body = HashMap::new();
    body.insert("email", "doce@example.com");
    body.insert("name", "Doce");

    let client = reqwest::Client::new();
    let status = client
        .post(format!("{}/subscribe", &url))
        .json(&body)
        .send()
        .await
        .expect("Could not make request")
        .status();
    assert!(status.is_success());

    let pg = connect_to_database().await;
    let subscription = query!("SELECT email, name FROM subscriptions")
        .fetch_one(&pg)
        .await
        .expect("Could not fetch subscription");

    assert_eq!(subscription.email, body.get("email").unwrap().to_string());
    assert_eq!(subscription.name, body.get("name").unwrap().to_string());
}

#[actix_web::test]
async fn subscribe_returns_400_for_invalid_data() {
    let url = spawn_app();
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
            .post(format!("{}/subscribe", &url))
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
