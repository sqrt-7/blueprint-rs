use std::sync;

use env_logger::Env;
use zero2prod::{datastore::inmem::InMemDatastore, http_server, service::Service};

pub struct TestServer {
    pub basepath: String,
}

impl TestServer {
    fn new(basepath: String) -> Self {
        TestServer { basepath }
    }
}

static LOG_INIT: sync::Once = sync::Once::new();

pub fn spawn_app() -> TestServer {
    LOG_INIT.call_once(|| {
        let _ = env_logger::Builder::from_env(Env::default().default_filter_or("info")).try_init();
    });

    let listener = // random port
        http_server::create_listener(String::from("127.0.0.1:0")).unwrap_or_else(|err| {
            panic!("unable to bind listener: {}", err);
        });

    let actual_http_port = listener.local_addr().unwrap().port();

    let ds = InMemDatastore::new();
    let svc = Service::new_arc(ds);

    let http_server = http_server::start_http_server(listener, svc).unwrap_or_else(|err| {
        panic!("failed to start http server: {}", err);
    });

    let basepath = format!("http://127.0.0.1:{}", actual_http_port);

    tokio::spawn(http_server);

    TestServer::new(basepath)
}
