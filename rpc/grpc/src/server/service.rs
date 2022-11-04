use std::{
    net::SocketAddr, pin::Pin, sync::Arc, io::ErrorKind,
};
use futures::Stream;
use rpc_core::{RpcResult};
use rpc_core::notify::channel::NotificationChannel;
use rpc_core::notify::listener::{ListenerID, ListenerReceiverSide};
use tokio::sync::{mpsc, RwLock};
use tonic::{
    Request, Response,
};
use rpc_core::{
    api::rpc::RpcApi as RpcApiT,
    notify::{
        events::EVENT_TYPE_ARRAY,
        notifier::Notifier,
        collector::Collector as CollectorT,
        collector_from::RpcCoreCollector,
    },
    server::{
        service::RpcApi,
    },
};
use crate::server::StatusResult;
use crate::protowire::{
    rpc_server::Rpc,
    KaspadRequest, KaspadResponse,
    kaspad_request::Payload,
    GetBlockResponseMessage, NotifyBlockAddedResponseMessage, GetInfoResponseMessage, 
};
use super::{
    connection::{
        GrpcSender,
        GrpcConnectionManager,
    },
};


/// A protowire RPC service.
/// 
/// Relay requests to a central core service that queries the consensus.
/// 
/// Registers into a central core service in order to receive consensus notifications and
/// send those forward to the registered clients.
pub struct RpcService {
    core_service: Arc<RpcApi>,
    core_channel: NotificationChannel,
    core_listener: Arc<ListenerReceiverSide>,
    connection_manager: Arc<RwLock<GrpcConnectionManager>>,
    notifier: Arc<Notifier>,
    collector: Arc<RpcCoreCollector>,
}

impl RpcService {
    pub fn new(core_service: Arc<RpcApi>) -> Self {

        // Prepare core objects
        let core_channel = NotificationChannel::default();
        let core_listener = Arc::new(core_service.register_new_listener(Some(core_channel.clone())));
    
        // Prepare internals
        let notifier = Arc::new(Notifier::new(Some(core_service.clone().notifier()), Some(core_listener.clone()), true));
        let collector = Arc::new(RpcCoreCollector::new(core_channel.receiver(), notifier.clone()));
        let connection_manager = Arc::new(RwLock::new(GrpcConnectionManager::new(notifier.clone())));

        Self {
            core_service,
            core_channel,
            core_listener,
            connection_manager,
            notifier,
            collector,
        }
    }

    pub async fn start(&self) -> RpcResult<()> {
        // Start the internal notifier & collector
        self.notifier.clone().start();
        self.collector.clone().start()?;

        // // Register the internal notifier into core_service
        // let listener_id: ListenerID;
        // {
        //     let mut core_listener = self.core_listener.write().await;
        //     let listener = self.core_service.register_new_listener(Some(self.core_channel.clone())).await;
        //     listener_id = listener.id;
        //     *core_listener = Some(listener);
        // }

        // // Be notified of all event types from core_service

        // // TODO: implement some auto-start/stop mechanism based on the actual
        // // internal notifier clients subscribtions to events.
        // for event in EVENT_TYPE_ARRAY.clone().into_iter() {
        //     self.core_service.start_notify(listener_id, event.into()).await?;
        // }

        Ok(())
    }

    pub async fn register_connection(&self, address: SocketAddr, sender: GrpcSender) -> ListenerID {
        self.connection_manager.write().await.register(address, sender).await
    }

    pub async fn unregister_connection(&self, address: SocketAddr) {
        self.connection_manager.write().await.unregister(address).await;
    }

    pub async fn stop(&self) -> RpcResult<()> {
        // // Unregister the internal notifier from core_service.
        // // This will automatically stop the notification of all event types.
        // {
        //     let mut core_listener = self.core_listener.write().await;
        //     if (*core_listener).is_some() {
        //         let listener = (*core_listener).take().unwrap();
        //         self.core_service.unregister_listener(listener.id).await?;
        //     }
        // }

        // Unsubscribe from all notification types
        let listener_id = self.core_listener.id;
        for event in EVENT_TYPE_ARRAY.clone().into_iter() {
            self.core_service.stop_notify(listener_id, event.into()).await?;
        }

        // Stop the internal notifier & collector
        self.collector.clone().stop().await?;
        self.notifier.clone().stop().await?;

        Ok(())
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
        let listener_id = self.register_connection(remote_addr, send_channel.clone()).await;
        
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
        let core_service = self.core_service.clone();
        let connection_manager = self.connection_manager.clone();
        let notifier = self.notifier.clone();
        let mut stream: tonic::Streaming<KaspadRequest> = request.into_inner();
        tokio::spawn(async move {
            loop {
                match stream.message().await {
                    Ok(Some(request)) => {
                        println!("Request is {:?}", request);
                        let response: KaspadResponse = match request.payload {
        
                            Some(Payload::GetBlockRequest(request)) => {
                                match (&request).try_into() {
                                    Ok(request) => {  core_service.get_block(request).await.into() }
                                    Err(err) => {  GetBlockResponseMessage::from(err).into() }
                                }
                            },
                            
                            Some(Payload::GetInfoRequest(request)) => {
                                match (&request).try_into() {
                                    Ok(request) => { core_service.get_info(request).await.into() }
                                    Err(err) => { GetInfoResponseMessage::from(err).into() }
                                }
                            },
                            
                            Some(Payload::NotifyBlockAddedRequest(_request)) => {
                                NotifyBlockAddedResponseMessage::from(notifier.start_notify(listener_id, rpc_core::NotificationType::BlockAdded)).into()
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
                        println!("Request handler stream {0} got Ok(None). Connection terminated by the server", remote_addr);
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