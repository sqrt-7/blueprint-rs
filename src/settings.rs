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
