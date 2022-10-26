use std::str::FromStr;
use tokio::time::{sleep, Duration};
use clap::Parser;
use rpc_core::{GetBlockRequest, RpcHash, GetInfoRequest};
use rpc_core::api::client::ClientApi;
use rpc_grpc::rpc_client::client::ClientApiGrpc;
use hashes::Hash;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Full URI of kaspa node (ie. http://127.0.0.1:16110)
    #[arg(short, long)]
    address: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    let mut c = ClientApiGrpc::connect("http://[::1]:10000".to_string()).await?;

    println!("*** RUST PROTOTYPE ***");
    println!("REQUEST Existing hash");
    let request = GetBlockRequest {
        hash: RpcHash::from_str("8270e63a0295d7257785b9c9b76c9a2efb7fb8d6ac0473a1bff1571c5030e995")?,
        include_transactions: false
    };
    let response = c.get_block(request).await;
    println!("RESPONSE = {:#?}", response);

    sleep(Duration::from_millis(3_000)).await;

    println!("REQUEST Missing hash");
    let request = GetBlockRequest {
        hash: Hash::from_str("0070e63a0295d7257785b9c9b76c9a2efb7fb8d6ac0473a1bff1571c5030e995")?,
        include_transactions: false
    };
    let response = c.get_block(request).await;
    println!("RESPONSE = {:#?}", response);

    println!("REQUEST info");
    let request = GetInfoRequest {};
    let response = c.get_info(request).await;
    println!("RESPONSE = {:#?}", response);



    println!("*** GO KASPA NODE ***");
    let mut c_public = ClientApiGrpc::connect(args.address).await?;

    println!("REQUEST Public node, existing hash");
    let request = GetBlockRequest {
        hash: Hash::from_str("49beb748fd67dd354de4077aecdbba47f05bedb9dc069870c1682e6043df3abf")?,
        include_transactions: false
    };
    let response = c_public.get_block(request).await;
    println!("RESPONSE = {:#?}", response);

    println!("REQUEST Public node, existing hash");
    let request = GetBlockRequest {
        hash: Hash::from_str("733dcea265ef401e8eb0fda5429a5d769377608575022e8173f1312f91fc0e98")?,
        include_transactions: false
    };
    let response = c_public.get_block(request).await;
    println!("RESPONSE = {:#?}", response);

    println!("REQUEST Public node, existing hash");
    let request = GetBlockRequest {
        hash: Hash::from_str("d1c27dfcce46e60df4494e02bef758ad818ad2161391f29f5be129a49b3ff20b")?,
        include_transactions: false
    };
    let response = c_public.get_block(request).await;
    println!("RESPONSE = {:#?}", response);

    println!("REQUEST Public node, non-existing hash");
    let request = GetBlockRequest {
        hash: Hash::from_str("0000000000000000000000000000000000000000000000000000000000000000")?,
        include_transactions: false
    };
    let response = c_public.get_block(request).await;
    println!("RESPONSE = {:#?}", response);

    println!("REQUEST Public node, info");
    let request = GetInfoRequest {};
    let response = c_public.get_info(request).await;
    println!("RESPONSE = {:#?}", response);

    sleep(Duration::from_millis(2000)).await;

    // Closing connections
    println!("Shutting down RUST PROTOTYPE connected client");
    c.shutdown().await?;
    println!("Shutting down GO KASPA NODE connected client");
    c_public.shutdown().await?;

    sleep(Duration::from_millis(2000)).await;

    Ok(())
}

