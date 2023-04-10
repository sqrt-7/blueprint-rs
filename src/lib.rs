#![allow(dead_code)]
#![allow(clippy::wrong_self_convention)]

pub mod datastore;
pub mod logic;
pub mod server;

// todo: these should be in separate external modules

use uuid::Uuid as uuid_bytes;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Uuid(String);

impl Uuid {
    pub fn new() -> Self {
        Uuid(uuid_bytes::new_v4().to_string())
    }
}

impl TryFrom<String> for Uuid {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match uuid_bytes::parse_str(value.as_str()) {
            Ok(raw) => Ok(Uuid(raw.to_string())),
            Err(_) => Err(format!("invalid uuid: {}", value)),
        }
    }
}

impl Default for Uuid {
    fn default() -> Self {
        Uuid::new()
    }
}

impl ToString for Uuid {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

pub fn custom_log(level: log::Level, target: &str, msg: &str, fields: Vec<opentelemetry::KeyValue>) {
    use opentelemetry::trace::TraceContextExt;

    let msg = msg.to_owned();

    let trace_id = opentelemetry::Context::current()
        .span()
        .span_context()
        .trace_id()
        .to_string();

    let mut fields_str = String::new();

    for kv in fields.iter() {
        fields_str.push_str(format!("[{} = {}]", kv.key, kv.value).as_str());
    }

    log::log!(
        target: target,
        level,
        "[trace_id = {}] [msg = {}] {}",
        trace_id,
        msg,
        fields_str
    );

    // Add to span
    opentelemetry::Context::current()
        .span()
        .add_event(msg, fields);
}

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
