use crate::{
    Notification,
    BlockAddedNotification,
};
use consensus_core::stubs;

// ----------------------------------------------------------------------------
// consensus_core to rpc_core
// ----------------------------------------------------------------------------

impl From<&stubs::Notification> for Notification {
    fn from(item: &stubs::Notification) -> Self {
        match item {
            stubs::Notification::BlockAdded(msg) => {
                Notification::BlockAdded(msg.into())
            },
        }
    }
}

impl From<&stubs::BlockAddedNotification> for BlockAddedNotification {
    fn from(item: &stubs::BlockAddedNotification) -> Self {
        Self {
            block: (&item.block).into(),
        }
    }
}

// ----------------------------------------------------------------------------
// rpc_core to consensus_core
// ----------------------------------------------------------------------------

