use blueprint::{
    datastore::inmem::InMemDatastore,
    logic::Controller,
    server::{grpc, http},
    Config,
};
use std::sync::Arc;

fn main() {
    // CONFIG
    let config = Config::new_from_file("config.yaml")
        .unwrap_or_else(|err| panic!("failed to load config: {}", err));

    println!("{:?}", config);
    run(config)
}

fn run(config: Config) {
    // TRACING
    init_tracing();

    // DB
    let datastore = Arc::new(InMemDatastore::new());

    // CONTROLLER
    let ctrl = Arc::new(Controller::new(datastore));

    // HTTP SERVER
    let http_listener = http::create_listener(config.http_port)
        .unwrap_or_else(|err| panic!("failed to init http listener: {}", err));
    let http_server = http::init(http_listener, Arc::clone(&ctrl))
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

fn init_tracing() {
    //tracing_subscriber::fmt().json().init();
    //tracing::dispatcher::set_global_default(dispatcher)
}
