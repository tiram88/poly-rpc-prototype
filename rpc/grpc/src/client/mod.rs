use std::sync::Arc;
use async_trait::async_trait;

use rpc_core::{
    api::rpc::RpcApi,
    api::ops::RpcApiOps,
    GetBlockRequest, GetBlockResponse,
    GetInfoRequest, GetInfoResponse,
    RpcResult,
    notify::{
        channel::NotificationChannel,
        listener::{
            ListenerReceiverSide,
            ListenerID, SendingChangedUtxo
        },
        notifier::Notifier,
    },
    NotificationType
};
use self::resolver::Resolver;
use self::result::Result;

mod errors;
mod resolver;
mod result;

pub struct RpcApiGrpc {
    inner: Arc<Resolver>,
    notifier: Arc<Notifier>,
}

impl RpcApiGrpc {
    pub async fn connect(address: String) -> Result<RpcApiGrpc>
    {
        let inner = Resolver::connect(address).await?;
        let notifier = Arc::new(Notifier::new(None, None, SendingChangedUtxo::FilteredByAddress));

        Ok(Self {
            inner,
            notifier,
        })
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


    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Notification API

    /// Register a new listenera and return an id and channel receiver.
    fn register_new_listener(&self, channel: Option<NotificationChannel>) -> ListenerReceiverSide {
        self.notifier.register_new_listener(channel)
    }

    /// Unregister an existing listener.
    /// 
    /// Stop all notifications for this listener and drop it's channel.
    async fn unregister_listener(&self, id: ListenerID) -> RpcResult<()> {
        self.notifier.unregister_listener(id)?;
        Ok(())
    }

    /// Start sending notifications of some type to a listener.
    async fn start_notify(&self, id: ListenerID, notification_type: NotificationType) -> RpcResult<()> {
        self.notifier.start_notify(id, notification_type)?;
        Ok(())
    }

    /// Stop sending notifications of some type to a listener.
    async fn stop_notify(&self, id: ListenerID, notification_type: NotificationType) -> RpcResult<()> {
        self.notifier.stop_notify(id, notification_type)?;
        Ok(())
    }
}