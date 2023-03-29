use std::{fmt::format, net::TcpListener};

#[tokio::test]
async fn health_check_works() {
    let url = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &url))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("localhost:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://localhost:{}", port)
}
