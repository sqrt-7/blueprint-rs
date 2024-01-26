use blueprint::{
    datastore::inmem::InMemDatastore,
    logic::Logic,
    server::{grpc, http},
    toolbox::logger,
    Config,
};
use std::sync::Arc;

fn main() {
    // CONFIG
    let config = Config::new_from_file("config.yaml")
        .unwrap_or_else(|err| panic!("failed to load config: {}", err));
    logger::logger()
        .log_entry(logger::Level::Debug, format!("{:?}", config))
        .publish();
    run(config)
}

fn run(config: Config) {
    // TRACING
    init_tracing();

    // DB
    let datastore = Arc::new(InMemDatastore::new());

    // LOGIC CONTROLLER
    let logic = Arc::new(Logic::new(datastore));

    // HTTP SERVER
    let http_listener = http::create_listener(config.http_port)
        .unwrap_or_else(|err| panic!("failed to init http listener: {}", err));
    let http_server = http::init(http_listener, Arc::clone(&logic))
        .unwrap_or_else(|err| panic!("failed to init http server: {}", err));

    // GRPC SERVER
    let grpc_server = grpc::init(config.grpc_port, logic)
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
    logger::logger()
        .log_entry(logger::Level::Info, format!("cleaning up..."))
        .publish();
    // todo
}

fn init_tracing() {
    //tracing_subscriber::fmt().json().init();
    //tracing::dispatcher::set_global_default(dispatcher)
}
