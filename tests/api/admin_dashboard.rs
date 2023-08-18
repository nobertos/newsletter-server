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
