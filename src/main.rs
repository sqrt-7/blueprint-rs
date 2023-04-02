use std::sync::Arc;

use opentelemetry::sdk::export::trace::stdout as otel_stdout;
use opentelemetry::sdk::trace as otel_trace;
use zero2prod::{
    datastore::inmem::InMemDatastore,
    logic::Service,
    server::{create_listener, http},
};

fn main() -> std::io::Result<()> {
    // CONFIG
    let config = Config::new_from_file("config.yaml")
        .unwrap_or_else(|err| panic!("failed to load config: {}", err));

    // TRACING
    let otel_tracer = otel_stdout::new_pipeline()
        .with_trace_config(otel_trace::config().with_sampler(otel_trace::Sampler::AlwaysOff))
        .install_simple();

    // LOGGING
    env_logger::builder()
        .parse_default_env()
        .default_format()
        .format_module_path(true)
        .format_target(true)
        .format_timestamp_millis()
        .filter_level(log::LevelFilter::Info)
        .target(env_logger::Target::Stdout)
        .init();

    // DB
    let datastore = Arc::new(InMemDatastore::new());

    // LOGIC
    let svc = Arc::new(Service::new(datastore));

    // HTTP SERVER
    let http_address = format!("127.0.0.1:{}", config.http_port);
    let http_listener = create_listener(http_address)
        .unwrap_or_else(|err| panic!("unable to bind http_listener: {}", err));
    let http_server = http::init_server(http_listener, svc, otel_tracer)
        .unwrap_or_else(|err| panic!("failed to start http server: {}", err));

    // RUNTIME
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap_or_else(|err| panic!("failed to build tokio runtime: {}", err));

    let main_thread = async { http_server.await };

    runtime.block_on(main_thread)
}

// CONFIG ---------------------------

#[derive(serde::Deserialize, Debug)]
pub struct Config {
    pub http_port: u16,
}

impl Config {
    pub fn new(http_port: u16) -> Self {
        Config { http_port }
    }

    pub fn new_from_file(filepath: &str) -> Result<Self, config::ConfigError> {
        let loader = config::Config::builder()
            .add_source(config::File::new(filepath, config::FileFormat::Yaml))
            .build()?;

        loader.try_deserialize::<Config>()
    }
}
