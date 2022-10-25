use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio_stream::wrappers::ReceiverStream;
use tonic::codec::CompressionEncoding;
use tonic::transport::Channel;
use tonic::Streaming;
use rpc_core::{
    api::client::ClientApi,
    api::ops::ClientApiOps,
    GetBlockRequest, GetBlockResponse,
    GetInfoRequest, GetInfoResponse,
    RpcResult,
};
use crate::protowire::GetInfoRequestMessage;
use crate::protowire::{
    KaspadResponse,
    rpc_client::RpcClient
};
use self::resolver::Resolver;
use self::errors::Error;
use self::result::Result;

mod errors;
mod resolver;
mod result;

pub struct ClientApiGrpc {
    _inner: RpcClient<Channel>,
    resolver: Arc<Resolver>,
    //recv_channel: Receiver<KaspadResponse>,
    forwarder_handle: JoinHandle<()>,
    //response_handler_handle: JoinHandle<()>,
    //stream: Arc<Streaming<KaspadRequest>>,
}

impl ClientApiGrpc {
    pub async fn connect(address: String) -> Result<ClientApiGrpc>
    {
        let mut client = RpcClient::connect(address).await
            .map_err(|x| Error::EndpointConnectionError(x))?
            .send_compressed(CompressionEncoding::Gzip)
            .accept_compressed(CompressionEncoding::Gzip);

        // External channel
        let (send_channel, recv) = mpsc::channel(2);
        send_channel.send(GetInfoRequestMessage {}.into()).await?;
        let resolver = Arc::new(Resolver::new(send_channel));

        // Internal channel
        let (send, recv_channel) = mpsc::channel(2);

        // KaspadResponse forwarder
        let mut stream: Streaming<KaspadResponse> = client
            .message_stream(ReceiverStream::new(recv))
            .await?
            .into_inner();

        let forwarder_handle = tokio::spawn(async move {
            loop {
                if send.is_closed() {
                    break;
                }
                match stream.message().await {
                    Ok(msg) => {
                        match msg {
                            Some(response) => {
                                if let Err(err) = send.send(response).await {
                                    println!("Client stream forwarder error: {:?}", err);
                                }
                            },
                            None =>{
                                println!("Client stream receiver error: no payload");
                                break;
                            }
                        }
                    },
                    Err(err) => {
                        println!("Client stream receiver error: {:?}", err);
                    }
                }
            }
            println!("Exiting client forwarder");
        });

        // KaspadResponse handler
        resolver.clone().receiver_task(recv_channel);

        Ok(Self {
            _inner: client,
            resolver,
            //recv_channel,
            forwarder_handle,
            //response_handler_handle,
        })
    }

    pub fn shutdown(&mut self) {
        self.forwarder_handle.abort();
        //self.response_handler_handle.abort();
    }

}

#[async_trait]
impl ClientApi for ClientApiGrpc {
    async fn get_block(&self, request: GetBlockRequest) -> RpcResult<GetBlockResponse> {
        
        let response = self.resolver.clone()
            .call(ClientApiOps::GetBlock, request)
            .await?;
        (&response).try_into()

    }

    async fn get_info(&self, request: GetInfoRequest) -> RpcResult<GetInfoResponse> {
        
        let response = self.resolver.clone()
            .call(ClientApiOps::GetInfo, request)
            .await?;
        (&response).try_into()

    }
}