use std::{
    net::SocketAddr, pin::Pin, sync::Arc, collections::HashMap,
};
use futures::{Stream, StreamExt};
use tokio::sync::{mpsc, RwLock};
use tonic::{
    Request, Response,
};
use rpc_core::{
    api::client::ClientApi as ClientApiT,
    rpc_client::server::service::ClientApi,
};
use crate::protowire::{
    rpc_server::Rpc,
    KaspadRequest, KaspadResponse,
    kaspad_request::Payload,
    GetBlockResponseMessage, NotifyBlockAddedResponseMessage, 
};


pub type GrpcSender = mpsc::Sender<KaspadResponse>;

pub struct GrpcConnection {
    address: SocketAddr,
    sender: GrpcSender,
}

impl GrpcConnection {
    pub fn new(address: SocketAddr, sender: GrpcSender) -> Self {
        Self { address, sender }
    }
    pub async fn send(&self, message: KaspadResponse) {
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
    pub async fn dismiss(& mut self, address: SocketAddr) {
        println!("dismiss a gRPC connection from: {}", address);
        if let Some(_connection) = self.connections.remove(&address) {
            //connection.sender.closed().await;
        }
    }
}

pub struct RpcService {
    pub core_service: Arc<ClientApi>,
    connection_manager: Arc<RwLock<GrpcConnectionManager>>,
}

impl RpcService {
    pub fn new(core_service: Arc<ClientApi>, connection_manager: Arc<RwLock<GrpcConnectionManager>>) -> Self {
        Self {
            core_service,
            connection_manager,
        }
    }

    pub async fn register_connection(&self, address: SocketAddr, sender: GrpcSender) {
        self.connection_manager.write().await.register(address, sender).await;
    }

    pub async fn dismiss_connection(&self, address: SocketAddr) {
        self.connection_manager.write().await.dismiss(address).await;
    }
}

#[tonic::async_trait]
impl Rpc for RpcService {

    type MessageStreamStream = Pin<
        Box<dyn Stream<Item = Result<KaspadResponse, tonic::Status>> + Send + Sync + 'static>
    >;

    async fn message_stream(
        &self,
        request: Request<tonic::Streaming<KaspadRequest>>,
    ) -> Result<Response<Self::MessageStreamStream>, tonic::Status> {
        let remote_addr = request.remote_addr()
            .ok_or(tonic::Status::new(tonic::Code::InvalidArgument, "missing or invalid request payload".to_string()))?;

        println!("MessageStream from {:?}", remote_addr);

        // External sender and reciever
        let (send_channel, mut recv_channel) = mpsc::channel::<KaspadResponse>(10);
        self.register_connection(remote_addr, send_channel.clone()).await;
        
        // Internal related sender and reciever
        let (stream_tx, stream_rx) = mpsc::channel::<Result<KaspadResponse, tonic::Status>>(10);

        // KaspadResponse forwarder
        let connection_manager = self.connection_manager.clone();
        tokio::spawn(async move {
            while let Some(msg) = recv_channel.recv().await {
                match stream_tx.send(Ok(msg)).await {
                    Ok(_) => {}
                    Err(_) => {
                        // If sending failed, then remove the connection from connection manager
                        println!("[Remote] stream tx sending error. Remote {:?}", &remote_addr);
                        connection_manager.write().await.dismiss(remote_addr).await;
                    }
                }
            }
        });

        // Request handler
        let core_service = self.core_service.clone();
        let mut stream: tonic::Streaming<KaspadRequest> = request.into_inner();
        tokio::spawn(async move {
            while let Some(request) = stream.next().await {
                println!("Request is {:?}", request);
                if let Ok(request) = request {
                    let response: KaspadResponse = match request.payload {
    
                        Some(Payload::NotifyBlockAddedRequest(_notify_block_added_request_message)) => {
                            NotifyBlockAddedResponseMessage::from(rpc_core::RpcError::NotImplemented).into()
                        },
        
                        Some(Payload::GetBlockRequest(get_block_request_message)) => {
                            let core_request: rpc_core::RpcResult<rpc_core::GetBlockRequest> = (&get_block_request_message).try_into();
                            match core_request {
                                Ok(request) => {
                                    (&(core_service.get_block(request).await)).into()
                                }
                                Err(err) => {
                                    GetBlockResponseMessage::from(err).into()
                                }
                            }
                        },
                        
                        _ => GetBlockResponseMessage::from(rpc_core::RpcError::String("err".to_string())).into()
        
                    };

                    match send_channel.send(response).await {
                        Ok(_) => {}
                        Err(err) => {
                            println!("tx send error: {:?}", err);
                        }
                    }

                } else {
                    println!("request error: {:?}", request.err());
                }
            }   
        });
        
        // Return connection stream

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(stream_rx),
        )))

    }
}