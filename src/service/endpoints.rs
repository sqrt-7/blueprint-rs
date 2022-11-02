use actix_web::{http::Method, web, HttpRequest, HttpResponse, Responder, Route};

pub const ROUTES_HEALTHCHECK: &'static str = "/healthz";
pub const ROUTES_SUBSCRIPTIONS: &'static str = "/subscriptions";

pub(super) fn endpoints() -> Vec<(String, Route)> {
    vec![
        (
            String::from(ROUTES_HEALTHCHECK),
            Route::new().method(Method::GET).to(health_check),
        ),
        (
            String::from(ROUTES_SUBSCRIPTIONS),
            web::post().to(subscribe),
        ),
    ]
}

async fn health_check(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok()
}
