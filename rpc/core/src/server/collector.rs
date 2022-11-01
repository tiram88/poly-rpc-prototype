use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use async_std::channel::Receiver;
use async_std::stream::StreamExt;
use async_trait::async_trait;
use futures::{
    future::FutureExt, // for `.fuse()`
    pin_mut,
    select,
};
use consensus_core::stubs::{
    Notification as ConsensusNotification,
};

use crate::notify::{
    channel::Channel,
    collector,
    notifier::Notifier,
    result::Result,
};
use crate::utils::triggers::DuplexTrigger;

pub(crate) type ConsensusNotificationChannel = Channel<Arc<ConsensusNotification>>;
//pub(crate) type ConsensusNotificationSender = Sender<Arc<ConsensusNotification>>;
pub(crate) type ConsensusNotificationReceiver = Receiver<Arc<ConsensusNotification>>;


/// `consensus` to `rpc-core` notifications collector
#[derive(Debug)]
pub(crate) struct Collector {
    recv_channel: ConsensusNotificationReceiver,
    notifier: Arc<Notifier>,
    collect_shutdown: Arc<DuplexTrigger>,
    collect_is_running: Arc<AtomicBool>,
}

impl Collector {
    pub(crate) fn new(
        recv_channel: ConsensusNotificationReceiver,
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
                                // FIXME: handle errors here
                                let _ = notifier.clone().notifiy(Arc::new((&*msg).into()));
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
impl collector::Collector for Collector {
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

