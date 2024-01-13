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

pub mod context {
    use std::collections::HashMap;

    pub struct Context {
        store: HashMap<String, Box<dyn std::any::Any>>,
    }

    impl Context {
        pub fn new() -> Self {
            Self {
                store: HashMap::new(),
            }
        }

        pub fn store<T: 'static>(&mut self, key: &str, item: T) {
            self.store
                .insert(key.to_string(), Box::new(item));
        }

        pub fn get<T: 'static>(&self, key: &str) -> Option<&T> {
            self.store
                .get(&key.to_string())
                .and_then(|boxed| boxed.downcast_ref::<T>())
        }
    }
}

pub mod blueprint_logger {
    use crate::context::Context;

    pub struct Logger {
        format: LogFormat,
    }

    impl Logger {
        pub fn new(format: LogFormat) -> Self {
            Self {
                format,
            }
        }

        pub fn with(&self, ctx: &Context) -> LogEntry {
            let trace_id = match ctx.get::<String>("trace_id") {
                Some(s) => s.clone(),
                None => String::new(),
            };

            LogEntry {
                format: self.format,
                trace_id,
                level: Level::Trace,
                path: module_path!().to_string(),
                line: format!("{}:{}", file!(), line!()),
                msg: String::new(),
            }
        }
    }

    impl Default for Logger {
        fn default() -> Self {
            Logger::new(LogFormat::Json)
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum LogFormat {
        Json,
    }

    #[derive(serde::Serialize, Debug)]
    enum Level {
        #[serde(rename = "trace")]
        Trace,
        #[serde(rename = "debug")]
        Debug,
        #[serde(rename = "info")]
        Info,
        #[serde(rename = "warn")]
        Warn,
        #[serde(rename = "error")]
        Error,
    }

    #[derive(serde::Serialize, Debug)]
    pub struct LogEntry {
        #[serde(skip)]
        format: LogFormat,

        level: Level,
        trace_id: String,
        path: String,
        line: String,
        msg: String,
    }

    impl LogEntry {
        pub fn info(mut self, msg: &str) {
            self.level = Level::Info;
            self.msg = msg.to_string();
            self.publish();
        }

        fn publish(self) {
            match self.format {
                LogFormat::Json => {
                    let js = serde_json::to_string(&self)
                        .unwrap_or_else(|err| format!("log entry failed: {:?}", err));
                    println!("{}", js);
                },
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::context::Context;

        #[test]
        fn log_with_context() {
            let mut ctx = Context::new();
            ctx.store("trace_id", "12345".to_string());

            let logger = Logger::new(LogFormat::Json);
            logger.with(&ctx).info("hello world");
        }
    }

    // impl From<&log::Record<'_>> for LogEntry {
    //     fn from(record: &log::Record) -> Self {
    //         let file_line: String = format!(
    //             "{}:{}",
    //             record.file().unwrap_or_default(),
    //             record.line().unwrap_or_default()
    //         );

    //         LogEntry {
    //             trace_id: String::new(),
    //             level: record.level().into(),
    //             path: record
    //                 .module_path()
    //                 .unwrap_or_default()
    //                 .to_string(),
    //             line: file_line,
    //             msg: format!("{}", record.args()),
    //         }
    //     }
    // }

    // impl log::Log for Logger {
    //     fn enabled(&self, _: &log::Metadata) -> bool {
    //         true
    //     }

    //     fn log(&self, record: &log::Record) {
    //         let mut entry: LogEntry = record.into();
    //         entry.trace_id = todo!();
    //         entry.publish(self.format)
    //     }

    //     fn flush(&self) {}
    // }

    // impl From<log::Level> for Level {
    //     fn from(value: log::Level) -> Self {
    //         match value {
    //             log::Level::Error => Level::Error,
    //             log::Level::Warn => Level::Warn,
    //             log::Level::Info => Level::Info,
    //             log::Level::Debug => Level::Debug,
    //             log::Level::Trace => Level::Trace,
    //         }
    //     }
    // }
}
