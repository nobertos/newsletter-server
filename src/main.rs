use std::net::TcpListener;

use zero2prod::config::get_config;
use zero2prod::startup::run;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_config().expect("Failed to read configuration.");
    let socket = format!("localhost:{}", config.application_port);
    let listener = TcpListener::bind(socket)?;
    run(listener)?.await
}
