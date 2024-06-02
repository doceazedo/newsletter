use std::net::TcpListener;

use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, Result, web};
use actix_web::dev::Server;
use serde::Deserialize;

#[derive(Deserialize)]
struct SubscribeRequest {
    name: String,
    email: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/subscribe")]
async fn subscribe(req: web::Json<SubscribeRequest>) -> Result<String> {
    Ok(format!("Hey {} ({})!", req.name, req.email))
}

pub fn get_server(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(health_check)
            .service(subscribe)
    })
        .listen(listener)?
        .run();
    Ok(server)
}

#[actix_web::main]
pub async fn run(listener: TcpListener) -> std::io::Result<()> {
    get_server(listener)?.await
}
