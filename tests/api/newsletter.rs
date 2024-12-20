use std::time::Duration;

use fake::faker::internet::en::SafeEmail;
use fake::faker::name::en::Name;
use fake::Fake;
use uuid::Uuid;
use wiremock::matchers::{any, method, path};
use wiremock::{Mock, ResponseTemplate};

use crate::helpers::{assert_is_redirect_to, spawn_app, ConfirmationLinks, TestApp};

#[tokio::test]
async fn newsletters_not_delivered_to_unconfirmed_subscribers() {
    let test_app = spawn_app().await;
    create_unconfirmed_subscriber(&test_app).await;

    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount(&test_app.email_server)
        .await;
    test_app.post_login_test_user().await;

    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as HTML</p>",
       "idempotency_key": Uuid::new_v4().to_string(),
    });

    let response = test_app
        .post_publish_newsletters(&newsletter_request_body)
        .await;

    assert_eq!(response.status().as_u16(), 303);
    test_app.dispatch_all_pending_emails().await;
}

#[tokio::test]
async fn newsletters_delivered_to_confirmed_subscribers() {
    let test_app = spawn_app().await;
    create_confirmed_subscriber(&test_app).await;
    test_app.post_login_test_user().await;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&test_app.email_server)
        .await;

    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as HTML</p>",
       "idempotency_key": Uuid::new_v4().to_string(),
    });
    let response = test_app
        .post_publish_newsletters(&newsletter_request_body)
        .await;

    assert_eq!(response.status().as_u16(), 303);
    test_app.dispatch_all_pending_emails().await;
}

async fn create_unconfirmed_subscriber(test_app: &TestApp) -> ConfirmationLinks {
    let name: String = Name().fake();
    let email: String = SafeEmail().fake();

    let body = serde_urlencoded::to_string(&serde_json::json!({
        "name": name,
        "email": email
    }))
    .unwrap();

    let _mock_guard = Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .named("Create unconfirmed subscriber.")
        .expect(1)
        .mount_as_scoped(&test_app.email_server)
        .await;

    test_app
        .post_subscriptions(&body)
        .await
        .error_for_status()
        .unwrap();

    let email_request = &test_app
        .email_server
        .received_requests()
        .await
        .unwrap()
        .pop()
        .unwrap();

    test_app.get_confirmation_links(email_request)
}

async fn create_confirmed_subscriber(test_app: &TestApp) {
    let confirmation_links = create_unconfirmed_subscriber(test_app).await;
    reqwest::get(confirmation_links.plain_text)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
}

#[tokio::test]
async fn newsletters_returns_400_for_invalid_data() {
    let test_app = spawn_app().await;
    test_app.post_login_test_user().await;

    let test_cases = vec![
        (
            serde_json::json!({
                "text_content": "Newsletter body as plain text",
                "html_content": "<p>Newsletter body as HTML</p>",
            }),
            "missing title",
        ),
        (
            serde_json::json!({"title": "Newsletter!"}),
            "missing content",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = test_app.post_publish_newsletters(&invalid_body).await;
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
    test_app.dispatch_all_pending_emails().await;
}

#[tokio::test]
async fn requests_missing_authorization_rejected() {
    let test_app = spawn_app().await;
    let body = serde_json::json!(
    {
        "title": "Newsletter title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as HTML</p>",
    }
    );

    let response = test_app.post_publish_newsletters(&body).await;

    assert_eq!(303, response.status().as_u16());
    test_app.dispatch_all_pending_emails().await;
}

#[tokio::test]
async fn newsletter_creation_idempotent() {
    let test_app = spawn_app().await;
    create_confirmed_subscriber(&test_app).await;
    test_app.post_login_test_user().await;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&test_app.email_server)
        .await;

    let newsletter_request_body = serde_json::json!({
       "title": "Newsletter title",
       "text_content": "Newsletter body as plain text.",
       "html_content": "<p>Newsletter body as HTML",
       "idempotency_key": Uuid::new_v4().to_string(),
    });

    let response = test_app
        .post_publish_newsletters(&newsletter_request_body)
        .await;
    assert_is_redirect_to(&response, "/admin/newsletters");

    let html_page = test_app.get_publish_newsletter_html().await;
    assert!(html_page.contains("The newsletter issue has been published!"));

    let response = test_app
        .post_publish_newsletters(&newsletter_request_body)
        .await;
    assert_is_redirect_to(&response, "/admin/newsletters");

    let html_page = test_app.get_publish_newsletter_html().await;
    assert!(html_page.contains("The newsletter issue has been published"));
    test_app.dispatch_all_pending_emails().await;
}

#[tokio::test]
async fn concurrent_form_submission_handled_gracefully() {
    let test_app = spawn_app().await;
    create_confirmed_subscriber(&test_app).await;
    test_app.post_login_test_user().await;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_secs(2)))
        .expect(1)
        .mount(&test_app.email_server)
        .await;

    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as HTML</p>",
        "idempotency_key": uuid::Uuid::new_v4().to_string()
    });

    let response1 = test_app.post_publish_newsletters(&newsletter_request_body);
    let response2 = test_app.post_publish_newsletters(&newsletter_request_body);
    let (response1, response2) = tokio::join!(response1, response2);

    assert_eq!(response1.status(), response2.status());
    assert_eq!(
        response1.text().await.unwrap(),
        response2.text().await.unwrap()
    );
    test_app.dispatch_all_pending_emails().await;
}
