use crate::{
    logic::{
        dto,
        error::{LogicError, LogicErrorCode},
        Logic,
    },
    toolbox::logger,
};
use actix_web::{
    http::Method,
    web::{self, ServiceConfig},
    HttpRequest, HttpResponse, Resource, Responder, Route, Scope,
};

pub type HttpResult = std::result::Result<HttpResponse, LogicError>;

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
                Resource::new("/users")
                    .route(
                        Route::new()
                            .method(Method::POST)
                            .to(post_user),
                    )
                    .route(
                        Route::new()
                            .method(Method::GET)
                            .to(list_users),
                    ),
            )
            .service(
                Resource::new("/users/{id}").route(
                    Route::new()
                        .method(Method::GET)
                        .to(get_user),
                ),
            ),
    );
}

pub(super) async fn healthz() -> impl Responder {
    HttpResponse::Ok()
}

pub(super) async fn post_user(
    logic: web::Data<Logic>, req: HttpRequest, body: web::Bytes,
) -> HttpResult {
    let ctx = super::ctx_from_req(&req);
    let data = serde_json::from_slice::<dto::CreateUserRequest>(&body);

    if let Err(json_err) = data {
        return Err(LogicError::new(LogicErrorCode::UserInvalidData).wrap(json_err));
    }

    logger::ctx_info!(ctx, "hello");

    let data = data.unwrap();
    let result = logic.create_user(&ctx, data).await?;

    Ok(HttpResponse::Created().json(result))
}

pub(super) async fn get_user(logic: web::Data<Logic>, req: HttpRequest) -> HttpResult {
    let ctx = super::ctx_from_req(&req);
    let id = req.match_info().get("id").unwrap();
    let result = logic.get_user(&ctx, id).await?;

    Ok(HttpResponse::Ok().json(result))
}

pub(super) async fn list_users(logic: web::Data<Logic>, req: HttpRequest) -> HttpResult {
    let ctx = super::ctx_from_req(&req);
    let query = dto::Query {};
    let result = logic.list_users(&ctx, query).await?;

    Ok(HttpResponse::Ok().json(result))
}
