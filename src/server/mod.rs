pub mod http_routes;

use crate::logic::{error::ServiceError, Service};
use actix_web::{
    dev::{Server, ServiceRequest},
    dev::{Service as _, ServiceResponse},
    web, App, HttpServer,
};
use std::{error::Error, net::TcpListener, sync::Arc};

pub fn create_listener(addr: String) -> Result<TcpListener, Box<dyn Error>> {
    let listener = TcpListener::bind(addr)?;
    Ok(listener)
}

pub fn start_http_server(
    listener: TcpListener,
    svc: Arc<Service>,
) -> Result<Server, Box<dyn Error>> {
    let svc_holder = web::Data::from(svc);

    let app_init = move || {
        let mut app = App::new().app_data(svc_holder.clone()).wrap_fn(|req, srv| {
            log_request(&req);
            let fut = srv.call(req);
            async {
                let res = fut.await?;
                log_response(&res);
                Ok(res)
            }
        });

        // Register endpoints
        for (path, route) in http_routes::endpoints() {
            app = app.route(path.as_str(), route);
        }

        app
    };

    let server = HttpServer::new(app_init).listen(listener)?.run();

    Ok(server)
}

fn log_request(req: &ServiceRequest) {
    tracing::info!(
        "[REQUEST] method = {} | url = {} | headers = {}",
        req.method(),
        req.uri().to_string(),
        format!("{:?}", req.headers())
    );
}

fn log_response(resp: &ServiceResponse) {
    let resp_msg: String;

    match resp.response().error() {
        Some(err) => {
            if let Some(svc_err) = err.as_error::<ServiceError>() {
                resp_msg = format!("[RESPONSE] {} {:?}", resp.status().as_u16(), svc_err);
            } else {
                resp_msg = format!("[RESPONSE] {} {}", resp.status().as_u16(), err);
            }
        }
        None => {
            resp_msg = format!("[RESPONSE] {}", resp.status());
        }
    };

    if resp.status().is_informational()
        || resp.status().is_success()
        || resp.status().is_redirection()
    {
        tracing::info!("{}", resp_msg);
    }

    if resp.status().is_client_error() {
        tracing::warn!("{}", resp_msg);
    }

    if resp.status().is_server_error() {
        tracing::error!("{}", resp_msg);
    }
}
