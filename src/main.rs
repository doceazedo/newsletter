use std::net::TcpListener;

use zero2prod::config::get_config;
use zero2prod::startup::run;

fn main() -> std::io::Result<()> {
    let config = get_config().expect("Could not load config");
    let ip = "127.0.0.1";
    let listener = TcpListener::bind(format!("{}:{}", &ip, &config.port))
        .expect("Could not bind to random port");
    let port = listener.local_addr().unwrap().port();
    println!("Server is running on http://{}:{}", ip, port);
    run(listener)
}
