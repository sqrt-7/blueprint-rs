pub mod endpoints;

use crate::datastore::Datastore;
use actix_web::{dev::Server, App, HttpServer};
use std::{error::Error, net::TcpListener};

pub struct Service<DS>
where
    DS: Datastore,
{
    datastore: DS,
    http_port: Option<String>,
}

impl<DS> Service<DS>
where
    DS: Datastore,
{
    // New -------------------------

    pub fn new(datastore: DS) -> Self {
        Service {
            datastore,
            http_port: None,
        }
    }

    pub fn start_http_server(&mut self, addr: &str) -> Result<Server, Box<dyn Error>> {
        let listener = TcpListener::bind(addr)?;
        let port = listener.local_addr()?.port();
        self.http_port = Some(port.to_string());

        // Register endpoints
        let app_init = || {
            let mut app = App::new();

            for (path, route) in endpoints::endpoints() {
                app = app.route(path.as_str(), route);
            }
            app
        };

        let server = HttpServer::new(app_init).listen(listener)?.run();

        Ok(server)
    }

    // Get & Set -------------------

    pub fn http_port(&self) -> Option<&String> {
        self.http_port.as_ref()
    }
}
