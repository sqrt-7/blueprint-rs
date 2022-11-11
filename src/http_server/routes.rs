use crate::service::{
    error::{ServiceError, ServiceErrorType},
    Service,
};
use actix_web::{http::Method, web, HttpRequest, HttpResponse, Responder, Route};
use uuid::Uuid;

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

pub(super) async fn get_subscription(
    svc: web::Data<Service>,
    req: HttpRequest,
) -> Result<HttpResponse, ServiceError> {
    let uuid = req.match_info().get("uuid").unwrap().to_string();

    let request_id = Uuid::new_v4().to_string();
    let request_span = tracing::info_span!("[routes::get_subscription]",
        request_id = %request_id,
        subscriber_uuid = uuid,
    );

    let _ = request_span.enter();

    let svc = svc.get_ref();
    let result = svc.get_subscription(uuid)?;

    Ok(HttpResponse::Ok().json(result))
}

#[derive(serde::Deserialize)]
pub(super) struct PostSubscriptionRequest {
    email: String,
    name: String,
}

pub(super) async fn post_subscription(
    svc: web::Data<Service>,
    body: web::Bytes,
) -> Result<HttpResponse, ServiceError> {
    tracing::info!("[routes::post_subscription]");

    let data = serde_json::from_slice::<PostSubscriptionRequest>(&body);

    if let Err(json_err) = data {
        return Err(ServiceError::new(
            format!("failed to parse json request: {}", json_err).as_str(),
        )
        .with_type(ServiceErrorType::Validation));
    }

    let data = data.unwrap();

    let svc = svc.get_ref();

    let result = svc.create_subscription(data.email, data.name)?;

    Ok(HttpResponse::Created().json(result))
}
