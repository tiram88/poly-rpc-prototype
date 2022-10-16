use std::str::FromStr;

use rpc_core::{GetBlockRequest, RpcHash};
use rpc_core::api::client::ClientApi;
use rpc_grpc::rpc_client::client::ClientApiGrpc;
use hashes::Hash;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let mut client = RpcClient::connect("http://[::1]:10000").await?;
    let c = ClientApiGrpc::connect("http://[::1]:10000".to_string()).await?;

    println!("*** ONE ROUND-TRIP RPC ***");
    println!("REQUEST Existing hash");
    let request = GetBlockRequest {
        hash: RpcHash::from_str("8270e63a0295d7257785b9c9b76c9a2efb7fb8d6ac0473a1bff1571c5030e995")?,
        include_transactions: false
    };
    let response = c.get_block(request).await;
    println!("RESPONSE = {:#?}", response);

    println!("REQUEST Missing hash");
    let request = GetBlockRequest {
        hash: Hash::from_str("0070e63a0295d7257785b9c9b76c9a2efb7fb8d6ac0473a1bff1571c5030e995")?,
        include_transactions: false
    };
    let response = c.get_block(request).await;
    println!("RESPONSE = {:#?}", response);

    Ok(())
}

