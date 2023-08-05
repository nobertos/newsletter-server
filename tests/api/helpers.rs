use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

use std::net::TcpListener;

use zero2prod::config::{get_config, DatabaseSettings};
use zero2prod::email_client::EmailClient;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

static TRACING: Lazy<()> = Lazy::new(|| {
    let filter_level = "debug";
    let subscriber_name = "test";
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, filter_level, std::io::stdout);
        init_subscriber(subscriber);
        return;
    }
    let subscriber = get_subscriber(subscriber_name, filter_level, std::io::sink);
    init_subscriber(subscriber);
});

pub struct TestApp {
    pub url: String,
    pub db_pool: PgPool,
}

async fn configure_db(config_db: &DatabaseSettings) -> PgPool {
    let mut connection =
        PgConnection::connect(&config_db.connection_string_without_db().expose_secret())
            .await
            .expect("Failed to connect to Postgres.");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config_db.database_name).as_str())
        .await
        .expect("Failed to create database");

    let connection_pool = PgPool::connect(&config_db.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database.");

    connection_pool
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);
    let listener = TcpListener::bind("localhost:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let url = format!("http://localhost:{}", port);

    let mut config = get_config().expect("Failed to read configuration");
    config.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_db(&config.database).await;

    let sender_email = config
        .email_client
        .sender()
        .expect("Invalid sender email address.");
    let timeout = config.email_client.timeout();
    let email_client = EmailClient::new(
        &url,
        sender_email,
        config.email_client.authorization_token,
        timeout,
    );

    let server =
        run(listener, connection_pool.clone(), email_client).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        url,
        db_pool: connection_pool,
    }
}
