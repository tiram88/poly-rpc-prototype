use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use async_std::channel::{
    Receiver,
    Sender,
};
use async_std::stream::StreamExt;
use async_trait::async_trait;
use futures::{
    future::FutureExt, // for `.fuse()`
    pin_mut,
    select,
};
use crate::Notification;
use crate::notify::{
    channel::Channel,
    collector,
    notifier::Notifier,
    result::Result,
};
use crate::utils::triggers::DuplexTrigger;

use super::collector::ArcConvert;

pub type CollectorNotificationChannel<T> = Channel<Arc<T>>;
pub type CollectorNotificationSender<T> = Sender<Arc<T>>;
pub type CollectorNotificationReceiver<T> = Receiver<Arc<T>>;


/// A notifications collector that receives [`T`] from a channel,
/// converts it into a [`Notification`] and sends it to a its 
/// [`Notifier`].
#[derive(Debug)]
pub struct CollectorFrom<T>
where
    T: Send + Sync + 'static + Sized,
{
    recv_channel: CollectorNotificationReceiver<T>,
    notifier: Arc<Notifier>,
    collect_shutdown: Arc<DuplexTrigger>,
    collect_is_running: Arc<AtomicBool>,
}

impl<T> CollectorFrom<T>
where
    T: Send + Sync + 'static + Sized,
    ArcConvert<T>: Into<Arc<Notification>>,
{
    pub fn new(
        recv_channel: CollectorNotificationReceiver<T>,
        notifier: Arc<Notifier>
    ) -> Self {
        Self {
            recv_channel,
            notifier,
            collect_shutdown: Arc::new(DuplexTrigger::new()),
            collect_is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    fn collect_task(&self) {
        let collect_shutdown = self.collect_shutdown.clone();
        let collect_is_running = self.collect_is_running.clone();
        let recv_channel = self.recv_channel.clone();
        let notifier = self.notifier.clone();
        collect_is_running.store(true, Ordering::SeqCst);

        workflow_core::task::spawn(async move {

            let shutdown = collect_shutdown.request.listener.clone().fuse();
            pin_mut!(shutdown);

            let notifications = recv_channel.fuse();
            pin_mut!(notifications);

            loop {

                select! {
                    _ = shutdown => { break; }
                    notification = notifications.next().fuse() => {
                        match notification {
                            Some(msg) => {
                                match notifier.clone().notifiy(ArcConvert::from(msg.clone()).into()) {
                                    Ok(_) => (),
                                    Err(err) => {
                                        println!("[CollectorSingleConver] notification sender error: {:?}", err);
                                    },
                                }
                            },
                            None => {}
                        }
                    }
                }

            }
            collect_is_running.store(false, Ordering::SeqCst);
            collect_shutdown.response.trigger.trigger();
        });

    }

    async fn stop_collect(&self) -> Result<()> {
        if self.collect_is_running.load(Ordering::SeqCst) != true {
            return Ok(());
        }

        self.collect_shutdown.request.trigger.trigger();
        self.collect_shutdown.response.listener.clone().await;
        
        Ok(())
    }

}

#[async_trait]
impl<T> collector::Collector for CollectorFrom<T>
where
    T: Send + Sync + 'static + Sized,
    ArcConvert<T>: Into<Arc<Notification>>,
{
    fn start(self: Arc<Self>) -> Result<()> {
        self.collect_task();
        Ok(())
    }

    async fn stop(self: Arc<Self>) -> Result<()> {
        self.stop_collect().await
    }
    
    fn notifier(self: Arc<Self>) -> Arc<Notifier> {
        self.notifier.clone()
    }
}

/// A rpc_core notification collector providing a simple pass-through.
/// No conversion occurs since both source and target data are of 
/// type [`Notification`].
pub type RpcCoreCollector = CollectorFrom<Notification>;