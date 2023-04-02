use std::net::TcpListener;

use env_logger::Builder;
use env_logger::Env;
use sqlx::PgPool;

use zero2prod::config::get_config;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = get_config().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connection to Postgres.");
    let socket = format!("localhost:{}", config.application_port);
    let listener = TcpListener::bind(socket)?;
    run(listener, connection_pool)?.await
}
