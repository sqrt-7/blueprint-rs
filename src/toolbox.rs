pub mod context {
    use std::{collections::HashMap, sync::Mutex};

    pub struct Context {
        store: Mutex<HashMap<String, Box<dyn std::any::Any + Send + Sync>>>,
    }

    impl Context {
        pub fn new() -> Self {
            Self {
                store: Mutex::new(HashMap::new()),
            }
        }

        pub fn store<T: 'static + Send + Sync>(&self, key: &str, item: T) {
            self.store
                .lock()
                .unwrap()
                .insert(key.to_string(), Box::new(item));
        }

        pub fn get_clone<T: Clone + 'static>(&self, key: &str) -> Option<T> {
            self.store
                .lock()
                .unwrap()
                .get(&key.to_string())
                .and_then(|boxed_any| boxed_any.downcast_ref::<T>())
                .cloned()
        }

        pub fn pop<T: 'static>(&self, key: &str) -> Option<T> {
            self.store
                .lock()
                .unwrap()
                .remove(&key.to_string())
                .and_then(|removed| match removed.downcast::<T>() {
                    Ok(dc) => Some(*dc),
                    Err(_) => None,
                })
        }

        pub fn modify<T: 'static>(&self, key: &str, func: impl Fn(&mut T)) -> bool {
            let mut binding = self.store.lock().unwrap();

            let item = binding
                .get_mut(&key.to_string())
                .and_then(|boxed_mut| boxed_mut.downcast_mut::<T>());

            match item {
                Some(val) => {
                    func(val);
                    true
                },
                None => false,
            }
        }
    }

    impl Default for Context {
        fn default() -> Self {
            Context::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn store_and_get_clone() {
            let ctx = Context::new();
            let key: &str = "test-1";
            let val = String::from("12345");
            ctx.store(key, val.clone());
            let res = ctx.get_clone::<String>(key).unwrap();
            assert_eq!(res, val);
        }

        #[test]
        fn store_and_pop() {
            let ctx = Context::new();
            let key: &str = "test-1";
            let val = String::from("12345");
            ctx.store(key, val.clone());

            let res = ctx.pop::<String>(key).unwrap();
            assert_eq!(res, val);

            let res2 = ctx.pop::<String>(key);
            assert_eq!(res2, None);
        }

        #[test]
        fn store_and_modify() {
            let ctx = Context::new();
            ctx.store("test", String::from("12345"));

            {
                let res = ctx.modify("test", |v: &mut String| {
                    v.push_str("67890");
                });
                assert!(res == true);
            }

            {
                let res = ctx.pop::<String>("test").unwrap();
                assert_eq!(res, "1234567890");
            }

            {
                let res = ctx.modify("test", |v: &mut String| {
                    v.push_str("67890");
                });
                assert!(res == false);
            }
        }
    }
}

pub mod logger {
    static LOGGER: &Logger = &Logger {
        format: LogFormat::Json,
    };

    pub fn logger() -> &'static Logger {
        LOGGER
    }

    pub struct Logger {
        format: LogFormat,
    }

    impl Logger {
        pub fn new(format: LogFormat) -> Self {
            Self {
                format,
            }
        }

        pub fn log_entry(&self, level: Level, msg: String) -> LogEntry {
            LogEntry {
                format: self.format,
                trace_id: String::new(),
                level,
                path: String::new(),
                line: String::new(),
                thread: String::new(),
                msg,
            }
        }

        pub fn log_entry_filled(
            &self, level: Level, msg: String, trace_id: String, path: String, line: String,
            thread: String,
        ) -> LogEntry {
            LogEntry {
                format: self.format,
                level,
                trace_id,
                path,
                line,
                msg,
                thread,
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
    pub enum Level {
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
        #[serde(skip_serializing_if = "String::is_empty")]
        trace_id: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        path: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        line: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        thread: String,
        msg: String,
    }

    impl LogEntry {
        pub fn publish(self) {
            match self.format {
                LogFormat::Json => {
                    let js = serde_json::to_string(&self)
                        .unwrap_or_else(|err| format!("log entry failed: {:?}", err));
                    println!("{}", js);
                },
            }
        }
    }

    // macro_rules! log_msg {
    //     ($lvl:expr, $($arg:tt)+) => ({
    //         crate::toolbox::logger::logger()
    //             .log_entry_filled(
    //                 $lvl,
    //                 format!($($arg)+),
    //                 String::new(),
    //                 module_path!().to_string(),
    //                 format!("{}:{}", file!(), line!()))
    //             .publish();
    //     });
    // }
    // pub(crate) use log_msg;

    macro_rules! ctx_log {
        ($ctx:expr, $lvl:expr, $($arg:tt)+) => ({
            let tid: String = match $ctx.get_clone::<String>("trace_id") {
                Some(v) => v.clone(),
                None => String::new(),
            };
            let thread_name: String = match std::thread::current().name() {
                Some(v) => v.to_string(),
                None => String::new(),
            };

            crate::toolbox::logger::logger()
                .log_entry_filled(
                    $lvl,
                    format!($($arg)+),
                    tid,
                    module_path!().to_string(),
                    format!("{}:{}", file!(), line!()),
                    thread_name)
                .publish();
        });
    }
    pub(crate) use ctx_log;

    macro_rules! ctx_info {
        ($ctx:expr, $($arg:tt)+) => {
            crate::toolbox::logger::ctx_log!(
                $ctx, crate::toolbox::logger::Level::Info, $($arg)+);
        }
    }
    pub(crate) use ctx_info;

    macro_rules! ctx_warning {
        ($ctx:expr, $($arg:tt)+) => {
            crate::toolbox::logger::ctx_log!(
                $ctx, crate::toolbox::logger::Level::Warn, $($arg)+);
        }
    }
    pub(crate) use ctx_warning;

    macro_rules! ctx_error {
        ($ctx:expr, $($arg:tt)+) => {
            crate::toolbox::logger::ctx_log!(
                $ctx, crate::toolbox::logger::Level::Error, $($arg)+);
        }
    }
    pub(crate) use ctx_error;

    #[cfg(test)]
    mod tests {}
}
