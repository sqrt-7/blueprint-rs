use zero2prod::{
    datastore::inmem::InMemDatastore, http_server, service::Service, settings::Settings,
};

pub struct TestServer {
    pub basepath: String,
}

impl TestServer {
    fn new(basepath: String) -> Self {
        TestServer { basepath }
    }
}

pub fn spawn_app() -> TestServer {
    let settings = Settings::new(0);

    let listener =
        http_server::create_listener(settings.get_http_addr().as_str()).unwrap_or_else(|err| {
            panic!("unable to bind listener: {}", err);
        });
    let actual_http_port = listener.local_addr().unwrap().port();

    let ds = InMemDatastore::new();
    let svc = Service::new(settings, ds);

    let http_server = http_server::start_http_server(listener, svc).unwrap_or_else(|err| {
        panic!("Failed to start http server: {}", err);
    });

    let basepath = format!("http://127.0.0.1:{}", actual_http_port);

    tokio::spawn(http_server);

    TestServer::new(basepath)
}
