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

// pub mod tracing_json {

//     pub struct JsonLogSubscriber {}

//     struct JsonLogEntry {
//         level: tracing::Level,
//         span_id: String,
//         path: String,
//     }

//     impl JsonLogSubscriber {
//         pub fn new() -> Self {
//             JsonLogSubscriber {}
//         }
//     }

//     impl JsonLogSubscriber {
//         fn write_log() {}
//     }

//     impl<S> tracing_subscriber::Layer<S> for JsonLogSubscriber
//     where
//         S: tracing::Subscriber,
//     {
//         fn max_level_hint(&self) -> Option<tracing::metadata::LevelFilter> {
//             Some(tracing::metadata::LevelFilter::from_level(tracing::Level::INFO))
//         }

//         fn on_new_span(
//             &self,
//             attrs: &tracing::span::Attributes<'_>,
//             id: &tracing::span::Id,
//             ctx: tracing_subscriber::layer::Context<'_, S>,
//         ) {
//             let _ = (attrs, id, ctx);
//         }

//         fn on_record(
//             &self,
//             _span: &tracing::span::Id,
//             _values: &tracing::span::Record<'_>,
//             _ctx: tracing_subscriber::layer::Context<'_, S>,
//         ) {
//         }

//         fn on_follows_from(
//             &self,
//             _span: &tracing::span::Id,
//             _follows: &tracing::span::Id,
//             _ctx: tracing_subscriber::layer::Context<'_, S>,
//         ) {
//         }

//         fn on_event(
//             &self,
//             _event: &tracing::Event<'_>,
//             _ctx: tracing_subscriber::layer::Context<'_, S>,
//         ) {
//         }

//         fn on_enter(
//             &self,
//             _id: &tracing::span::Id,
//             _ctx: tracing_subscriber::layer::Context<'_, S>,
//         ) {
//         }

//         fn on_exit(
//             &self,
//             _id: &tracing::span::Id,
//             _ctx: tracing_subscriber::layer::Context<'_, S>,
//         ) {
//         }

//         fn on_close(
//             &self,
//             _id: tracing::span::Id,
//             _ctx: tracing_subscriber::layer::Context<'_, S>,
//         ) {
//         }

//         fn on_id_change(
//             &self,
//             _old: &tracing::span::Id,
//             _new: &tracing::span::Id,
//             _ctx: tracing_subscriber::layer::Context<'_, S>,
//         ) {
//         }
//     }

// impl tracing::Subscriber for JsonLogSubscriber {
//     // This is only called once per callsite
//     fn enabled(&self, _: &tracing::Metadata<'_>) -> bool {
//         true
//     }

//     // Determines the ID of a new span
//     fn new_span(&self, _: &tracing::span::Attributes<'_>) -> tracing::span::Id {
//         let mut rng = rand::thread_rng();
//         let mut num = 0u64;

//         while num == 0 {
//             num = rng.gen();
//         }

//         tracing::span::Id::from_u64(num)
//     }

//     fn record(&self, span: &tracing::span::Id, values: &tracing::span::Record<'_>) {
//         println!("[RECORD] {:?} | {:?}", span, values);
//     }

//     fn record_follows_from(&self, span: &tracing::span::Id, follows: &tracing::span::Id) {
//         println!("[RECORD_FOLLOWS] {:?} | {:?}", span, follows);
//     }

//     fn event(&self, event: &tracing::Event<'_>) {
//         println!("[EVENT] {:?}", event);
//     }

//     fn enter(&self, span: &tracing::span::Id) {
//         println!("[ENTER] {:?}", span);
//     }

//     fn exit(&self, span: &tracing::span::Id) {
//         println!("[EXIT] {:?}", span);
//     }

//     fn max_level_hint(&self) -> Option<tracing::metadata::LevelFilter> {
//         Some(tracing::metadata::LevelFilter::from_level(tracing::Level::INFO))
//     }
// }
//}

// new_error_code!(FOO) =>
// pub const CODE_FOO: &str = "FOO";
macro_rules! new_error_code {
    ($code:ident) => {
        paste::paste! {
            pub const [<CODE_ $code>]: &str = stringify!($code);
        }
    };
}

pub(crate) use new_error_code;
