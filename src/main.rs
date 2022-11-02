use zero2prod::{datastore::inmem::InMemDatastore, service::Service, LOCALHOST_RANDOM};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let datastore = InMemDatastore::new();
    let mut app = Service::new(datastore);
    let server = app.start_http_server(LOCALHOST_RANDOM).unwrap();

    server.await
}
