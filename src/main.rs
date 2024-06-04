use std::net::TcpListener;

use sqlx::PgPool;

use zero2prod::config::get_config;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".to_string(), "info".to_string(), false);
    init_subscriber(subscriber);

    let config = get_config();
    let db_options = config.database.get_options();

    let db = PgPool::connect_lazy_with(db_options);
    let listener = TcpListener::bind(format!("{}:{}", &config.ip, &config.port))
        .expect("Could not bind to port");
    let port = listener.local_addr().unwrap().port();

    println!("Server is running on http://{}:{}", &config.ip, port);
    run(listener, db).await
}
