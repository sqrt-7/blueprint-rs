mod helpers;

use actix_web::http;
use std::collections::HashMap;
use zero2prod::logic::{
    domain::Subscription,
    error::{ServiceError, ServiceErrorType, CODE_INVALID_UUID},
};

// #[tokio::test]
// async fn post_subscription_201_and_get_200() {
//     let srv = helpers::spawn_app();
//     let client = reqwest::Client::new();

//     let name = "Jeff Jeffries";
//     let email = "jeff@bob.com";

//     let mut req = HashMap::new();
//     req.insert("name", name);
//     req.insert("email", email);

//     // Create subscription
//     let created_sub: Subscription = {
//         let endpoint = format!("{}/subscriptions", srv.basepath);

//         let resp = client
//             .post(endpoint)
//             .json(&req)
//             .send()
//             .await
//             .expect("failed to execute request");

//         let status_code = resp.status();
//         assert_eq!(http::StatusCode::CREATED, status_code);

//         let text = resp.text().await.expect("failed to get payload");

//         serde_json::from_str(&text).expect("failed to parse json payload")
//     };

//     // Get subscription
//     let get_sub: Subscription = {
//         let endpoint = format!("{}/subscriptions/{}", srv.basepath, &created_sub.uuid());

//         let resp = client
//             .get(endpoint)
//             .send()
//             .await
//             .expect("failed to execute request");

//         let status_code = resp.status();
//         assert_eq!(http::StatusCode::OK, status_code);

//         resp.json::<Subscription>()
//             .await
//             .expect("failed to parse json payload")
//     };

//     assert_eq!(created_sub.uuid(), get_sub.uuid());
//     assert_eq!(email, get_sub.email());
//     assert_eq!(name, get_sub.name());
// }

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
    assert!(matches!(err.error_type(), ServiceErrorType::Validation));
}

// #[tokio::test]
// async fn get_subscription_404() {
//     let srv = helpers::spawn_app();
//     let client = reqwest::Client::new();

//     let endpoint = format!("{}/subscriptions/blah123", srv.basepath);

//     let resp = client
//         .get(endpoint)
//         .send()
//         .await
//         .expect("failed to execute request");

//     let status_code = resp.status();

//     let err = resp.json::<ServiceError>().await.expect("failed to get payload");

//     assert_eq!(http::StatusCode::NOT_FOUND, status_code);
//     assert!(matches!(err.error_type(), ServiceErrorType::NotFound));
// }
