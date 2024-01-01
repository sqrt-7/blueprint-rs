use crate::logic::{
    dto,
    error::{ServiceError, ServiceErrorCode, ServiceErrorType},
    Controller,
};
use actix_web::{
    http::Method,
    web::{self, ServiceConfig},
    HttpRequest, HttpResponse, Resource, Responder, Route, Scope,
};

pub type HttpResult = std::result::Result<HttpResponse, ServiceError>;

pub(super) fn endpoints(cfg: &mut ServiceConfig) {
    cfg.service(
        Resource::new("/healthz").route(
            Route::new()
                .method(Method::GET)
                .to(healthz),
        ),
    );

    cfg.service(
        Scope::new("/api/v1")
            .service(
                Resource::new("/users").route(
                    Route::new()
                        .method(Method::POST)
                        .to(post_user),
                ),
            )
            .service(
                Resource::new("/users/{id}").route(
                    Route::new()
                        .method(Method::GET)
                        .to(get_user),
                ),
            )
            .service(
                Resource::new("/subscriptions").route(
                    Route::new()
                        .method(Method::POST)
                        .to(post_subscription),
                ),
            )
            .service(
                Resource::new("/subscriptions/user/{user_id}").route(
                    Route::new()
                        .method(Method::GET)
                        .to(list_subscriptions_by_user),
                ),
            ),
    );
}
#[derive(serde::Serialize, Debug)]
pub struct HealthzResponse {
    strong_count: usize,
    weak_count: usize,
}

pub(super) async fn healthz() -> impl Responder {
    HttpResponse::Ok()
}

//#[tracing::instrument(skip(svc))]
pub(super) async fn post_user(svc: web::Data<Controller>, body: web::Bytes) -> HttpResult {
    let data = serde_json::from_slice::<dto::CreateUserRequest>(&body);

    if let Err(json_err) = data {
        return Err(
            ServiceError::new(ServiceErrorCode::UserInvalidData)
                .with_type(ServiceErrorType::InvalidArgument)
                .wrap(Box::new(json_err)),
        );
    }

    let data = data.unwrap();
    let svc = svc.get_ref();
    let result = svc.create_user(data)?;

    Ok(HttpResponse::Created().json(result))
}

//#[tracing::instrument(skip(svc))]
pub(super) async fn get_user(svc: web::Data<Controller>, req: HttpRequest) -> HttpResult {
    let id = req.match_info().get("id").unwrap();

    let svc = svc.get_ref();
    let result = svc.get_user(id)?;

    Ok(HttpResponse::Ok().json(result))
}

//#[tracing::instrument(skip(svc))]
pub(super) async fn list_subscriptions_by_user(
    svc: web::Data<Controller>,
    req: HttpRequest,
) -> HttpResult {
    let user_id = req.match_info().get("user_id").unwrap();

    let svc = svc.get_ref();
    let result = svc.list_subscriptions_by_user(user_id)?;

    Ok(HttpResponse::Ok().json(result))
}

//#[tracing::instrument(skip(svc))]
pub(super) async fn post_subscription(svc: web::Data<Controller>, body: web::Bytes) -> HttpResult {
    let data = serde_json::from_slice::<dto::CreateSubscriptionRequest>(&body);

    if let Err(json_err) = data {
        return Err(
            ServiceError::new(ServiceErrorCode::SubscriptionInvalidData)
                .with_type(ServiceErrorType::InvalidArgument)
                .wrap(Box::new(json_err)),
        );
    }

    let data = data.unwrap();
    let svc = svc.get_ref();
    let result = svc.create_subscription(data)?;

    Ok(HttpResponse::Created().json(result))
}
