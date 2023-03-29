#![allow(dead_code)]
#![allow(clippy::wrong_self_convention)]

pub mod datastore;
pub mod domain;
pub mod logic;
pub mod server;

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
