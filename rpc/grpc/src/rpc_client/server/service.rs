use std::{pin::Pin, sync::Arc};
use futures::{Stream, StreamExt};
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
    kaspad_request,
    GetBlockResponseMessage, 
};

pub struct RpcService {
    pub core_service: Arc<ClientApi>,
}

impl RpcService {
    pub fn new(core_service: Arc<ClientApi>) -> Self {
        Self { core_service }
    }
}

#[tonic::async_trait]
impl Rpc for RpcService {

    type MessageStreamStream = Pin<Box<dyn Stream<Item = Result<KaspadResponse, tonic::Status>> + Send + 'static>>;

    async fn message_stream(
        &self,
        request: Request<tonic::Streaming<KaspadRequest>>,
    ) -> Result<Response<Self::MessageStreamStream>, tonic::Status> {
        println!("MessageStream");

        println!("Remote address: {:?}", request.remote_addr());
        let mut stream = request.into_inner();

        // Since all client api calls are single request and single response
        // handle the request outside the stream and let the stream yield 
        // the unique response.
        if let Some(request) = stream.next().await {
            println!("Request is {:?}", request);
            let request = request?;

            let response: KaspadResponse = match request.payload {

                Some(kaspad_request::Payload::NotifyBlockAddedRequest(_notify_block_added_request_message)) => {
                    GetBlockResponseMessage::from(rpc_core::RpcError::NotImplemented).into()
                    //Err(tonic::Status::new(tonic::Code::InvalidArgument, rpc_core::RpcError::NotImplemented.to_string()))
                },

                Some(kaspad_request::Payload::GetBlockRequest(get_block_request_message)) => {
                    let core_request: rpc_core::RpcResult<rpc_core::GetBlockRequest> = (&get_block_request_message).try_into();
                    match core_request {
                        Ok(request) => {
                            KaspadResponse::from(&(self.core_service.get_block(request).await))
                        }
                        Err(err) => {
                            GetBlockResponseMessage::from(err).into()
                        }
                    }
                },
                
                None => GetBlockResponseMessage::from(rpc_core::RpcError::String("missing or invalid payload".to_string())).into(),
            };
            
            let output = async_stream::try_stream! {
                yield response;
            };

            Ok(Response::new(Box::pin(output) as Self::MessageStreamStream))
        } else {
            Err(tonic::Status::new(tonic::Code::DeadlineExceeded, "no request, closing stream".to_string()))
        }
    }
}