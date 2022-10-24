use std::net::SocketAddr;
use std::sync::Arc;
use rpc_core::rpc_client::server::service::ClientApi;
use crate::protowire::rpc_server::RpcServer;
use crate::rpc_client::server::service::GrpcConnectionManager;
use tonic::codec::CompressionEncoding;
use tokio::sync::RwLock;
use tonic::transport::{Server, Error};
use tokio::task::JoinHandle;

pub mod service;

// see https://hyper.rs/guides/server/graceful-shutdown/
async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

pub fn run_server(address: SocketAddr) -> JoinHandle<Result<(), Error>> {
    println!("KaspadRPCServer listening on: {}", address);

    let grpc_service = service::RpcService::new(ClientApi::new(), Arc::new(RwLock::new(GrpcConnectionManager::new())));

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