#![allow(dead_code)]
#![allow(clippy::wrong_self_convention)]

pub mod datastore;
pub mod logic;
pub mod server;

pub fn custom_log(
    level: log::Level,
    target: &str,
    msg: &str,
    fields: Vec<opentelemetry::KeyValue>,
) {
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
