use poly_rpc_prototype::protowire::{
    rpc_server::{Rpc, RpcServer},
    KaspadRequest, KaspadResponse,
    kaspad_response, kaspad_request,
    NotifyBlockAddedResponseMessage, GetBlockResponseMessage,
    RpcBlock, RpcBlockHeader,
};

// use std::collections::HashMap;
use std::pin::Pin;
// use std::sync::Arc;
// use std::time::Instant;

use futures::{Stream, StreamExt};
// use tokio::sync::mpsc;
// use tokio_stream::wrappers::ReceiverStream;
use tonic::{
    transport::Server,
    Request, Response
};

#[derive(Debug)]
struct RpcService;

#[tonic::async_trait]
impl Rpc for RpcService {

    type MessageStreamStream = Pin<Box<dyn Stream<Item = Result<KaspadResponse, tonic::Status>> + Send + 'static>>;

    async fn message_stream(
        &self,
        request: Request<tonic::Streaming<KaspadRequest>>,
    ) -> Result<Response<Self::MessageStreamStream>, tonic::Status> {
        println!("MessageStream");

        let mut stream = request.into_inner();

        let output = async_stream::try_stream! {
            while let Some(request) = stream.next().await {
                let request = request?;
                
                let payload = match request.payload {
                    Some(kaspad_request::Payload::NotifyBlockAddedRequest(_notify_block_added_request_message)) => {
                        Some(kaspad_response::Payload::NotifyBlockAddedResponse(NotifyBlockAddedResponseMessage {
                            error: None
                        }))
                    },
                    Some(kaspad_request::Payload::GetBlockRequest(_get_block_request_message)) => {
                        Some(kaspad_response::Payload::GetBlockResponse(GetBlockResponseMessage {
                            block: Some(create_dummy_rpc_block()),
                            error: None,
                        }))
                    },
                    None => None,
                };
                let response = KaspadResponse {
                    payload: payload
                };

                yield response;
            }
        };

        Ok(Response::new(Box::pin(output) as Self::MessageStreamStream))
    }
}

fn create_dummy_rpc_block() -> RpcBlock {
    RpcBlock {
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
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();

    println!("KaspadRPCServer listening on: {}", addr);

    let kaspad_rpc = RpcService {
        //features: Arc::new(data::load()),
    };

    let svc = RpcServer::new(kaspad_rpc);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}