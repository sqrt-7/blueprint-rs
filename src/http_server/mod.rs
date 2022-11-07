pub mod routes;

use crate::service::Service;
use actix_web::{dev::Server, web, App, HttpServer};
use std::{error::Error, net::TcpListener, sync::Arc};

pub fn create_listener(addr: String) -> Result<TcpListener, Box<dyn Error>> {
    let listener = TcpListener::bind(addr)?;
    Ok(listener)
}

pub fn start_http_server(
    listener: TcpListener,
    svc: Arc<Service>,
) -> Result<Server, Box<dyn Error>> {
    let ss = web::Data::new(svc);

    // Register endpoints
    let app_init = move || {
        let mut app = App::new();

        for (path, route) in routes::endpoints() {
            app = app.route(path.as_str(), route);
        }

        app = app.app_data(ss.clone());

        app
    };

    let listen = HttpServer::new(app_init).listen(listener)?;
    let server = listen.run();

    Ok(server)
}
