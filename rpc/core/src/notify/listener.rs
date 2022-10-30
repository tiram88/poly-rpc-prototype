use std::sync::Arc;

use ahash::AHashMap;
use async_std::channel::unbounded;

use crate::{NotificationReceiver, RpcHexData, Notification, NotificationSender};
use super::events::{EventArray, EventType};
use super::channel::NotificationChannel;

// TODO: consider use of a newtype instead
pub type ListenerID = u64;

#[derive(Debug)]
pub(crate) struct Listener {
    id: u64,
    pub channel: NotificationChannel,
    pub active_event: EventArray<bool>,
    pub utxo_addresses: Vec<RpcHexData>,             // FIXME: We want an explicit type here, ie. Vec<RpcUtxoAddress>
}

impl Listener {
    pub(crate) fn new(id: ListenerID) -> Listener {
        let channel = NotificationChannel::new(unbounded::<Arc<Notification>>());
        Self {
            id,
            channel,
            active_event: EventArray::default(),
            utxo_addresses: Vec::new(),
        }
    }

    pub(crate) fn id(&self) -> ListenerID {
        self.id
    }

    pub(crate) fn receives(&self, event: EventType) -> bool {
        self.active_event[event]
    }

    pub(crate) fn toggle_event(&mut self, event: EventType, active: bool) -> bool {
        if self.active_event[event.clone()] != active {
            self.active_event[event] = active;
            return true;
        }
        false
    }

}

/// Contains the receiver side of a listener
pub struct ListenerReceiverSide {
    pub id: ListenerID,
    pub recv_channel: NotificationReceiver,
}

/// Contains the sender side of a listener
pub(crate) struct ListenerSenderSide {
    pub send_channel: NotificationSender,
    pub utxos_addresses: AHashMap<RpcHexData, ()>,
}

impl From<&Listener> for ListenerSenderSide {
    fn from(item: &Listener) -> Self {
        Self {
            send_channel: item.channel.sender(),
            utxos_addresses: item.utxo_addresses.iter().map(|x| (x.clone(), ())).collect(),
        }
    }
}

impl From<&mut Listener> for ListenerSenderSide {
    fn from(item: &mut Listener) -> Self {
        Self {
            send_channel: item.channel.sender(),
            utxos_addresses: item.utxo_addresses.iter().map(|x| (x.clone(), ())).collect(),
        }
    }
}

