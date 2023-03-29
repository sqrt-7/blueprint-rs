use tracing::Level;
use zero2prod::{
    datastore::inmem::InMemDatastore,
    logic::Service,
    server::{create_listener, start_http_server},
};

fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(Level::INFO)
        .init();

    let config = Config::new_from_file("config.yaml")
        .unwrap_or_else(|err| panic!("failed to load settings: {}", err));

    let http_address = format!("127.0.0.1:{}", config.http_port);

    let datastore = InMemDatastore::new();

    let svc = Service::new_arc(datastore);

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
