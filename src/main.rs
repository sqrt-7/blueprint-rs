use blueprint::{
    datastore::{inmem::InMemDatastore, sql::SqlDatastore, Datastore},
    logic::Logic,
    server::{grpc, http},
    toolbox::logger,
    Config, ConfigDbType,
};
use std::{sync::Arc, time::Duration};
use tokio::runtime::Runtime;

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
    // RUNTIME
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap_or_else(|err| panic!("failed to build tokio runtime: {}", err));

    // DB
    let datastore = init_db(config.datastore, &runtime);

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

    runtime.block_on(async {
        if let Err(e) = tokio::try_join!(
            runtime.spawn(http_server),
            runtime.spawn(grpc_server)
        ) {
            println!("main thread error: {}", e);
        }
    });

    // Cleanup
    logger::logger()
        .log_entry(logger::Level::Info, "cleaning up...".to_string())
        .publish();

    runtime.shutdown_timeout(Duration::from_secs(5));
}

fn init_db(config: ConfigDbType, runtime: &Runtime) -> Box<dyn Datastore + Send + Sync> {
    match config {
        blueprint::ConfigDbType::InMem => Box::new(InMemDatastore::new()),
        blueprint::ConfigDbType::MySql {
            addr,
            port,
            user,
            password,
        } => {
            let res =
                runtime.block_on(async { SqlDatastore::new(&addr, port, &user, &password).await });

            if let Err(e) = res {
                panic!("failed to connect to db: {}", e);
            }

            logger::logger()
                .log_entry(logger::Level::Info, "MYSQL_CONNECTED".to_string())
                .publish();
            Box::new(res.unwrap())
        },
    }
}
