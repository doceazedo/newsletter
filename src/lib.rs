use actix_web::dev::Server;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

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

pub fn get_server(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(health_check)
    })
    .listen(listener)?
    .run();
    Ok(server)
}

#[actix_web::main]
pub async fn run(listener: TcpListener) -> std::io::Result<()> {
    get_server(listener)?.await
}
