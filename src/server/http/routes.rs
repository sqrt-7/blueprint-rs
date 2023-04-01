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
        "/subscriptions/{uuid}",
        Route::new().method(Method::GET).to(get_subscription),
    );
}

pub(super) async fn healthz(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

pub(super) async fn get_subscription(
    svc: web::Data<Service>,
    req: HttpRequest,
) -> Result<HttpResponse, ServiceError> {
    let uuid = req.match_info().get("uuid").unwrap();

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
    let data = serde_json::from_slice::<PostSubscriptionRequest>(&body);

    if let Err(json_err) = data {
        return Err(ServiceError::new(CODE_SUB_INVALID_DATA)
            .with_type(ServiceErrorType::Validation)
            .with_internal(format!("request json parse: {}", json_err)));
    }

    let data = data.unwrap();
    let svc = svc.get_ref();
    let result = svc.create_subscription(data.email, data.name)?;

    Ok(HttpResponse::Created().json(result))
}
