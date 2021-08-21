use std::{net::TcpListener, sync::Arc};
mod error;
mod rpc;
mod types;
fn main() {
    let server = Arc::new(rpc::Server::new().unwrap());
    rpc::start(server.clone());
}
