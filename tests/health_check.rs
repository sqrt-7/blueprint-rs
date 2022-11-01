use actix_web::http;
use std::{collections::HashMap, net::TcpListener};
use zero2prod::values;

struct TestServer {
    addr: String,
}

impl TestServer {
    fn new(addr: String) -> Self {
        TestServer { addr }
    }
}

fn spawn_app() -> TestServer {
    let listener = TcpListener::bind(values::LOCALHOST_RANDOM)
        .expect("failed to bind random port");

    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).unwrap_or_else(|err| {
        panic!("failed to spawn server: {}", err);
    });

    tokio::spawn(server);

    TestServer::new(format!("http://{}:{}", values::LOCALHOST, port))
}

#[tokio::test]
async fn health_check_success() {
    let srv = spawn_app();
    let client = reqwest::Client::new();
    let endpoint = format!("{}{}", srv.addr, values::ROUTES_HEALTHCHECK);

    let resp = client
        .get(endpoint)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(http::StatusCode::OK, resp.status());
    assert_eq!(Some(0), resp.content_length());
}

#[tokio::test]
async fn subscribe_form_valid_returns_200() {
    let srv = spawn_app();
    let client = reqwest::Client::new();
    let endpoint = format!("{}{}", srv.addr, values::ROUTES_SUBSCRIPTIONS);

    let mut form = HashMap::new();
    form.insert("name", "Jeff Jeffries");
    form.insert("email", "jeff@bob.com");

    let resp = client
        .post(endpoint)
        .form(&form)
        .send()
        .await
        .expect("failed to execute request");

    assert_eq!(http::StatusCode::OK, resp.status());
}

#[tokio::test]
async fn subscribe_form_invalid_returns_400() {
    let srv = spawn_app();
    let client = reqwest::Client::new();
    let endpoint = format!("{}{}", srv.addr, values::ROUTES_SUBSCRIPTIONS);

    let mut form = HashMap::new();
    form.insert("name", "Jeff Jeffries");

    let resp = client
        .post(endpoint)
        .form(&form)
        .send()
        .await
        .expect("failed to execute request");

    assert_eq!(http::StatusCode::BAD_REQUEST, resp.status());
}
