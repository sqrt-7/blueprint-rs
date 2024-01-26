#![allow(dead_code)]
#![allow(clippy::wrong_self_convention)]

pub mod datastore;
pub mod logic;
pub mod server;
pub mod toolbox;

// Import proto generated files
pub mod proto {
    include!("../proto/blueprint.rs");
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
            .add_source(config::File::new(
                filepath,
                config::FileFormat::Yaml,
            ))
            .build()?;

        loader.try_deserialize::<Config>()
    }
}
