mod routes;

use crate::{
    logic::{self},
    toolbox::{context, logger},
};
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    web, HttpMessage,
};
use actix_web_lab::middleware;
use std::{error::Error, net::TcpListener, sync::Arc};

pub fn create_listener(port: u16) -> Result<TcpListener, Box<dyn Error>> {
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(addr)?;
    Ok(listener)
}

pub fn init(
    listener: TcpListener,
    logic: Arc<logic::Logic>,
) -> Result<actix_web::dev::Server, Box<dyn Error>> {
    let app_init = move || {
        let logic = web::Data::from(Arc::clone(&logic));
        actix_web::App::new()
            // Attach logic controller
            .app_data(logic)
            // Custom request/response logging middleware
            .wrap(middleware::from_fn(custom_logger_mw))
            // Inject Context into request
            .wrap(middleware::from_fn(inject_context))
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

async fn inject_context(
    req: ServiceRequest,
    next: middleware::Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let tid = uuid::Uuid::new_v4().to_string(); // todo
    let ctx = context::Context::new();
    ctx.store("trace_id", tid);
    req.extensions_mut().insert(ctx);

    next.call(req).await
}

async fn custom_logger_mw(
    req: ServiceRequest,
    next: middleware::Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let ctx = ctx_from_req(req.request());

    // LOG REQUEST
    logger::ctx_info!(
        &ctx,
        "[HTTP] method: {} | url: {}",
        req.method().to_string(),
        req.uri().to_string(),
    );

    // NEXT
    let resp_wrap = next.call(req).await;

    // LOG RESPONSE
    if let Err(ref e) = resp_wrap {
        logger::ctx_error!(&ctx, "handler failed: {:?}", e);
        return resp_wrap;
    }

    let resp = resp_wrap.as_ref().unwrap();

    if let Some(err) = resp.response().error() {
        //if let Some(svc_err) = err.as_error::<logic::error::ServiceError>() {
        if resp.status().is_client_error() {
            logger::ctx_warning!(&ctx, "{:?}", err);
        } else {
            logger::ctx_error!(&ctx, "{:?}", err);
        }
        //}
    };

    resp_wrap
}

fn ctx_from_req(req: &actix_web::HttpRequest) -> Arc<context::Context> {
    let ext = req.extensions();
    let ctx = ext
        .get::<Arc<context::Context>>()
        .unwrap();
    Arc::clone(ctx)
}
