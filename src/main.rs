use blueprint::{
    datastore::inmem::InMemDatastore,
    logic::Controller,
    server::{grpc, http},
    Config,
};
use std::sync::Arc;
//use tracing_subscriber::prelude::*;

fn main() {
    // CONFIG
    let config = Config::new_from_file("config.yaml")
        .unwrap_or_else(|err| panic!("failed to load config: {}", err));

    println!("{:?}", config);
    run(config)
}

fn run(config: Config) {
    // TRACING
    //init_tracing();

    // DB
    let datastore = Arc::new(InMemDatastore::new());

    // CONTROLLER
    let ctrl = Arc::new(Controller::new(datastore));

    // HTTP SERVER
    let http_listener = http::create_listener(config.http_port)
        .unwrap_or_else(|err| panic!("failed to init http listener: {}", err));
    let http_server = http::init(http_listener, ctrl.clone())
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
        // let http_main = runtime.spawn(http_server);
        // println!("starting http server on port {}", config.http_port);

        // let grpc_main = runtime.spawn(grpc_server);
        // println!("starting grpc server on port {}", config.grpc_port);

        if let Err(e) = tokio::try_join!(
            runtime.spawn(http_server),
            runtime.spawn(grpc_server)
        ) {
            println!("main thread error: {}", e);
        }
    });

    cleanup();
}

fn cleanup() {
    println!("cleaning up");
    // todo
}

// fn init_tracing() {
//     // OpenTelemetry pipeline (turned off for now)
//     // otel_stdout::new_pipeline()
//     //     .with_trace_config(otel_trace::config().with_sampler(otel_trace::Sampler::AlwaysOff))
//     //     .install_simple()

//     // let my_subscriber = blueprint::tracing_json::JsonLogSubscriber::new();
//     // tracing::subscriber::set_global_default(my_subscriber).expect("setting tracing default failed");

//     // tracing_subscriber::registry()
//     //     .with(blueprint::tracing_json::JsonLogSubscriber::new())
//     //     .init();
// }
