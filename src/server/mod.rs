use std::{error::Error, net::TcpListener};

pub mod http;

pub fn create_listener(addr: String) -> Result<TcpListener, Box<dyn Error>> {
    let listener = TcpListener::bind(addr)?;
    Ok(listener)
}
