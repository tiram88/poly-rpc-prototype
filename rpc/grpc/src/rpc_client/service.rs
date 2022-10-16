use std::{pin::Pin, sync::Arc};
use futures::{Stream, StreamExt};
use tonic::{
    Request, Response,
};
use rpc_core::{
    api::client::ClientApi as ClientApiT,
    rpc_client::service::ClientApi,
};
use crate::protowire::{
    rpc_server::Rpc,
    KaspadRequest, KaspadResponse,
    kaspad_response, kaspad_request,
    NotifyBlockAddedResponseMessage, GetBlockResponseMessage,
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

        let mut stream = request.into_inner();
        let core_service = self.core_service.clone();

        let output = async_stream::try_stream! {
            while let Some(request) = stream.next().await {
                println!("Request is {:?}", request);
                let request = request?;
                
                let payload = match request.payload {

                    Some(kaspad_request::Payload::NotifyBlockAddedRequest(_notify_block_added_request_message)) => {
                        Some(kaspad_response::Payload::NotifyBlockAddedResponse(NotifyBlockAddedResponseMessage {
                            error: None
                        }))
                    },

                    Some(kaspad_request::Payload::GetBlockRequest(get_block_request_message)) => {
                        // let response = self.core_service.get_block((&get_block_request_message).try_into());
                        let core_request: rpc_core::GetBlockRequest = (&get_block_request_message).try_into().unwrap();
                        let core_response = core_service.get_block(core_request).await;
                        let response = GetBlockResponseMessage::from(&core_response);
                        Some(kaspad_response::Payload::GetBlockResponse(response))
                    },
                    
                    None => None,
                };
                let response = KaspadResponse {
                    payload: payload
                };

                yield response;
            }
        };

        Ok(Response::new(Box::pin(output) as Self::MessageStreamStream))
    }
}