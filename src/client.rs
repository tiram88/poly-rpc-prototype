use poly_rpc_prototype::kaspadrpc;

// use std::error::Error;
// use std::time::Duration;

// use futures::stream;
// use rand::rngs::ThreadRng;
// use rand::Rng;
// use tokio::time;
// use tonic::transport::Channel;
use tonic::Request;

use kaspadrpc::kaspad_rpc_client::KaspadRpcClient;
use kaspadrpc::GetBlockRequestMessage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = KaspadRpcClient::connect("http://[::1]:10000").await?;

    println!("*** SIMPLE RPC ***");
    let response = client
        .get_block(Request::new(GetBlockRequestMessage { hash: String::from("A"), include_transactions: false }))
        .await?;
    println!("RESPONSE = {:#?}", response);

    Ok(())
}

