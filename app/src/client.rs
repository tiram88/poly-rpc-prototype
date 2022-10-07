use rpc_grpc::protowire::{
    rpc_client::RpcClient,
    KaspadRequest, kaspad_request,
    GetBlockRequestMessage,
    KaspadResponse,
};

// use std::error::Error;
// use std::time::Duration;

// use futures::stream;
// use rand::rngs::ThreadRng;
// use rand::Rng;
// use tokio::time;
use tonic::transport::Channel;
use tonic::Request;

async fn run_get_block(
    client: &mut RpcClient<Channel>,
    hash: String,
    include_transactions: bool
) -> Result<Option<KaspadResponse>, tonic::Status> {

    let outbound = async_stream::stream! {
        let request = KaspadRequest {
            payload:
                Some(kaspad_request::Payload::GetBlockRequest(
                    GetBlockRequestMessage { hash: hash, include_transactions: include_transactions }
                ))
            };
        yield request;
    };

    let response = client.message_stream(Request::new(outbound)).await?;
    let mut inbound = response.into_inner();

    let result = inbound.message().await;
    result
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RpcClient::connect("http://[::1]:10000").await?;

    println!("*** ONE ROUND-TRIP RPC ***");
    let response = run_get_block(&mut client, String::from("A"), false).await?;
    println!("RESPONSE = {:#?}", response);

    Ok(())
}

