use poly_rpc_prototype::kaspadrpc::{self, RpcBlock, RpcBlockHeader};

// use std::collections::HashMap;
// use std::pin::Pin;
// use std::sync::Arc;
// use std::time::Instant;

// use futures::{Stream, StreamExt};
// use tokio::sync::mpsc;
// use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use kaspadrpc::kaspad_rpc_server::{KaspadRpc, KaspadRpcServer};
use kaspadrpc::{GetBlockRequestMessage, GetBlockResponseMessage};

#[derive(Debug)]
struct KaspadRpcService;

#[tonic::async_trait]
impl KaspadRpc for KaspadRpcService {
    async fn get_block(&self, request: Request<GetBlockRequestMessage>) -> Result<Response<GetBlockResponseMessage>, Status> {
        println!("GetBlock = {:?}", request);

        let block_response = GetBlockResponseMessage {
            block: Some(RpcBlock {
                header: Some(RpcBlockHeader {
                    version: 1,
                    parents: vec![],
                    hash_merkle_root: String::from("A"),
                    accepted_id_merkle_root: String::from("B"),
                    utxo_commitment: String::from("ok"),
                    timestamp: 123456789,
                    bits: 1,
                    nonce: 1234,
                    daa_score: 123456,
                    blue_work: String::from("1234567890"),
                    pruning_point: String::from("C"),
                    blue_score: 12345678901,
                }),
            }),
            error: None,
        };
        return Ok(Response::new(block_response));
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();

    println!("KaspadRPCServer listening on: {}", addr);

    let kaspad_rpc = KaspadRpcService {
        //features: Arc::new(data::load()),
    };

    let svc = KaspadRpcServer::new(kaspad_rpc);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}

