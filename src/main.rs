use std::net::TcpListener;

use secrecy::ExposeSecret;
use sqlx::PgPool;

use zero2prod::config::get_config;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".to_string(), "info".to_string(), false);
    init_subscriber(subscriber);

    let config = get_config();
    let db_uri = config.database.get_uri();

    let db = PgPool::connect_lazy(db_uri.expose_secret())
        .expect("Could not connect to database");
    let listener = TcpListener::bind(format!("{}:{}", &config.ip, &config.port))
        .expect("Could not bind to random port");
    let port = listener.local_addr().unwrap().port();

    println!("Server is running on http://{}:{}", &config.ip, port);
    run(listener, db).await
}
