#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub http_port: u16,
}

impl Settings {
    pub fn new(http_port: u16) -> Self {
        Settings { http_port }
    }

    pub fn new_from_file(filepath: &str) -> Result<Self, config::ConfigError> {
        let loader = config::Config::builder()
            .add_source(config::File::new(filepath, config::FileFormat::Yaml))
            .build()?;

        loader.try_deserialize::<Settings>()
    }
}
