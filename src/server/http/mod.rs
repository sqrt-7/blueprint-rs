mod routes;

use crate::logic;
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    web,
};
use actix_web_lab::middleware::{from_fn, Next};

use std::{error::Error, net::TcpListener, sync::Arc};

pub fn create_listener(port: u16) -> Result<TcpListener, Box<dyn Error>> {
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(addr)?;
    Ok(listener)
}

pub fn init(
    listener: TcpListener,
    controller: Arc<logic::Controller>,
) -> Result<actix_web::dev::Server, Box<dyn Error>> {
    let holder = web::Data::from(controller);

    let app_init = move || {
        //let tracer = tracer.clone();
        actix_web::App::new()
            // Attach logic controller
            .app_data(holder.clone())
            // Custom request/response logging middleware
            .wrap(from_fn(custom_logger_mw))
            // .wrap_fn(move |req, srv| {
            //     tracer.in_span("trace_http", move |cx: Context| srv.call(req).with_context(cx))
            // })
            // Register endpoints
            .configure(routes::endpoints)
    };

    let server = actix_web::HttpServer::new(app_init)
        .shutdown_timeout(30)
        .listen(listener)?
        .run();

    // Does nothing until we call await
    Ok(server)
}

async fn custom_logger_mw(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    // LOG REQUEST
    // logging::new(logging::Level::Info, "http_request")
    //     .with_string("method", req.method().to_string())
    //     .with_string("url", req.uri().to_string())
    //     .with_string("headers", format!("{:?}", req.headers()))
    //     .send();

    // NEXT
    let resp_wrap = next.call(req).await;

    // LOG RESPONSE
    if let Err(ref e) = resp_wrap {
        // logging::new(logging::Level::Error, "actix_web handler failed")
        //     .with_string("error", e.to_string())
        //     .send();

        println!("actix_web handler failed: {}", e);
        return resp_wrap;
    }

    // let resp = resp_wrap.as_ref().unwrap();

    // let mut builder = logging::new(logging::Level::Info, "http_response")
    //     .with_string("status", resp.status().to_string());

    // if let Some(err) = resp.response().error() {
    //     if let Some(svc_err) = err.as_error::<logic::error::ServiceError>() {
    //         builder = builder
    //             .with_string("error_code", svc_err.code().to_string())
    //             .with_string("error_type", svc_err.error_type().to_string())
    //             .with_string("error_internal", format!("{:?}", svc_err.internal_msg()));
    //     }
    // };

    // if resp.status().is_client_error() {
    //     builder = builder.set_level(logging::Level::Warn);
    // }

    // if resp.status().is_server_error() {
    //     builder = builder.set_level(logging::Level::Error);
    // }

    // builder.send();

    resp_wrap
}
