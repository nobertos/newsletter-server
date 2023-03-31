use std::net::TcpListener;

use sqlx::{Connection, PgConnection};
use zero2prod::config::get_config;
use zero2prod::startup::run;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_config().expect("Failed to read configuration.");
    let connection = PgConnection::connect(&config.database.connection_string())
        .await
        .expect("Failed to connection to Postgres.");
    let socket = format!("localhost:{}", config.application_port);
    let listener = TcpListener::bind(socket)?;
    run(listener, connection)?.await
}
