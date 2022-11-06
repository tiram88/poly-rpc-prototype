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
    feedback_channel: Channel<SubscribeMessage>,
    feedback_shutdown_listener: Arc<Mutex<Option<triggered::Listener>>>,
    feedback_is_running: Arc<AtomicBool>,
}

impl Subscriber {
    pub fn new(
        subscription_manager: DynSubscriptionManager,
        listener_id: ListenerID,
    ) -> Self {
        Self {
            subscription_manager,
            listener_id,
            feedback_channel: Channel::default(),
            feedback_shutdown_listener: Arc::new(Mutex::new(None)),
            feedback_is_running: Arc::new(AtomicBool::default()),
        }
    }

    pub(crate) fn sender(&self) -> Sender<SubscribeMessage> {
        self.feedback_channel.sender()
    }

    pub fn start(self: Arc<Self>) {
        if self.clone().feedback_is_running.load(Ordering::SeqCst) != true {
            let (shutdown_trigger, shutdown_listener) = triggered::trigger();
            let mut feedback_shutdown_listener = self.feedback_shutdown_listener.lock().unwrap();
            *feedback_shutdown_listener = Some(shutdown_listener);
            self.feedback_task(shutdown_trigger, self.feedback_channel.receiver());
        }
    }

    /// Launch the feedback task
    fn feedback_task(
        &self,
        shutdown_trigger: triggered::Trigger,
        feedback_rx: Receiver<SubscribeMessage>)
    {
        let feedback_is_running = self.feedback_is_running.clone();
        feedback_is_running.store(true, Ordering::SeqCst);
        let subscription_manager = self.subscription_manager.clone();
        // let listener = self.listener.clone();
        let listener_id = self.listener_id;


        workflow_core::task::spawn(async move {
            loop {
                let feedback = feedback_rx.recv().await.unwrap();

                match feedback {

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
            feedback_is_running.store(false, Ordering::SeqCst);
            shutdown_trigger.trigger();
        });
    }

    fn try_send_feedback(self: Arc<Self>, msg: SubscribeMessage) -> Result<()> {
        self.feedback_channel.sender().try_send(msg)?;
        Ok(())
    }

    async fn stop_feedback(self: Arc<Self>) -> Result<()> {
        let mut result: Result<()> = Ok(());
            if self.feedback_is_running.load(Ordering::SeqCst) == true {
                match self.clone().try_send_feedback(SubscribeMessage::Shutdown) {
                    Ok(_) => {
                        let mut feedback_shutdown_listener = self.feedback_shutdown_listener.lock().unwrap();
                        let shutdown_listener = feedback_shutdown_listener.take().unwrap();
                        shutdown_listener.await;
                    },
                    Err(err) => { result = Err(err) },
                }
            }
        result
    }

    pub async fn stop(self: Arc<Self>) -> Result<()> {
        self.clone().stop_feedback().await
    }

}