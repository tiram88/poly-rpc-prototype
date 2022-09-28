use poly_rpc_prototype::protowire::{
    rpc_client::RpcClient,
    KaspadRequest, kaspad_request,
    GetBlockRequestMessage,
};

// use std::error::Error;
// use std::time::Duration;

// use futures::stream;
// use rand::rngs::ThreadRng;
// use rand::Rng;
// use tokio::time;
// use tonic::transport::Channel;
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RpcClient::connect("http://[::1]:10000").await?;

    println!("*** SIMPLE RPC ***");
    let response = client
        .message_stream(Request::new(KaspadRequest {
            payload:
                Some(kaspad_request::Payload::GetBlockRequest(
                    GetBlockRequestMessage { hash: String::from("A"), include_transactions: false }
                ))
            }))
        .await?;
    println!("RESPONSE = {:#?}", response);

    Ok(())
}

