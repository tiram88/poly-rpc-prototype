use async_trait::async_trait;
use tonic::Request;
use tonic::transport::{Channel, Error};
use rpc_core::{
    api::client::ClientApi, GetBlockRequest, GetBlockResponse, RpcResult, RpcError,
};
use crate::protowire::{
    KaspadRequest,
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
        
        let request: KaspadRequest = (&req).into();
        let outbound = async_stream::stream! {
            yield request;
        };
    
        // Cloning the inner RpcClient is the recommended way to deal with mutability
        // see https://docs.rs/tonic/latest/tonic/client/index.html
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
            .ok_or(RpcError::String("missing response".to_string()))?;

        (&response).try_into()
    }
}