use std::sync::{Arc};
use consensus_core::stubs::{
    Notification as ConsensusNotification,
};
use crate::notify::{
    channel::Channel,
    collector_from::CollectorFrom,
};

pub(crate) type ConsensusCollector = CollectorFrom<ConsensusNotification>;

pub(crate) type ConsensusNotificationChannel = Channel<Arc<ConsensusNotification>>;
