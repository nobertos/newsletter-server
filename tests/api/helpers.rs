use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

use zero2prod::config::{get_config, DatabaseSettings};
use zero2prod::startup::{get_connection_pool, Application};
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

impl TestApp {
    pub async fn post_subscriptions(&self, body: &str) -> reqwest::Response {
        reqwest::Client::new()
            .post(&format!("{}/subscriptions", &self.url))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body.to_string())
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

async fn configure_db(config_db: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config_db.without_db())
        .await
        .expect("Failed to connect to Postgres.");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config_db.database_name).as_str())
        .await
        .expect("Failed to create database");

    let connection_pool = PgPool::connect_with(config_db.with_db())
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

    let config = {
        let mut c = get_config().expect("Failed to read configuration");
        c.database.database_name = Uuid::new_v4().to_string();
        c.application.port = 0;
        c
    };

    configure_db(&config.database).await;

    let application = Application::build(config.clone())
        .await
        .expect("Failed to build application.");
    let url = format!("http://localhost:{}", application.port());
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        url,
        db_pool: get_connection_pool(&config.database),
    }
}
