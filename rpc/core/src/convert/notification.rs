use std::sync::Arc;

use crate::{
    Notification,
    BlockAddedNotification,
    notify::collector::ArcConvert,
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

/// Pseudo conversion from Arc<Notification> to Arc<Notification>.
/// This is basically a clone() op.
impl From<ArcConvert<Notification>> for Arc<Notification> {
    fn from(item: ArcConvert<Notification>) -> Self {
        (*item).clone()
    }
}

impl From<ArcConvert<stubs::Notification>> for Arc<Notification> {
    fn from(item: ArcConvert<stubs::Notification>) -> Self {
        Arc::new(
            (&**item).into()
        )
    }
}

// ----------------------------------------------------------------------------
// rpc_core to consensus_core
// ----------------------------------------------------------------------------

