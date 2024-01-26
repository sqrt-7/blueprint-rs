use std::sync::Arc;

use crate::{
    logic::{
        dto,
        error::{ServiceError, ServiceErrorCode, ServiceErrorType},
        Logic,
    },
    toolbox::{context::Context, logger},
};
use actix_web::{
    http::Method,
    web::{self, ServiceConfig},
    HttpMessage, HttpRequest, HttpResponse, Resource, Responder, Route, Scope,
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

fn ctx_from_req(req: &HttpRequest) -> Arc<Context> {
    let ext = req.extensions();
    let ctx = ext.get::<Arc<Context>>().unwrap();
    Arc::clone(&ctx)
}

pub(super) async fn healthz() -> impl Responder {
    HttpResponse::Ok()
}

pub(super) async fn post_user(
    logic: web::Data<Logic>,
    req: HttpRequest,
    body: web::Bytes,
) -> HttpResult {
    let ctx = ctx_from_req(&req);
    let data = serde_json::from_slice::<dto::CreateUserRequest>(&body);

    if let Err(json_err) = data {
        return Err(
            ServiceError::new(ServiceErrorCode::UserInvalidData)
                .with_type(ServiceErrorType::InvalidArgument)
                .wrap(json_err),
        );
    }

    logger::ctx_info!(ctx, "hello");

    let data = data.unwrap();
    let result = logic.create_user(Arc::clone(&ctx), data)?;

    Ok(HttpResponse::Created().json(result))
}

pub(super) async fn get_user(logic: web::Data<Logic>, req: HttpRequest) -> HttpResult {
    let ctx = ctx_from_req(&req);
    let id = req.match_info().get("id").unwrap();
    let result = logic.get_user(Arc::clone(&ctx), id)?;

    Ok(HttpResponse::Ok().json(result))
}

pub(super) async fn list_subscriptions_by_user(
    logic: web::Data<Logic>,
    req: HttpRequest,
) -> HttpResult {
    let ctx = ctx_from_req(&req);
    let user_id = req.match_info().get("user_id").unwrap();
    let result = logic.list_subscriptions_by_user(Arc::clone(&ctx), user_id)?;

    Ok(HttpResponse::Ok().json(result))
}

pub(super) async fn post_subscription(
    logic: web::Data<Logic>,
    req: HttpRequest,
    body: web::Bytes,
) -> HttpResult {
    let ctx = ctx_from_req(&req);
    let data = serde_json::from_slice::<dto::CreateSubscriptionRequest>(&body);

    if let Err(json_err) = data {
        return Err(
            ServiceError::new(ServiceErrorCode::SubscriptionInvalidData)
                .with_type(ServiceErrorType::InvalidArgument)
                .wrap(json_err),
        );
    }

    let data = data.unwrap();
    let result = logic.create_subscription(Arc::clone(&ctx), data)?;

    Ok(HttpResponse::Created().json(result))
}
