use std::io::Error;
use std::net::TcpListener;

use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

use crate::routes::*;

pub fn create_server(listener: TcpListener, db: PgPool) -> Result<Server, Error> {
    let db = web::Data::new(db);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(health_check)
            .service(subscribe)
            .app_data(db.clone())
    })
        .listen(listener)?
        .run();
    Ok(server)
}

pub async fn run(listener: TcpListener, db: PgPool) -> std::io::Result<()> {
    create_server(listener, db)
        .expect("Could not create server")
        .await
}
