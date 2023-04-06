mod routes;

use crate::logic;
use actix_service::Service as _;
use actix_web::{
    dev::ServiceResponse,
    dev::{Server, ServiceRequest},
    web, App, HttpServer,
};
use actix_web_lab::middleware::{from_fn, Next};
use log::Level;
use opentelemetry::{
    trace::{FutureExt, Tracer},
    Key,
};
use opentelemetry::{Context, KeyValue};
use std::{error::Error, net::TcpListener, sync::Arc};

pub fn init_server(
    listener: TcpListener,
    svc: Arc<logic::Service>,
    otel_tracer: opentelemetry::sdk::trace::Tracer,
) -> Result<Server, Box<dyn Error>> {
    let svc_holder = web::Data::from(svc);

    let app_init = move || {
        let otel_tracer = otel_tracer.clone();

        App::new()
            // Attach logic::Service
            .app_data(svc_holder.clone())
            // Custom request/response logging middleware
            .wrap(from_fn(custom_logger_mw))
            // Telemetry
            .wrap_fn(move |req, srv| {
                otel_tracer.in_span("otel_http", move |cx: Context| srv.call(req).with_context(cx))
            })
            // Register endpoints
            .configure(routes::endpoints)
    };

    let server = HttpServer::new(app_init).listen(listener)?.run();

    Ok(server)
}

async fn custom_logger_mw(
    req: ServiceRequest,
    next: Next<impl actix_web::body::MessageBody>,
) -> Result<ServiceResponse<impl actix_web::body::MessageBody>, actix_web::Error> {
    // LOG REQUEST
    crate::custom_log(
        log::Level::Info,
        "custom_logger_mw",
        "HTTP_REQUEST",
        vec![
            Key::new("method").string(req.method().to_string()),
            Key::new("url").string(req.uri().to_string()),
            Key::new("headers").string(format!("{:?}", req.headers())),
        ],
    );

    // NEXT
    let resp_wrap = next.call(req).await;

    // LOG RESPONSE
    if let Err(ref e) = resp_wrap {
        crate::custom_log(
            log::Level::Error,
            "custom_logger_mw",
            "actix_web handler failed",
            vec![Key::new("error").string(e.to_string())],
        );
        return resp_wrap;
    }

    let resp = resp_wrap.as_ref().unwrap();
    let mut fields: Vec<KeyValue> = Vec::new();
    let mut level = Level::Info;

    fields.push(Key::new("status").string(resp.status().to_string()));

    if let Some(err) = resp.response().error() {
        if let Some(svc_err) = err.as_error::<logic::error::ServiceError>() {
            fields.push(Key::new("error_code").string(svc_err.code().to_owned()));
            fields.push(Key::new("error_type").string(svc_err.error_type().to_string()));
            fields.push(Key::new("error_internal").string(format!("{:?}", svc_err.internal_msg())));
        }
    };

    if resp.status().is_client_error() {
        level = Level::Warn;
    }

    if resp.status().is_server_error() {
        level = Level::Error;
    }

    crate::custom_log(level, "custom_logger_mw", "HTTP_RESPONSE", fields);

    resp_wrap
}
