fn spawn_app() {
    let server = zero2prod::run("127.0.0.1:8000").unwrap_or_else(|err| {
        panic!("Failed to spawn server: {}", err);
    });
    let _ = tokio::spawn(server);
}

#[tokio::test]
async fn health_check_success() {
    spawn_app();

    let client = reqwest::Client::new();

    let resp = client
        .get("http://127.0.0.1:8000/health-check")
        .send()
        .await
        .expect("Failed to execute request");

    assert!(resp.status().is_success());
    assert_eq!(Some(0), resp.content_length());
}
