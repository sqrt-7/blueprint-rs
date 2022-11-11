use env_logger::Env;
use zero2prod::{
    datastore::inmem::InMemDatastore,
    http_server::{create_listener, start_http_server},
    service::Service,
    settings::Settings,
};

fn main() -> std::io::Result<()> {
    let settings = Settings::new_from_file("settings.yaml")
        .unwrap_or_else(|err| panic!("failed to load settings: {}", err));

    let http_address = format!("127.0.0.1:{}", settings.http_port);

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let datastore = InMemDatastore::new();

    let svc = Service::new_arc(settings, datastore);

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap_or_else(|err| panic!("failed to build tokio runtime: {}", err));

    let http_listener = create_listener(http_address)
        .unwrap_or_else(|err| panic!("unable to bind http_listener: {}", err));

    let http_server = start_http_server(http_listener, svc)
        .unwrap_or_else(|err| panic!("failed to start http server: {}", err));

    let main_thread = async { http_server.await };

    runtime.block_on(main_thread)
}
