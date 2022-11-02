use actix_web::http;
use std::collections::HashMap;
use zero2prod::{
    datastore::inmem::InMemDatastore,
    service::{
        endpoints::{ROUTES_HEALTHCHECK, ROUTES_SUBSCRIPTIONS},
        Service,
    },
    LOCALHOST, LOCALHOST_RANDOM,
};

struct TestServer {
    addr: String,
}

impl TestServer {
    fn new(addr: String) -> Self {
        println!(">> starting TestServer on {}", addr);
        TestServer { addr }
    }
}

fn spawn_app() -> TestServer {
    let ds = InMemDatastore::new();
    let mut svc = Service::new(ds);

    let server = svc.start_http_server(LOCALHOST_RANDOM).unwrap();

    tokio::spawn(server);

    TestServer::new(format!("http://{}:{}", LOCALHOST, svc.http_port().unwrap()))
}

#[tokio::test]
async fn health_check_success() {
    let srv = spawn_app();
    let client = reqwest::Client::new();
    let endpoint = format!("{}{}", srv.addr, ROUTES_HEALTHCHECK);

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
    let endpoint = format!("{}{}", srv.addr, ROUTES_SUBSCRIPTIONS);

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
    let endpoint = format!("{}{}", srv.addr, ROUTES_SUBSCRIPTIONS);

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
