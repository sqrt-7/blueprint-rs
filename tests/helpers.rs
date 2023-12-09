use std::sync::{self, Arc};

use opentelemetry::sdk::{export::trace::stdout as otel_stdout, trace as otel_trace};

use blueprint::{
    datastore::inmem::InMemDatastore,
    logic::Controller,
    server::{self, http},
};

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

static LOG_INIT: sync::Once = sync::Once::new();

pub fn spawn_app() -> TestServer {
    LOG_INIT.call_once(|| {
        env_logger::builder()
            .parse_default_env()
            .default_format()
            .format_module_path(true)
            .format_target(true)
            .format_timestamp_millis()
            .filter_level(log::LevelFilter::Info)
            .target(env_logger::Target::Stdout)
            .init();
    });

    let tracer = otel_stdout::new_pipeline()
        .with_trace_config(otel_trace::config().with_sampler(otel_trace::Sampler::AlwaysOff))
        .install_simple();

    let listener = // random port
        server::create_listener(String::from("127.0.0.1:0")).unwrap_or_else(|err| {
            panic!("unable to bind listener: {}", err);
        });

    let actual_http_port = listener.local_addr().unwrap().port();

    let ds = Arc::new(InMemDatastore::new());
    let svc = Arc::new(Controller::new(ds));

    let http_server = http::init_server(listener, svc, tracer).unwrap_or_else(|err| {
        panic!("failed to start http server: {}", err);
    });

    let basepath = format!("http://127.0.0.1:{}", actual_http_port);

    tokio::spawn(http_server);

    TestServer::new(basepath)
}
