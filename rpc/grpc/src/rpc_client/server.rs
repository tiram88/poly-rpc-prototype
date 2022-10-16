use std::net::SocketAddr;

use rpc_core::rpc_client::service::ClientApi;
use crate::{
    rpc_client::service::RpcService,
    protowire::{
        rpc_server::{RpcServer},
}};

use tonic::{
    transport::Server,
};

pub fn run_server(address: SocketAddr) {
    println!("KaspadRPCServer listening on: {}", address);

    let kaspad_rpc = RpcService {core_service: ClientApi::new()};

    let svc = RpcServer::new(kaspad_rpc);

    tokio::spawn(async move {
        Server::builder()
            .add_service(svc)
            .serve(address)
            .await
            .unwrap();
    });
}