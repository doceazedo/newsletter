use std::net::TcpListener;
use actix_web::rt::spawn;
use zero2prod::get_server;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Could not bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = get_server(listener).expect("Could not get server");
    let _ = spawn(server);
    format!("http://127.0.0.1:{port}")
}

#[actix_web::test]
async fn health_check_works() {
    let url = spawn_app();
    let status = reqwest::get(format!("{}/health_check", &url)).await.unwrap().status();
    assert!(status.is_success());
}