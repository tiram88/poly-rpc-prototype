use clap::Parser;
use hashes::Hash;
use rpc_core::api::rpc::RpcApi;
use rpc_core::{GetBlockRequest, GetInfoRequest, RpcHash};
use rpc_grpc::client::RpcApiGrpc;
use std::str::FromStr;
use tokio::time::{sleep, Duration};

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

    // -------------------------------------------------------------------------------------------
    println!("************************");
    println!("***  RUST PROTOTYPE  ***");
    println!("************************");
    // -------------------------------------------------------------------------------------------

    let mut c = RpcApiGrpc::connect("http://[::1]:10000".to_string()).await?;
    c.start().await;
    println!("connection to rust prototype established");

    let c_listener = c.register_new_listener(None);
    let c_listener_recv = c_listener.recv_channel.clone();

    // Launch a reporting task
    tokio::spawn(async move {
        loop {
            if c_listener_recv.is_closed() {
                break;
            }
            match c_listener_recv.recv().await {
                Ok(notification) => {
                    println!("RUST PROTOTYPE Notification received: {}", &*notification)
                }
                Err(err) => println!("Error in notification reporting loop: {:?}", err),
            }
        }
        println!("Exiting notification reporting loop");
    });

    // Register for notifications
    c.start_notify(c_listener.id, rpc_core::NotificationType::BlockAdded).await?;

    println!("REQUEST RP Existing hash");
    let request = GetBlockRequest {
        hash: RpcHash::from_str("8270e63a0295d7257785b9c9b76c9a2efb7fb8d6ac0473a1bff1571c5030e995")?,
        include_transactions: false,
    };
    let response = c.get_block(request).await;
    println!("RESPONSE RP = {:#?}", response);

    sleep(Duration::from_millis(3_000)).await;

    println!("REQUEST RP Missing hash");
    let request = GetBlockRequest {
        hash: Hash::from_str("0070e63a0295d7257785b9c9b76c9a2efb7fb8d6ac0473a1bff1571c5030e995")?,
        include_transactions: false,
    };
    let response = c.get_block(request).await;
    println!("RESPONSE RP = {:#?}", response);

    println!("REQUEST RP info");
    let request = GetInfoRequest {};
    let response = c.get_info(request).await;
    println!("RESPONSE RP = {:#?}", response);

    // -------------------------------------------------------------------------------------------
    println!("***********************");
    println!("***  GO KASPA NODE  ***");
    println!("***********************");
    // -------------------------------------------------------------------------------------------

    let mut c_public = RpcApiGrpc::connect(args.address).await?;
    c_public.start().await;
    println!("connection to go kaspad established");

    let c_public_listener = c_public.register_new_listener(None);
    let c_public_listener_recv = c_public_listener.recv_channel.clone();

    // Launch a reporting task
    tokio::spawn(async move {
        loop {
            if c_public_listener_recv.is_closed() {
                break;
            }
            match c_public_listener_recv.recv().await {
                Ok(notification) => println!("KASPAD Notification received: {}", &*notification),
                Err(err) => println!("Error in notification reporting loop: {:?}", err),
            }
        }
        println!("Exiting notification reporting loop");
    });

    // Register for notifications
    c_public.start_notify(c_public_listener.id, rpc_core::NotificationType::BlockAdded).await?;

    println!("REQUEST GK Public node, existing hash");
    let request = GetBlockRequest {
        hash: Hash::from_str("49beb748fd67dd354de4077aecdbba47f05bedb9dc069870c1682e6043df3abf")?,
        include_transactions: false,
    };
    let response = c_public.get_block(request).await;
    println!("RESPONSE GK = {:#?}", response);

    // println!("REQUEST Public node, existing hash");
    // let request = GetBlockRequest {
    //     hash: Hash::from_str("733dcea265ef401e8eb0fda5429a5d769377608575022e8173f1312f91fc0e98")?,
    //     include_transactions: false
    // };
    // let response = c_public.get_block(request).await;
    // println!("RESPONSE = {:#?}", response);

    // println!("REQUEST Public node, existing hash");
    // let request = GetBlockRequest {
    //     hash: Hash::from_str("d1c27dfcce46e60df4494e02bef758ad818ad2161391f29f5be129a49b3ff20b")?,
    //     include_transactions: false
    // };
    // let response = c_public.get_block(request).await;
    // println!("RESPONSE = {:#?}", response);

    // println!("REQUEST Public node, non-existing hash");
    // let request = GetBlockRequest {
    //     hash: Hash::from_str("0000000000000000000000000000000000000000000000000000000000000000")?,
    //     include_transactions: false
    // };
    // let response = c_public.get_block(request).await;
    // println!("RESPONSE = {:#?}", response);

    // println!("REQUEST Public node, info");
    // let request = GetInfoRequest {};
    // let response = c_public.get_info(request).await;
    // println!("RESPONSE = {:#?}", response);

    sleep(Duration::from_millis(2500)).await;
    println!("Stop getting notifications from RUST PROTOTYPE");
    c.stop_notify(c_listener.id, rpc_core::NotificationType::BlockAdded).await?;

    sleep(Duration::from_millis(3000)).await;

    // Closing connections
    println!("Shutting down RUST PROTOTYPE connected client");
    c.unregister_listener(c_listener.id).await?;
    c.stop().await?;
    c.shutdown().await?;

    println!("Shutting down GO KASPA NODE connected client");
    c_public.unregister_listener(c_public_listener.id).await?;
    c_public_listener.recv_channel.close();
    c_public.stop().await?;
    c_public.shutdown().await?;

    //sleep(Duration::from_millis(2000)).await;

    println!("Terminating client app");

    Ok(())
}
