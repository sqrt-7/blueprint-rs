use zero2prod::{
    datastore::inmem::InMemDatastore,
    http_server::{create_listener, start_http_server},
    service::Service,
    settings::Settings,
};

fn main() -> std::io::Result<()> {
    let settings = Settings::new_from_file("settings.yaml")
        .unwrap_or_else(|err| panic!("failed to load settings: {}", err));

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap_or_else(|err| panic!("failed to build runtime: {}", err));

    runtime.block_on(start(settings))
}

async fn start(settings: Settings) -> Result<(), std::io::Error> {
    let http_addr = &settings.get_http_addr();

    let datastore = InMemDatastore::new();

    let svc = Service::new(settings, datastore);

    let http_listener = create_listener(http_addr)
        .unwrap_or_else(|err| panic!("unable to bind http_listener: {}", err));

    let http_server = start_http_server(http_listener, svc)
        .unwrap_or_else(|err| panic!("failed to start http server: {}", err));

    http_server.await
}
