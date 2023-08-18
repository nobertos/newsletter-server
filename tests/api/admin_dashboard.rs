use uuid::Uuid;

use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn must_be_logged_in_to_access_admin_dashboard() {
    let test_app = spawn_app().await;

    let response = test_app.get_admin_dashboard().await;

    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn logout_clears_session_state() {
    let test_app = spawn_app().await;

    let response = test_app.post_login_test_user().await;
    assert_is_redirect_to(&response, "/admin/dashboard");

    let html_page = test_app.get_admin_dashboard_html().await;
    assert!(html_page.contains(&format!("Welcome {}", test_app.test_user.username)));

    let response = test_app.post_logout().await;
    assert_is_redirect_to(&response, "/login");

    let html_page = test_app.get_login_html().await;
    assert!(html_page.contains("You have successfully logged out."));

    let response = test_app.get_admin_dashboard().await;
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn changing_password_works() {
    let test_app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();

    let response = test_app.post_login_test_user().await;
    assert_is_redirect_to(&response, "/admin/dashboard");

    let response = test_app
        .post_change_password(&serde_json::json!({
            "current_password": &test_app.test_user.password,
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;
    assert_is_redirect_to(&response, "/admin/password");

    let html_page = test_app.get_login_html().await;
    assert!(html_page.contains("Your password has been changed."));

    let response = test_app.post_logout().await;
    assert_is_redirect_to(&response, "/login");

    let html_page = test_app.get_login_html().await;
    assert!(html_page.contains("You have successfully logged out."));

    let login_body = serde_json::json!({
        "username": &test_app.test_user.username,
        "password": &new_password
    });
    let response = test_app.post_login(&login_body).await;
    assert_is_redirect_to(&response, "/admin/dashboard");
}
