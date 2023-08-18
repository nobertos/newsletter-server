use uuid::Uuid;

use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn must_be_logged_in_to_see_change_password_form() {
    let test_app = spawn_app().await;

    let response = test_app.get_change_password().await;

    assert_is_redirect_to(&response, "/login")
}

#[tokio::test]
async fn must_be_logged_in_to_change_password() {
    let test_app = spawn_app().await;

    let new_password = Uuid::new_v4().to_string();

    let body = serde_json::json!({
        "current_password": Uuid::new_v4().to_string(),
        "new_password": &new_password,
        "new_password_check": &new_password,
    });
    let response = test_app.post_change_password(&body).await;

    assert_is_redirect_to(&response, "/login")
}

#[tokio::test]
async fn new_password_fields_must_match() {
    let test_app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let another_new_password = Uuid::new_v4().to_string();

    test_app
        .post_login(&serde_json::json!({
            "username": &test_app.test_user.username,
            "password": &test_app.test_user.password
        }))
        .await;

    let response = test_app
        .post_change_password(&serde_json::json!({
            "current_password": &test_app.test_user.password,
            "new_password": &new_password,
            "new_password_check": &another_new_password
        }))
        .await;

    assert_is_redirect_to(&response, "/admin/password");

    let html_page = test_app.get_change_password_html().await;
    assert!(html_page.contains("You entered two different new passwords"))
}

#[tokio::test]
async fn current_password_must_be_valid() {
    let test_app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let wrong_password = Uuid::new_v4().to_string();

    test_app.post_login_test_user().await;

    let response = test_app
        .post_change_password(&serde_json::json!({
            "current_password": &wrong_password,
            "new_password": &new_password,
            "new_password_check": &new_password
        }))
        .await;

    assert_is_redirect_to(&response, "/admin/password");

    let html_page = test_app.get_change_password_html().await;
    assert!(html_page.contains("The current password is incorrect."))
}

#[tokio::test]
async fn short_passwords_failure() {
    let test_app = spawn_app().await;
    let new_password = "123456789";

    test_app.post_login_test_user().await;

    let response = test_app
        .post_change_password(&serde_json::json!({
            "current_password": &test_app.test_user.password,
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;

    assert_is_redirect_to(&response, "/admin/password");

    let html_page = test_app.get_change_password_html().await;
    assert!(
        html_page.contains("Passwords must be longer than 12 and shorter than 128 characters.",)
    );
}
