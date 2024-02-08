#[rustfmt::skip]
use std::sync::Arc;

use blueprint::{datastore::inmem::InMemDatastore, logic::Logic, server::http};

pub struct TestServer {
    pub basepath: String,
}

impl TestServer {
    fn new(basepath: String) -> Self {
        TestServer {
            basepath,
        }
    }
}

pub fn spawn_app() -> TestServer {
    // random port
    let listener = http::create_listener(0).unwrap_or_else(|err| {
        panic!("unable to bind http listener: {}", err);
    });

    let actual_http_port = listener.local_addr().unwrap().port();

    let ds = Box::new(InMemDatastore::new());
    let svc = Arc::new(Logic::new(ds));

    let http_server = http::init(listener, svc).unwrap_or_else(|err| {
        panic!("failed to start http server: {}", err);
    });

    let basepath = format!("http://127.0.0.1:{}", actual_http_port);

    tokio::spawn(http_server);

    TestServer::new(basepath)
}
