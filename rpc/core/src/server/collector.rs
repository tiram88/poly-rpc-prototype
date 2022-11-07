use std::sync::{Arc};
use async_std::channel::{Sender, Receiver};
use consensus_core::stubs::{
    Notification as ConsensusNotification,
};
use crate::notify::{
    channel::Channel,
    collector::CollectorFrom,
};

pub(crate) type ConsensusCollector = CollectorFrom<ConsensusNotification>;

pub type ConsensusNotificationChannel = Channel<Arc<ConsensusNotification>>;
pub type ConsensusNotificationSender = Sender<Arc<ConsensusNotification>>;
pub type ConsensusNotificationReceiver = Receiver<Arc<ConsensusNotification>>;
