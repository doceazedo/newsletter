use std::net::TcpListener;

use zero2prod::startup::run;

fn main() -> std::io::Result<()> {
    let ip = "127.0.0.1";
    let listener = TcpListener::bind(format!("{}:0", &ip)).expect("Could not bind to random port");
    let port = listener.local_addr().unwrap().port();
    println!("Server is running on {}:{}", ip, port);
    run(listener)
}
