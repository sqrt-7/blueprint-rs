use blueprint::{
    datastore::inmem::InMemDatastore,
    logic::Controller,
    server::{grpc, http},
};
use std::sync::Arc;
use tracing_subscriber::prelude::*;

fn main() {
    // CONFIG
    let config = Config::new_from_file("config.yaml")
        .unwrap_or_else(|err| panic!("failed to load config: {}", err));

    // TRACING
    init_tracing();

    // DB
    let datastore = Arc::new(InMemDatastore::new());

    // CONTROLLER
    let ctrl = Arc::new(Controller::new(datastore));

    // HTTP SERVER
    let http_server = http::init(config.http_port, ctrl.clone())
        .unwrap_or_else(|err| panic!("failed to init http server: {}", err));

    // GRPC SERVER
    let grpc_server = grpc::init(config.grpc_port, ctrl)
        .unwrap_or_else(|err| panic!("failed to init grpc server: {}", err));

    // RUNTIME
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap_or_else(|err| panic!("failed to build tokio runtime: {}", err));

    runtime.block_on(async {
        let http_main = runtime.spawn(async {
            tracing::info!("starting http server");
            http_server.await
        });
        let grpc_main = runtime.spawn(async {
            tracing::info!("starting grpc server");
            grpc_server.await
        });

        if let Err(e) = tokio::try_join!(http_main, grpc_main) {
            tracing::error!("main thread error: {}", e);
        }
    });

    cleanup();
}

fn cleanup() {
    tracing::info!("cleaning up");
    // todo
}

fn init_tracing() {
    // OpenTelemetry pipeline (turned off for now)
    // otel_stdout::new_pipeline()
    //     .with_trace_config(otel_trace::config().with_sampler(otel_trace::Sampler::AlwaysOff))
    //     .install_simple()

    // let my_subscriber = blueprint::tracing_json::JsonLogSubscriber::new();
    // tracing::subscriber::set_global_default(my_subscriber).expect("setting tracing default failed");

    tracing_subscriber::registry()
        .with(blueprint::tracing_json::JsonLogSubscriber::new())
        .init();
}

// CONFIG ---------------------------

#[derive(serde::Deserialize, Debug)]
pub struct Config {
    pub http_port: u16,
    pub grpc_port: u16,
}

impl Config {
    pub fn new(http_port: u16, grpc_port: u16) -> Self {
        Config {
            http_port,
            grpc_port,
        }
    }

    pub fn new_from_file(filepath: &str) -> Result<Self, config::ConfigError> {
        let loader = config::Config::builder()
            .add_source(config::File::new(filepath, config::FileFormat::Yaml))
            .build()?;

        loader.try_deserialize::<Config>()
    }
}
