use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{App, HttpServer};

use crate::routes::*;

pub fn create_server(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check).service(subscribe))
        .listen(listener)?
        .run();
    Ok(server)
}

#[actix_web::main]
pub async fn run(listener: TcpListener) -> std::io::Result<()> {
    create_server(listener)?.await
}
