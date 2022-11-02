use std::net::SocketAddr;
use rpc_core::server::service::RpcApi;
use crate::protowire::rpc_server::RpcServer;
use tonic::codec::CompressionEncoding;
use tonic::transport::{Server, Error};
use tokio::task::JoinHandle;

pub mod collector;
pub mod connection;
pub mod service;

pub type StatusResult<T> = Result<T, tonic::Status>;


// see https://hyper.rs/guides/server/graceful-shutdown/
async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

pub fn run_server(address: SocketAddr) -> JoinHandle<Result<(), Error>> {
    println!("KaspadRPCServer listening on: {}", address);

    let grpc_service = service::RpcService::new(RpcApi::new());

    let svc = RpcServer::new(grpc_service)
        .send_compressed(CompressionEncoding::Gzip)
        .accept_compressed(CompressionEncoding::Gzip);

    let join = tokio::spawn(async move {
        Server::builder()
            .add_service(svc)
            .serve_with_shutdown(address, shutdown_signal())
            .await
    });
    join
}