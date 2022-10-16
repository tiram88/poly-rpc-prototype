use async_trait::async_trait;
use tonic::Request;
use tonic::transport::{Channel, Error};
use rpc_core::{
    api::client::ClientApi, GetBlockRequest, GetBlockResponse, RpcResult, RpcError,
};
use crate::protowire::{
    KaspadRequest, kaspad_request::Payload,
    GetBlockRequestMessage,
    rpc_client::RpcClient
};

pub struct ClientApiGrpc {
    inner: RpcClient<Channel>,
}

impl ClientApiGrpc {
    pub async fn connect(address: String) -> Result<ClientApiGrpc, Error> {
        let client = RpcClient::connect(address.clone()).await?;
        Ok(ClientApiGrpc { inner: client })
    }
}

#[async_trait]
impl ClientApi for ClientApiGrpc {
    async fn get_block(&self, req: GetBlockRequest) -> RpcResult<GetBlockResponse> {
        let request: GetBlockRequestMessage = (&req).into();
        let outbound = async_stream::stream! {
            let request = KaspadRequest {
                payload:
                    Some(Payload::GetBlockRequest(request))
                };
            yield request;
        };
    
        let mut inner = self.inner.clone();
        let response = inner
            .message_stream(Request::new(outbound))
            .await
            .map_err(|x| RpcError::String(x.to_string()))?;
        let mut inbound = response.into_inner();
    
        let response = inbound
            .message()
            .await
            .map_err(|x| RpcError::String(x.to_string()))?
            .expect("a respons message is included when request succeeds");


        (&response).try_into()
    }
}