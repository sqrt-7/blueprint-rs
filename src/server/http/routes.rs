use crate::logic::{
    error::{ServiceError, ServiceErrorType, CODE_SUB_INVALID_DATA},
    Service,
};
use actix_web::{
    http::Method,
    web::{self, ServiceConfig},
    HttpRequest, HttpResponse, Responder, Route,
};

pub(super) fn endpoints(cfg: &mut ServiceConfig) {
    cfg.route("/healthz", Route::new().method(Method::GET).to(healthz));

    cfg.route(
        "/subscriptions",
        Route::new().method(Method::POST).to(post_subscription),
    );

    cfg.route(
        "/subscriptions/{user_id}",
        Route::new()
            .method(Method::GET)
            .to(list_subscriptions_by_user),
    );
}

pub(super) async fn healthz(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

pub(super) async fn list_subscriptions_by_user(
    svc: web::Data<Service>,
    req: HttpRequest,
) -> Result<HttpResponse, ServiceError> {
    let user_id = req.match_info().get("user_id").unwrap();

    let svc = svc.get_ref();
    let result = svc.list_subscriptions_by_user(user_id)?;

    Ok(HttpResponse::Ok().json(result))
}

#[derive(serde::Deserialize)]
pub(super) struct PostSubscriptionRequest {
    user_id: String,
    journal_id: String,
}

pub(super) async fn post_subscription(
    svc: web::Data<Service>,
    body: web::Bytes,
) -> Result<HttpResponse, ServiceError> {
    let data = serde_json::from_slice::<PostSubscriptionRequest>(&body);

    if let Err(json_err) = data {
        return Err(ServiceError::new(CODE_SUB_INVALID_DATA)
            .with_type(ServiceErrorType::Validation)
            .with_internal(format!("request json parse: {}", json_err)));
    }

    let data = data.unwrap();
    let svc = svc.get_ref();
    let result = svc.create_subscription(data.user_id, data.journal_id)?;

    Ok(HttpResponse::Created().json(result))
}
