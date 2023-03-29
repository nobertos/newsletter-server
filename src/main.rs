use std::net::TcpListener;

use zero2prod::run;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:8000").expect("Failed to bind an address");
    run(listener)?.await
}
