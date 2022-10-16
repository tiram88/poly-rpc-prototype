use std::net::SocketAddr;
use rpc_core::rpc_client::server::service::ClientApi;
use crate::protowire::rpc_server::RpcServer;
use tonic::transport::{Server, Error};
use tokio::task::JoinHandle;

pub mod service;

pub fn run_server(address: SocketAddr) -> JoinHandle<Result<(), Error>> {
    println!("KaspadRPCServer listening on: {}", address);

    let grpc_service = service::RpcService {core_service: ClientApi::new()};

    let svc = RpcServer::new(grpc_service);

    let join = tokio::spawn(async move {
        Server::builder()
            .add_service(svc)
            .serve(address)
            .await
    });
    join
}