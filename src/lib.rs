#![allow(dead_code)]

pub mod datastore;
pub mod domain;
pub mod logic;
pub mod server;
pub mod settings;

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
