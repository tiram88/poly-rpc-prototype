use std::sync::Arc;

use async_trait::async_trait;
use rpc_core::{
    api::client::ClientApi,
    api::ops::ClientApiOps,
    GetBlockRequest, GetBlockResponse,
    GetInfoRequest, GetInfoResponse,
    RpcResult,
};
use self::resolver::Resolver;
use self::result::Result;

mod errors;
mod resolver;
mod result;

pub struct ClientApiGrpc {
    inner: Arc<Resolver>,
}

impl ClientApiGrpc {
    pub async fn connect(address: String) -> Result<ClientApiGrpc>
    {
        let inner = Resolver::connect(address).await?;
        Ok(Self { inner })
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        self.inner.clone().shutdown().await?;
        Ok(())
    }
}

#[async_trait]
impl ClientApi for ClientApiGrpc {
    async fn get_block(&self, request: GetBlockRequest) -> RpcResult<GetBlockResponse> {
        self.inner.clone().call(ClientApiOps::GetBlock, request).await?.as_ref().try_into()
    }

    async fn get_info(&self, request: GetInfoRequest) -> RpcResult<GetInfoResponse> {
        self.inner.clone().call(ClientApiOps::GetInfo, request).await?.as_ref().try_into()
    }
}