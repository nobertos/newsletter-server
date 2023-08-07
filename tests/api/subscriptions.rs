use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

use crate::helpers::spawn_app;

#[tokio::test]
async fn subscribe_returns_200_for_valid_form() {
    let test_app = spawn_app().await;
    let body = "name=rayene%20nassim&email=jr_zorgani%40esi.dz";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&test_app.email_server)
        .await;

    let response = test_app.post_subscriptions(body).await;
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved subscriptions.");
    assert_eq!(saved.email, "jr_zorgani@esi.dz");
    assert_eq!(saved.name, "rayene nassim");
}

#[tokio::test]
async fn subscribe_returns_400_when_data_missing() {
    let test_app = spawn_app().await;
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = test_app.post_subscriptions(invalid_body).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        )
    }
}

#[tokio::test]
async fn subscribe_returns_400_when_fields_are_present_but_empty() {
    let test_app = spawn_app().await;

    let test_cases = vec![
        ("name=&email=ursula_le_guin%40gmail.com", "empty name"),
        ("name=Ursula&email=", "empty email"),
        ("name=Ursula&email=definitely-not-an-email", "invalid email"),
    ];

    for (body, desc) in test_cases {
        let response = test_app.post_subscriptions(body).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 200 OK when payload was {:#?}",
            desc
        )
    }
}

#[tokio::test]
async fn subscribe_sends_confirmation_email_for_valid_data() {
    let test_app = spawn_app().await;
    let body = "name=nassim%20rayene&email=jr_zorgani%40esi.dz";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&test_app.email_server)
        .await;

    test_app.post_subscriptions(body.into()).await;
}
