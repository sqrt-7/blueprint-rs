#[rustfmt::skip]
use std::collections::HashMap;

use actix_web::http;
use blueprint::logic::{
    domain::{User, ID},
    error::{ServiceError, ServiceErrorCode},
};
use serde_json;

mod helpers;

#[tokio::test]
async fn post_user_201_get_user_200() {
    let srv = helpers::spawn_app();
    let client = reqwest::Client::new();

    let email = "foo@bar.com";
    let name = "Jeff Jefferson";

    let created_usr: User = {
        let endpoint = format!("{}/api/v1/users", srv.basepath);

        let mut req = HashMap::new();
        req.insert("email", email);
        req.insert("name", name);

        let resp = client
            .post(endpoint)
            .json(&req)
            .send()
            .await
            .expect("failed to execute request");

        let status_code = resp.status();
        assert_eq!(http::StatusCode::CREATED, status_code);

        let text = resp
            .text()
            .await
            .expect("failed to get payload");
        serde_json::from_str(&text).expect("failed to parse json payload")
    };

    assert_eq!(name, created_usr.name().to_string());
    assert_eq!(email, created_usr.email().to_string());
    assert!(created_usr.id().to_string().len() == 36);

    let get_usr: User = {
        let endpoint = format!(
            "{}/api/v1/users/{}",
            srv.basepath,
            created_usr.id()
        );

        let resp = client
            .get(endpoint)
            .send()
            .await
            .expect("failed to execute request");

        let status_code = resp.status();
        assert_eq!(http::StatusCode::OK, status_code);

        let text = resp
            .text()
            .await
            .expect("failed to get payload");
        serde_json::from_str(&text).expect("failed to parse json payload")
    };

    assert_eq!(name, get_usr.name().to_string());
    assert_eq!(email, get_usr.email().to_string());
    assert_eq!(
        created_usr.id().to_string(),
        get_usr.id().to_string()
    );
}

#[tokio::test]
async fn get_user_404() {
    let srv = helpers::spawn_app();
    let client = reqwest::Client::new();

    let id = ID::new().to_string();
    let endpoint = format!("{}/api/v1/users/{}", srv.basepath, id);

    let resp = client
        .get(endpoint)
        .send()
        .await
        .expect("failed to execute request");

    let status_code = resp.status();
    assert_eq!(http::StatusCode::NOT_FOUND, status_code);

    let err = resp
        .json::<ServiceError>()
        .await
        .expect("failed to get payload");

    assert!(matches!(
        err.code(),
        ServiceErrorCode::UserNotFound
    ));
}

#[tokio::test]
async fn create_user_duplicate() {
    // todo
}

#[tokio::test]
async fn create_user_bad_data() {
    // todo
}
