use std::{
    sync::{Arc, Mutex}
};
use ahash::AHashMap;
//use workflow_core::task::spawn;
use crate::{Notification, NotificationType};
use super::{listener::{ListenerID, Listener, ListenerReceiverSide}, message::DispatchMessage, channel::Channel};
use super::events::{EventArray, EventType, EVENT_TYPE_ARRAY};
use super::result::Result;

/// Manage a list of events listeners and, for each one, a set of events to be notified.
/// Actually notify the listeners of incoming events.
pub struct Notifier {
    /// Map of registered listeners
    listeners: Arc<Mutex<AHashMap<ListenerID, Listener>>>,

    /// Array of dispatcher channels by event type
    event_dispacher_channel: EventArray<Channel<DispatchMessage>>,

    /// If false filter utxos changes by address, otherwise notify all utxos changes
    send_all_utxos_changes: bool,
}

impl Notifier {
    pub fn new(send_all_utxos: bool) -> Notifier {
        let test = Channel::<DispatchMessage>::default();
        Self {
            listeners: Arc::new(Mutex::new(AHashMap::new())),
            event_dispacher_channel: EventArray::default(),
            send_all_utxos_changes: send_all_utxos,
        }
    }

    pub async fn register_new_listener(&self) -> ListenerReceiverSide {
        let mut listeners = self.listeners.lock().unwrap();
        loop {
            let id = u64::from_le_bytes(rand::random::<[u8; 8]>());

            // This is very unlikely to happen but still, check for duplicates
            if !listeners.contains_key(&id) {
                let listener = Listener::new(id);
                let registration: ListenerReceiverSide = (&listener).into();
                listeners.insert(id, listener);
                return registration;
            }
        }
    }

    pub async fn unregister_listener(&self, id: ListenerID) -> Result<()> {
        let mut listeners = self.listeners.lock().unwrap();
        if let Some(listener) = listeners.remove(&id) {
            drop(listeners);
            let active_events: Vec<EventType> = EVENT_TYPE_ARRAY.clone().into_iter().filter(|event| listener.active_event[event.clone()]).collect();
            for event in active_events.iter() {
                self.stop_notify(listener.id(), event.clone()).await?;
            }
        }
        Ok(())
    }

    pub async fn start_notify(&self, id: ListenerID, notification_type: NotificationType) -> Result<()> {
        let event: EventType = (&notification_type).into();
        let mut listeners = self.listeners.lock().unwrap();
        if let Some(listener) = listeners.get_mut(&id) {
            if listener.toggle_event(event.clone(), true) {
                let msg = DispatchMessage::AddListener(listener.id(), listener.into());
                self.send_to_dispatcher(event, msg).await?;
            }
        }
        Ok(())
    }

    pub async fn stop_notify(&self, id: ListenerID, event: EventType) -> Result<()> {
        let mut listeners = self.listeners.lock().unwrap();
        if let Some(listener) = listeners.get_mut(&id) {
            if listener.toggle_event(event.clone(), false) {
                let msg = DispatchMessage::RemoveListener(listener.id());
                self.send_to_dispatcher(event, msg).await?;
            }
        }
        Ok(())
    }

    pub async fn notifiy(&self, notification: Notification) -> Result<()> {
        let event: EventType = (&notification).into();
        let msg = DispatchMessage::Send(notification);
        self.send_to_dispatcher(event, msg).await?;
        Ok(())
    }

    async fn send_to_dispatcher(&self, event: EventType, msg: DispatchMessage) -> Result<()> {
        self.event_dispacher_channel[event].sender().send(msg).await?;
        Ok(())
    }

}

impl From<&Listener> for ListenerReceiverSide {
    fn from(item: &Listener) -> Self {
        Self {
            id: item.id(),
            recv_channel: item.channel.receiver(),
        }
    }
}