use std::sync::Arc;

use async_trait::async_trait;
use rpc_core::{
    api::rpc::RpcApi,
    api::ops::RpcApiOps,
    GetBlockRequest, GetBlockResponse,
    GetInfoRequest, GetInfoResponse,
    RpcResult,
};
use self::resolver::Resolver;
use self::result::Result;

mod errors;
mod resolver;
mod result;

pub struct RpcApiGrpc {
    inner: Arc<Resolver>,
}

impl RpcApiGrpc {
    pub async fn connect(address: String) -> Result<RpcApiGrpc>
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
impl RpcApi for RpcApiGrpc {
    async fn get_block(&self, request: GetBlockRequest) -> RpcResult<GetBlockResponse> {
        self.inner.clone().call(RpcApiOps::GetBlock, request).await?.as_ref().try_into()
    }

    async fn get_info(&self, request: GetInfoRequest) -> RpcResult<GetInfoResponse> {
        self.inner.clone().call(RpcApiOps::GetInfo, request).await?.as_ref().try_into()
    }
}