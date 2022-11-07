use std::sync::Arc;

use crate::service::{
    error::{ServiceError, ServiceErrorType},
    Service,
};
use actix_web::{http::Method, web, HttpRequest, HttpResponse, Responder, Route};

pub(super) fn endpoints() -> Vec<(String, Route)> {
    vec![
        (
            String::from("/healthz"),
            Route::new().method(Method::GET).to(healthz),
        ),
        (
            String::from("/subscriptions"),
            Route::new().method(Method::POST).to(post_subscription),
        ),
        (
            String::from("/subscriptions/{uuid}"),
            Route::new().method(Method::GET).to(get_subscription),
        ),
    ]
}

pub(super) async fn healthz(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

#[derive(serde::Deserialize)]
pub(super) struct PostSubscriptionRequest {
    email: String,
    name: String,
}

pub(super) async fn post_subscription(
    svc: web::Data<Arc<Service>>,
    body: web::Bytes,
) -> Result<HttpResponse, ServiceError> {
    let data = serde_json::from_slice::<PostSubscriptionRequest>(&body);

    if let Err(json_err) = data {
        return Err(ServiceError::new("failed to parse json request")
            .with_type(ServiceErrorType::Validation)
            .with_internal(json_err.to_string().as_str()));
    }

    let data = data.unwrap();

    let svc = svc.get_ref();

    let result = svc.create_subscription(data.email, data.name)?;

    Ok(HttpResponse::Created().json(result))
}

pub(super) async fn get_subscription(
    svc: web::Data<Arc<Service>>,
    req: HttpRequest,
) -> Result<HttpResponse, ServiceError> {
    let uuid = req.match_info().get("uuid").unwrap();

    let svc = svc.get_ref();
    let result = svc.get_subscription(uuid.to_string())?;

    Ok(HttpResponse::Ok().json(result))
}
