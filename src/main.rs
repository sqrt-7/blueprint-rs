use std::net::TcpListener;
use zero2prod::{run, values};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(values::LOCALHOST_RANDOM)
        .expect("failed to bind random port");

    run(listener)?.await
}
