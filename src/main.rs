use actix_web::{
    http::{Method, StatusCode},
    web, App, HttpRequest, HttpResponse, HttpResponseBuilder, HttpServer,
    Responder, Route,
};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    let greet = format!("Hello {}!", name);

    let resp = HttpResponse::new(StatusCode::OK).set_body(greet);

    resp
}

async fn health_check(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // web::get() = Route::new().method(Method::GET)
            .route("/", Route::new().method(Method::GET).to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/health-check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
