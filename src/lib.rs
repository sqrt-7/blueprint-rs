#![allow(dead_code)]
#![allow(clippy::wrong_self_convention)]

pub mod datastore;
pub mod logic;
pub mod server;

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

pub mod bplog {
    pub struct JsonLogger {}

    impl log::Log for JsonLogger {
        fn enabled(&self, _: &log::Metadata) -> bool {
            true
        }

        fn log(&self, record: &log::Record) {
            let entry: LogEntry = record.into();
            let js = serde_json::to_string(&entry)
                .unwrap_or_else(|err| format!("log entry failed: {:?}", err));
            println!("{}", js);
        }

        fn flush(&self) {}
    }

    #[derive(serde::Serialize, Debug)]
    enum Level {
        #[serde(rename = "debug")]
        Debug,
        #[serde(rename = "info")]
        Info,
        #[serde(rename = "warn")]
        Warn,
        #[serde(rename = "error")]
        Error,
    }

    impl From<log::Level> for Level {
        fn from(value: log::Level) -> Self {
            match value {
                log::Level::Error => Level::Error,
                log::Level::Warn => Level::Warn,
                log::Level::Info => Level::Info,
                log::Level::Debug => Level::Debug,
                log::Level::Trace => Level::Debug,
            }
        }
    }

    #[derive(serde::Serialize, Debug)]
    struct LogEntry {
        level: Level,
        path: String,
        line: String,
        msg: String,
    }

    impl From<&log::Record<'_>> for LogEntry {
        fn from(record: &log::Record) -> Self {
            let file_line: String = format!(
                "{}:{}",
                record.file().unwrap_or_default(),
                record.line().unwrap_or_default()
            );

            LogEntry {
                level: record.level().into(),
                path: record
                    .module_path()
                    .unwrap_or_default()
                    .to_string(),
                line: file_line,
                msg: format!("{}", record.args()),
            }
        }
    }
}
