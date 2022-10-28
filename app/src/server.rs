use rpc_grpc::server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();
    let server_handle = server::run_server(addr);
    server_handle
        .await?
        .map_err(|x| x.into())
}