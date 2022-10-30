use std::sync::Arc;
use async_std::channel::{Sender,Receiver};
use serde::{Deserialize, Serialize};
use borsh::{BorshSerialize, BorshDeserialize, BorshSchema};
use crate::model::message::*;
use crate::stubs::*;

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
pub enum NotificationType {
    BlockAdded,
    VirtualSelectedParentChainChanged,
    FinalityConflicts,
    UtxosChanged(Vec<Address>),
    // StopNotifyingUtxosChanged(Vec<Address>),
    VirtualSelectedParentBlueScoreChanged,
    DaaScoreChanged,
    PruningPointUTXOSetOverride,
    NewBlockTemplate,
    
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
pub enum Notification {
    
    BlockAdded(BlockAddedNotification),
    // VirtualSelectedParentChainChanged(VirtualSelectedParentChainChangedNotification),
    // FinalityConflict(FinalityConflictNotification),
    // FinalityConflictResolved(FinalityConflictResolvedNotification),
    // UtxosChanged(UtxosChangedNotification),
    // VirtualSelectedParentBlueScoreChanged(VirtualSelectedParentBlueScoreChangedNotification),
    // VirtualDaaScoreChanged(VirtualDaaScoreChangedNotification),
    // PruningPointUTXOSetOverride(PruningPointUTXOSetOverrideNotification),
    // NewBlockTemplate(NewBlockTemplateNotification),

}

pub type NotificationSender = Sender<Arc<Notification>>;
pub type NotificationReceiver = Receiver<Arc<Notification>>;

pub enum NotificationHandle {
    Existing(u64),
    New(NotificationSender),
}

impl AsRef<Notification> for Notification {
    fn as_ref(&self) -> &Self {
        &self
    }
}