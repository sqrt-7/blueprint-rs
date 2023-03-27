pub mod routes;

use crate::service::Service;
use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};
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
        let mut app = App::new()
            .app_data(svc_holder.clone())
            .wrap(Logger::default());

        // Register endpoints
        for (path, route) in routes::endpoints() {
            app = app.route(path.as_str(), route);
        }

        app
    };

    let server = HttpServer::new(app_init).listen(listener)?.run();

    Ok(server)
}
