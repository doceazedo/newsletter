use std::net::TcpListener;

use sqlx::PgPool;

use zero2prod::config::get_config;
use zero2prod::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ip = "127.0.0.1";
    let config = get_config();
    let db_uri = config.database.get_uri();

    let db = PgPool::connect(&db_uri)
        .await
        .expect("Could not connect to database");
    let listener = TcpListener::bind(format!("{}:{}", &ip, &config.port))
        .expect("Could not bind to random port");
    let port = listener.local_addr().unwrap().port();

    println!("Server is running on http://{}:{}", ip, port);
    run(listener, db).await
}
