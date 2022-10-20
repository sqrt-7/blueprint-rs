use std::io;

use actix_web::{
    dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};

async fn health_check(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(address: &str) -> Result<Server, io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            // // web::get() = Route::new().method(Method::GET)
            // .route("/", Route::new().method(Method::GET).to(greet))
            // .route("/{name}", web::get().to(greet))
            .route("/health-check", web::get().to(health_check))
    })
    .bind(address)?
    .run();

    Ok(server)
}
