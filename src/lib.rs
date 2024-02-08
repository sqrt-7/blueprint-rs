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
    pub datastore: ConfigDbType,
}

#[derive(serde::Deserialize)]
#[serde(tag = "db_type", content = "config")]
pub enum ConfigDbType {
    #[serde(rename = "inmem")]
    InMem,
    #[serde(rename = "mysql")]
    MySql {
        addr: String,
        port: u16,
        user: String,
        password: String,
    },
}

impl std::fmt::Debug for ConfigDbType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InMem => f.write_str("InMem"),
            Self::MySql {
                addr,
                port,
                user,
                .. // hide password
            } => write!(f, "MySql({user}:xxx@{addr}:{port})"),
        }
    }
}

impl Config {
    pub fn new(http_port: u16, grpc_port: u16, datastore: ConfigDbType) -> Self {
        Config {
            http_port,
            grpc_port,
            datastore,
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
