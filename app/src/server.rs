use rpc_core::server::service::RpcApi;
use rpc_grpc::server;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let consensus_block_producer = Arc::new(consensus::notifiy::RandomBlockProducer::new());
    let consensus_recv = (&consensus_block_producer).start().await;
    let core_service = RpcApi::new(consensus_recv);
    core_service.start();

    let addr = "[::1]:10000".parse().unwrap();
    let server_handle = server::run_server(addr, core_service);
    server_handle.await?.map_err(|x| x.into())
}
