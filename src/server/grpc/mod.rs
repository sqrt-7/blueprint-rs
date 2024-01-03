mod handler;

use crate::{logic, proto::blueprint_server::BlueprintServer};
use futures::Future;
use std::{error::Error, net::SocketAddr, sync::Arc};
use tokio::signal::unix::SignalKind;
use tonic::{transport::Server, Request, Status};

pub fn init(
    port: u16,
    controller: Arc<logic::Controller>,
) -> Result<impl Future<Output = Result<(), tonic::transport::Error>>, Box<dyn Error>> {
    let addr = format!("127.0.0.1:{}", port);
    let addr: SocketAddr = addr.parse()?;

    // Handler implements blueprint_server::Blueprint
    let handler = handler::Handler::new(controller);

    let svr = BlueprintServer::with_interceptor(handler, intercept_logger);

    let server = Server::builder()
        .add_service(svr)
        .serve_with_shutdown(addr, shutdown_watcher());

    Ok(server)
}

async fn shutdown_watcher() {
    let mut rx1 = tokio::signal::unix::signal(SignalKind::terminate()).unwrap();
    let mut rx2 = tokio::signal::unix::signal(SignalKind::interrupt()).unwrap();

    tokio::select! {
        _ = rx1.recv() => println!("shutdown: SIGTERM"),
        _ = rx2.recv() => println!("shutdown: SIGINT"),
    }
}

fn intercept_logger(req: Request<()>) -> Result<Request<()>, Status> {
    println!("Intercepting request: {:?}", req);
    Ok(req)
}
