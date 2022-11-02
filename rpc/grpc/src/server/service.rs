use std::{
    net::SocketAddr, pin::Pin, sync::Arc, io::ErrorKind,
};
use futures::Stream;
use tokio::sync::{mpsc, RwLock};
use tonic::{
    Request, Response,
};
use rpc_core::{
    api::rpc::RpcApi as RpcApiT,
    server::service::RpcApi,
    RpcResult,
};
use crate::server::StatusResult;
use crate::protowire::{
    rpc_server::Rpc,
    KaspadRequest, KaspadResponse,
    kaspad_request::Payload,
    GetBlockResponseMessage, NotifyBlockAddedResponseMessage, GetInfoResponseMessage, 
};
use super::connection::{
    GrpcSender,
    GrpcConnectionManager,
};



pub struct RpcService {
    core_service: Arc<RpcApi>,
    connection_manager: Arc<RwLock<GrpcConnectionManager>>,
}

impl RpcService {
    pub fn new(core_service: Arc<RpcApi>) -> Self {
        let connection_manager = Arc::new(RwLock::new(GrpcConnectionManager::new()));
        Self {
            core_service,
            connection_manager,
        }
    }

    pub async fn register_connection(&self, address: SocketAddr, sender: GrpcSender) {
        self.connection_manager.write().await.register(address, sender).await;
    }

    pub async fn unregister_connection(&self, address: SocketAddr) {
        self.connection_manager.write().await.unregister(address).await;
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
            .ok_or(tonic::Status::new(tonic::Code::InvalidArgument, "Incoming connection opening request has no remote address".to_string()))?;

        println!("MessageStream from {:?}", remote_addr);

        // External sender and reciever
        let (send_channel, mut recv_channel) = mpsc::channel::<StatusResult<KaspadResponse>>(128);
        self.register_connection(remote_addr, send_channel.clone()).await;
        
        // Internal related sender and reciever
        let (stream_tx, stream_rx) = mpsc::channel::<StatusResult<KaspadResponse>>(10);

        // KaspadResponse forwarder
        let connection_manager = self.connection_manager.clone();
        tokio::spawn(async move {
            while let Some(msg) = recv_channel.recv().await {
                match stream_tx.send(msg).await {
                    Ok(_) => {}
                    Err(_) => {
                        // If sending failed, then remove the connection from connection manager
                        println!("[Remote] stream tx sending error. Remote {:?}", &remote_addr);
                        connection_manager.write().await.unregister(remote_addr).await;
                    }
                }
            }
        });

        // Request handler
        let connection_manager = self.connection_manager.clone();
        let core_service = self.core_service.clone();
        let mut stream: tonic::Streaming<KaspadRequest> = request.into_inner();
        tokio::spawn(async move {
            loop {
                match stream.message().await {
                    Ok(Some(request)) => {
                        println!("Request is {:?}", request);
                        let response: KaspadResponse = match request.payload {
        
                            Some(Payload::NotifyBlockAddedRequest(_notify_block_added_request_message)) => {
                                NotifyBlockAddedResponseMessage::from(rpc_core::RpcError::NotImplemented).into()
                            },
            
                            Some(Payload::GetBlockRequest(request)) => {
                                let core_request: RpcResult<rpc_core::GetBlockRequest> = (&request).try_into();
                                match core_request {
                                    Ok(request) => {
                                        (&(core_service.get_block(request).await)).into()
                                    }
                                    Err(err) => {
                                        GetBlockResponseMessage::from(err).into()
                                    }
                                }
                            },
                            
                            Some(Payload::GetInfoRequest(request)) => {
                                let core_request: RpcResult<rpc_core::GetInfoRequest> = (&request).try_into();
                                match core_request {
                                    Ok(request) => {
                                        (&(core_service.get_info(request).await)).into()
                                    }
                                    Err(err) => {
                                        GetInfoResponseMessage::from(err).into()
                                    }
                                }
                            },
                            
                            _ => GetBlockResponseMessage::from(rpc_core::RpcError::String("Server-side API Not implemented".to_string())).into()
            
                        };

                        match send_channel.send(Ok(response)).await {
                            Ok(_) => {}
                            Err(err) => {
                                println!("tx send error: {:?}", err);
                            }
                        }
                    },
                    Ok(None) => {
                        //println!("request error: {:?}", request.err());
                        println!("Request handler stream {0} got Ok(None). Connection terminated by the client.", remote_addr);
                        break;
                    },
                    Err(err) => {
                        if let Some(io_err) = match_for_io_error(&err) {
                            if io_err.kind() == ErrorKind::BrokenPipe {
                                // here you can handle special case when client
                                // disconnected in unexpected way
                                eprintln!("\tRequest handler stream {0} error: client disconnected, broken pipe", remote_addr);
                                break;
                            }
                        }

                        match send_channel.send(Err(err)).await {
                            Ok(_) => (),
                            Err(_err) => break, // response was droped
                        }
                    }
                }
            }
            println!("Request handler {0} terminated", remote_addr);
            connection_manager.write().await.unregister(remote_addr).await;
        });
        
        // Return connection stream

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(stream_rx),
        )))

    }
}

fn match_for_io_error(err_status: &tonic::Status) -> Option<&std::io::Error> {
    let mut err: &(dyn std::error::Error + 'static) = err_status;

    loop {
        if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
            return Some(io_err);
        }

        // h2::Error do not expose std::io::Error with `source()`
        // https://github.com/hyperium/h2/pull/462
        if let Some(h2_err) = err.downcast_ref::<h2::Error>() {
            if let Some(io_err) = h2_err.get_io() {
                return Some(io_err);
            }
        }

        err = match err.source() {
            Some(err) => err,
            None => return None,
        };
    }
}