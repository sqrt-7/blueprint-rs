mod helpers;

use actix_web::http;

#[tokio::test]
async fn health_check_success() {
    let srv = helpers::spawn_app();
    let client = reqwest::Client::new();
    let endpoint = format!("{}/healthz", srv.basepath);

    let resp = client
        .get(endpoint)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(http::StatusCode::OK, resp.status());
    assert_eq!(Some(0), resp.content_length());
}
