#[rustfmt::skip]
mod helpers;

use actix_web::http;
use blueprint::logic::error::{ServiceError, ServiceErrorType, CODE_INVALID_UUID};
use std::collections::HashMap;

#[tokio::test]
async fn post_subscription_400_invalid_uuid() {
    let srv = helpers::spawn_app();
    let client = reqwest::Client::new();

    let endpoint = format!("{}/subscriptions", srv.basepath);

    let user_id = "blah";
    let journal_id = "blah2";

    let mut req = HashMap::new();
    req.insert("user_id", user_id);
    req.insert("journal_id", journal_id);

    let resp = client
        .post(endpoint)
        .json(&req)
        .send()
        .await
        .expect("failed to execute request");

    let status_code = resp.status();
    assert_eq!(http::StatusCode::BAD_REQUEST, status_code);

    let err = resp
        .json::<ServiceError>()
        .await
        .expect("failed to get payload");

    assert_eq!(http::StatusCode::BAD_REQUEST, status_code);
    assert_eq!(err.code(), CODE_INVALID_UUID);
    assert!(matches!(
        err.error_type(),
        ServiceErrorType::InvalidArgument
    ));
}

#[tokio::test]
async fn create_sub_201_get_sub_200() {
    // todo: create user
    // todo: create journal
    // todo: create sub
}
