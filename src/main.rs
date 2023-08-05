use zero2prod::config::get_config;
use zero2prod::email_client::EmailClient;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

use std::net::TcpListener;

use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod", "info", std::io::stdout);
    init_subscriber(subscriber);

    let config = get_config().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(&config.database.connection_string().expose_secret())
        .expect("Failed to connection to Postgres.");
    let socket = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(socket)?;

    let sender = config
        .email_client
        .sender()
        .expect("Invalid send email address");
    let timeout = config.email_client.timeout();
    let email_client = EmailClient::new(
        &config.email_client.base_url,
        sender,
        config.email_client.authorization_token,
        timeout,
    );

    run(listener, connection_pool, email_client)?.await?;
    Ok(())
}
