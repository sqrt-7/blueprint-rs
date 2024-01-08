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
        actix_web::App::new()
            // Attach logic controller
            .app_data(holder.clone())
            // Custom request/response logging middleware
            .wrap(from_fn(custom_logger_mw))
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
    log::info!(
        "[http_request] method: {} | url: {}",
        req.method().to_string(),
        req.uri().to_string(),
    );

    // NEXT
    let resp_wrap = next.call(req).await;

    // LOG RESPONSE
    if let Err(ref e) = resp_wrap {
        log::error!("actix_web handler failed: {:?}", e);
        return resp_wrap;
    }

    let resp = resp_wrap.as_ref().unwrap();

    if let Some(err) = resp.response().error() {
        //if let Some(svc_err) = err.as_error::<logic::error::ServiceError>() {
        if resp.status().is_client_error() {
            log::warn!("{:?}", err);
        } else {
            log::error!("{:?}", err);
        }
        //}
    };

    resp_wrap
}
