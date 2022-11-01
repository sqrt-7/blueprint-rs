pub mod values;

use actix_web::{
    dev::Server, http::Method, web, App, HttpRequest, HttpResponse, HttpServer,
    Responder, Route,
};
use std::{io, net::TcpListener};

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

pub fn run(listener: TcpListener) -> Result<Server, io::Error> {
    let app_init = || {
        App::new()
            .route(
                values::ROUTES_HEALTHCHECK,
                Route::new().method(Method::GET).to(health_check),
            )
            .route(values::ROUTES_SUBSCRIPTIONS, web::post().to(subscribe))
    };

    let server = HttpServer::new(app_init).listen(listener)?.run();

    Ok(server)
}
