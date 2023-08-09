use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

use crate::helpers::spawn_app;

#[tokio::test]
async fn link_returned_by_subscribe_returns_200_if_called() {
    let test_app = spawn_app().await;
    let body = "name=rayene%20nassim&email=jr_zorgani%40esi.dz";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&test_app.email_server)
        .await;

    test_app.post_subscriptions(body).await;
    let email_request = &test_app.email_server.received_requests().await.unwrap()[0];

    let mut confirmation_link = test_app.get_confirmation_links(email_request).plain_text;

    assert_eq!(confirmation_link.host_str().unwrap(), "localhost");

    confirmation_link.set_port(Some(test_app.port)).unwrap();

    let response = reqwest::get(confirmation_link).await.unwrap();

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn confirmations_without_token_are_rejected_with_400() {
    let test_app = spawn_app().await;

    let response = reqwest::get(&format!("{}/subscriptions/confirm", test_app.url))
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn clicking_confirmation_link_confirms_subscriber() {
    let test_app = spawn_app().await;
    let body = "name=rayene%20nassim&email=jr_zorgani%40esi.dz";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&test_app.email_server)
        .await;

    test_app.post_subscriptions(body.into()).await;
    let email_request = &test_app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = test_app.get_confirmation_links(email_request);

    reqwest::get(confirmation_links.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions;")
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "jr_zorgani@esi.dz");
    assert_eq!(saved.name, "rayene nassim");
    assert_eq!(saved.status, "confirmed");
}
