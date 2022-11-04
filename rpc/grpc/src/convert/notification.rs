use rpc_core::Notification;

use crate::protowire::{
    BlockAddedNotificationMessage,
    kaspad_response::Payload, KaspadResponse,
};

// ----------------------------------------------------------------------------
// rpc_core to protowire
// ----------------------------------------------------------------------------

impl From<&rpc_core::Notification> for KaspadResponse {
    fn from(item: &rpc_core::Notification) -> Self {
        Self {
            payload: Some(item.into())
        }
    }
}

impl From<&rpc_core::Notification> for Payload {
    fn from(item: &rpc_core::Notification) -> Self {
        match item {
            Notification::BlockAdded(ref notif) => Payload::BlockAddedNotification ( notif.into() ),
            Notification::VirtualSelectedParentChainChanged(_) => todo!(),
            Notification::FinalityConflict(_) => todo!(),
            Notification::FinalityConflictResolved(_) => todo!(),
            Notification::UtxosChanged(_) => todo!(),
            Notification::VirtualSelectedParentBlueScoreChanged(_) => todo!(),
            Notification::VirtualDaaScoreChanged(_) => todo!(),
            Notification::PruningPointUTXOSetOverride(_) => todo!(),
            Notification::NewBlockTemplate(_) => todo!(),
        }
    }
}

impl From<&rpc_core::BlockAddedNotification> for BlockAddedNotificationMessage {
    fn from(item: &rpc_core::BlockAddedNotification) -> Self {
        Self {
            block: Some((&item.block).into()),
        }
    }
}

// ----------------------------------------------------------------------------
// protowire to rpc_core
// ----------------------------------------------------------------------------

