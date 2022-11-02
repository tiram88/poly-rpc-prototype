use std::{
    net::SocketAddr, collections::HashMap,
};
use tokio::sync::mpsc;
use crate::protowire::{KaspadResponse};
use crate::server::StatusResult;

pub type GrpcSender = mpsc::Sender<StatusResult<KaspadResponse>>;

pub struct GrpcConnection {
    address: SocketAddr,
    sender: GrpcSender,
}

impl GrpcConnection {
    pub fn new(address: SocketAddr, sender: GrpcSender) -> Self {
        Self { address, sender }
    }
    pub async fn send(&self, message: StatusResult<KaspadResponse>) {
        match self.sender.send(message).await {
            Ok(_) => {}
            Err(err) => {
                println!("[send] SendError: to {}, {:?}", self.address, err);
                // TODO: drop this connection
            }
        }
    }
}

pub struct GrpcConnectionManager {
    connections: HashMap<SocketAddr, GrpcConnection>,
}

impl GrpcConnectionManager {
    pub fn new() -> Self {
        Self { connections: HashMap::new(), }
    }

    pub async fn register(&mut self, address: SocketAddr, sender: GrpcSender) {
        println!("register a new gRPC connection from: {}", address);
        let connection = GrpcConnection::new(address, sender);
        match self.connections.insert(address.clone(), connection) {
            Some(_prev) => {
                //prev.sender.closed().await
            },
            None => {}
        }
    }
    pub async fn unregister(& mut self, address: SocketAddr) {
        println!("dismiss a gRPC connection from: {}", address);
        if let Some(_connection) = self.connections.remove(&address) {
            //connection.sender.closed().await;
        }
    }
}