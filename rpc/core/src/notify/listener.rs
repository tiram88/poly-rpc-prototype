use std::fmt::Debug;
use std::sync::Arc;

use ahash::AHashMap;

use crate::stubs::RpcUtxoAddress;
use crate::{NotificationReceiver, RpcHexData, Notification, NotificationSender};
use super::events::{EventArray, EventType};
use super::channel::NotificationChannel;
use super::result::Result;

// TODO: consider the use of a newtype instead
pub type ListenerID = u64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SendingChangedUtxo {
    /// Send all changed UTXO events, whatever the address
    All,

    /// Send all changed UTXO events filtered by the address
    FilteredByAddress,
}

#[derive(Debug)]
pub(crate) struct Listener {
    id: u64,
    channel: NotificationChannel,
    active_event: EventArray<bool>,
    utxo_addresses: Vec<RpcUtxoAddress>,
}

impl Listener {
    pub(crate) fn new(id: ListenerID, channel: Option<NotificationChannel>) -> Listener {
        let channel = channel.unwrap_or_default();
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

    /// Has registered for [`EventType`] notifications?
    pub(crate) fn has(&self, event: EventType) -> bool {
        self.active_event[event]
    }

    /// Toggle registration for [`EventType`] notifications.
    /// Return true if a change occured in the registration state.
    pub(crate) fn toggle(&mut self, event: EventType, active: bool) -> bool {
        if self.active_event[event] != active {
            self.active_event[event] = active;
            return true;
        }
        false
    }

    pub(crate) fn close(&mut self) {
        if !self.is_closed() {
            self.channel.close();
        }
    }

    pub(crate) fn is_closed(&self) -> bool {
        self.channel.is_closed()
    }

}

/// Contains the receiver side of a listener
#[derive(Debug)]
pub struct ListenerReceiverSide {
    pub id: ListenerID,
    pub recv_channel: NotificationReceiver,
}

impl From<&Listener> for ListenerReceiverSide {
    fn from(item: &Listener) -> Self {
        Self {
            id: item.id(),
            recv_channel: item.channel.receiver(),
        }
    }
}

#[derive(Debug)]
/// Contains the sender side of a listener
pub(crate) struct ListenerSenderSide {
    send_channel: NotificationSender,
    filter: Box<dyn Filter + Send + Sync>,
}

impl ListenerSenderSide {

    pub(crate) fn new(listener: &Listener, sending_changed_utxos: SendingChangedUtxo, event: EventType) -> Self {
        match event {
            EventType::UtxosChanged if sending_changed_utxos == SendingChangedUtxo::FilteredByAddress => {
                Self {
                    send_channel: listener.channel.sender(),
                    filter: Box::new(FilterUtxoAddress{
                        utxos_addresses: listener.utxo_addresses.iter().map(|x| (x.clone(), ())).collect()
                    }),
                }
            },
            _ => {
                Self {
                    send_channel: listener.channel.sender(),
                    filter: Box::new(Unfiltered{}),
                }
            },
        }
    }

    /// Try to send a notification.
    /// 
    /// If the notification does not meet requirements (see [`Notification::UtxosChanged`]) returns `Ok(false)`,
    /// otherwise returns `Ok(true)`.
    pub(crate) fn try_send(&self, notification: Arc<Notification>) -> Result<bool> {
        if self.filter.filter(notification.clone()) {
            match self.send_channel.try_send(notification) {
                Ok(_) => { return Ok(true); },
                Err(err) => { return Err(err.into()); },
            }
        }
        Ok(false)
    }

    pub(crate) fn is_closed(&self) -> bool {
        self.send_channel.is_closed()
    }

}

trait InnerFilter {
    fn filter(&self, notification: Arc<Notification>) -> bool;
}

trait Filter: InnerFilter + Debug {}

#[derive(Clone, Debug)]
struct Unfiltered;
impl InnerFilter for Unfiltered {
    fn filter(&self, _: Arc<Notification>) -> bool {
        true
    }
}
impl Filter for Unfiltered {}

#[derive(Clone, Debug)]
struct FilterUtxoAddress {
    utxos_addresses: AHashMap<RpcHexData, ()>,
}

impl InnerFilter for FilterUtxoAddress {
    fn filter(&self, notification: Arc<Notification>) -> bool {
        if let Notification::UtxosChanged(ref notification) = *notification {
            return self.utxos_addresses.contains_key(&notification.utxo_address);
        }
        false
    }
}
impl Filter for FilterUtxoAddress {}
