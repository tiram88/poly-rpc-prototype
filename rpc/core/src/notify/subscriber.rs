use core::fmt::Debug;
use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}};
use async_std::channel::{Receiver, Sender};
use async_trait::async_trait;
extern crate derive_more;
use crate::{
    NotificationType, RpcResult
};
use super::{
    channel::Channel,
    listener::{
        ListenerID,
    },
    message::SubscribeMessage,
    result::Result,
};

/// A manager of subscriptions to notifications for registered listeners
#[async_trait]
pub trait SubscriptionManager: Send + Sync + Debug {
    async fn start_notify(self: Arc<Self>, id: ListenerID, notification_type: NotificationType) -> RpcResult<()>;
    async fn stop_notify(self: Arc<Self>, id: ListenerID, notification_type: NotificationType) -> RpcResult<()>;
}

pub type DynSubscriptionManager = Arc<dyn SubscriptionManager>;

/// A subscriber handling subscription messages executing them into a [SubscriptionManager].
#[derive(Debug)]
pub struct Subscriber {
    /// Subscription manager
    subscription_manager: DynSubscriptionManager,
    listener_id: ListenerID,

    /// Feedback channel
    subscribe_channel: Channel<SubscribeMessage>,
    subscribe_shutdown_listener: Arc<Mutex<Option<triggered::Listener>>>,
    subscribe_is_running: Arc<AtomicBool>,
}

impl Subscriber {
    pub fn new(
        subscription_manager: DynSubscriptionManager,
        listener_id: ListenerID,
    ) -> Self {
        Self {
            subscription_manager,
            listener_id,
            subscribe_channel: Channel::default(),
            subscribe_shutdown_listener: Arc::new(Mutex::new(None)),
            subscribe_is_running: Arc::new(AtomicBool::default()),
        }
    }

    pub(crate) fn sender(&self) -> Sender<SubscribeMessage> {
        self.subscribe_channel.sender()
    }

    pub fn start(self: Arc<Self>) {
        if self.clone().subscribe_is_running.load(Ordering::SeqCst) != true {
            let (shutdown_trigger, shutdown_listener) = triggered::trigger();
            let mut subscribe_shutdown_listener = self.subscribe_shutdown_listener.lock().unwrap();
            *subscribe_shutdown_listener = Some(shutdown_listener);
            self.subscribe_task(shutdown_trigger, self.subscribe_channel.receiver());
        }
    }

    /// Launch the subscribe task
    fn subscribe_task(
        &self,
        shutdown_trigger: triggered::Trigger,
        subscribe_rx: Receiver<SubscribeMessage>)
    {
        let subscribe_is_running = self.subscribe_is_running.clone();
        subscribe_is_running.store(true, Ordering::SeqCst);
        let subscription_manager = self.subscription_manager.clone();
        // let listener = self.listener.clone();
        let listener_id = self.listener_id;


        workflow_core::task::spawn(async move {
            loop {
                let subscribe = subscribe_rx.recv().await.unwrap();

                match subscribe {

                    SubscribeMessage::StartEvent(ref notification_type) => {
                        match subscription_manager.clone().start_notify(listener_id, notification_type.clone()).await {
                            Ok(_) => (),
                            Err(err) => {
                                println!("[Reporter] start notify error: {:?}", err);
                            }
                        }
                    },

                    SubscribeMessage::StopEvent(ref notification_type) => {
                        match subscription_manager.clone().stop_notify(listener_id, notification_type.clone()).await {
                            Ok(_) => (),
                            Err(err) => {
                                println!("[Reporter] start notify error: {:?}", err);
                            }
                        }
                    },

                    SubscribeMessage::Shutdown => {
                        break;
                    },

                }

            }
            subscribe_is_running.store(false, Ordering::SeqCst);
            shutdown_trigger.trigger();
        });
    }

    fn try_send_subscribe(self: Arc<Self>, msg: SubscribeMessage) -> Result<()> {
        self.subscribe_channel.sender().try_send(msg)?;
        Ok(())
    }

    async fn stop_subscribe(self: Arc<Self>) -> Result<()> {
        let mut result: Result<()> = Ok(());
            if self.subscribe_is_running.load(Ordering::SeqCst) == true {
                match self.clone().try_send_subscribe(SubscribeMessage::Shutdown) {
                    Ok(_) => {
                        let mut subscribe_shutdown_listener = self.subscribe_shutdown_listener.lock().unwrap();
                        let shutdown_listener = subscribe_shutdown_listener.take().unwrap();
                        shutdown_listener.await;
                    },
                    Err(err) => { result = Err(err) },
                }
            }
        result
    }

    pub async fn stop(self: Arc<Self>) -> Result<()> {
        self.clone().stop_subscribe().await
    }

}