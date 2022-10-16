use rpc_core::rpc_client::service::ClientApi;
use rpc_grpc::{
    rpc_client::service::RpcService,
    protowire::rpc_server::RpcServer
};

use tonic::{
    transport::Server,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();

    println!("KaspadRPCServer listening on: {}", addr);

    let kaspad_rpc = RpcService::new(ClientApi::new());
    let svc = RpcServer::new(kaspad_rpc);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}