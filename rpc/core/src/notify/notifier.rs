use std::{
    sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}, 
};
use ahash::AHashMap;
use async_std::channel::Receiver;
//use workflow_core::task::spawn;
use crate::{Notification, NotificationType};
use super::{listener::{ListenerID, Listener, ListenerReceiverSide, ListenerSenderSide}, message::DispatchMessage, channel::Channel};
use super::events::{EventArray, EventType, EVENT_TYPE_ARRAY};
use super::result::Result;

/// Manage a list of events listeners and, for each one, a set of events to be notified.
/// Actually notify the listeners of incoming events.
pub struct Notifier {
    /// Map of registered listeners
    listeners: Arc<Mutex<AHashMap<ListenerID, Listener>>>,

    /// Array of dispatcher channels by event type
    dispatcher_channel: EventArray<Channel<DispatchMessage>>,

    /// Array of dispatcher shutdown listeners by event type
    dispatcher_shutdown_listener: EventArray<Option<triggered::Listener>>,

    dispatcher_is_running: EventArray<AtomicBool>,

    /// If true, filter utxos changes by address, otherwise notify all utxos changes
    filter_utxos_changes: AtomicBool,
}

impl Notifier {
    pub fn new(filter_utxos_changes: bool) -> Notifier {
        Self {
            listeners: Arc::new(Mutex::new(AHashMap::new())),
            dispatcher_channel: EventArray::default(),
            dispatcher_shutdown_listener: EventArray::default(),
            dispatcher_is_running: EventArray::default(),
            filter_utxos_changes: AtomicBool::new(filter_utxos_changes),
        }
    }

    pub async fn connect(&mut self) {
        for event in EVENT_TYPE_ARRAY.clone().into_iter() {
            if !self.dispatcher_is_running[event].load(Ordering::SeqCst) {
                let (shutdown_trigger, shutdown_listener) = triggered::trigger();
                self.dispatcher_shutdown_listener[event] = Some(shutdown_listener);
                self.dispatch_task(shutdown_trigger, self.dispatcher_channel[event].receiver());
                self.dispatcher_is_running[event].store(true, Ordering::SeqCst);
            }
        }
    }

    fn dispatch_task(
        &self,
        shutdown_trigger: triggered::Trigger,
        dispatch_rx: Receiver<DispatchMessage>) {

        // This holds the map of all active listeners for the event type
        let mut listeners: AHashMap<ListenerID, Arc<ListenerSenderSide>> = AHashMap::new();
        
        workflow_core::task::spawn(async move {
            let dispatch = dispatch_rx.recv().await.unwrap();
            loop {

                match dispatch {

                    DispatchMessage::Send(ref notification) => {
                        // Create a store for closed listeners to be removed from the map
                        let mut purge: Vec<ListenerID> = Vec::new();

                        // Broadcast the notification to all listeners
                        for (id, listener) in listeners.iter() {
                            match listener.try_send(notification.clone()){
                                Ok(_) => {},
                                Err(_) => {
                                    if listener.is_closed() {
                                        purge.push(*id);
                                    }
                                },
                            }
                        }

                        // Remove closed listeners
                        for id in purge {
                            listeners.remove(&id);
                        }
                    },

                    DispatchMessage::AddListener(id, ref listener) => {
                        listeners.insert(id, listener.clone());
                    },

                    DispatchMessage::RemoveListener(id) => {
                        listeners.remove(&id);
                    },

                    DispatchMessage::Shutdown => {
                        break;
                    },

                }

            }
            shutdown_trigger.trigger();
        });
    }

    pub fn register_new_listener(&self) -> ListenerReceiverSide {
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

    pub fn unregister_listener(&self, id: ListenerID) -> Result<()> {
        let mut listeners = self.listeners.lock().unwrap();
        if let Some(listener) = listeners.remove(&id) {
            drop(listeners);
            let active_events: Vec<EventType> = EVENT_TYPE_ARRAY.clone().into_iter().filter(|event| listener.has(*event)).collect();
            for event in active_events.iter() {
                self.stop_notify(listener.id(), *event)?;
            }
        }
        Ok(())
    }

    pub fn start_notify(&self, id: ListenerID, notification_type: NotificationType) -> Result<()> {
        let event: EventType = (&notification_type).into();
        let mut listeners = self.listeners.lock().unwrap();
        if let Some(listener) = listeners.get_mut(&id) {
            if listener.toggle(event, true) {
                let listener_sender_side = ListenerSenderSide::new(
                    listener,
                    self.filter_utxos_changes.load(Ordering::SeqCst),
                    event);
                let msg = DispatchMessage::AddListener(listener.id(), Arc::new(listener_sender_side));
                self.try_send(event, msg)?;
            }
        }
        Ok(())
    }

    pub fn notifiy(&self, notification: Arc<Notification>) -> Result<()> {
        let event: EventType = notification.as_ref().into();
        let msg = DispatchMessage::Send(notification);
        self.try_send(event, msg)?;
        Ok(())
    }

    pub fn stop_notify(&self, id: ListenerID, event: EventType) -> Result<()> {
        let mut listeners = self.listeners.lock().unwrap();
        if let Some(listener) = listeners.get_mut(&id) {
            if listener.toggle(event, false) {
                let msg = DispatchMessage::RemoveListener(listener.id());
                self.try_send(event, msg)?;
            }
        }
        Ok(())
    }

    fn try_send(&self, event: EventType, msg: DispatchMessage) -> Result<()> {
        self.dispatcher_channel[event].sender().try_send(msg)?;
        Ok(())
    }

    pub async fn disconnect(&self) -> Result<()> {
        for event in EVENT_TYPE_ARRAY.clone().into_iter() {
            self.try_send(event, DispatchMessage::Shutdown)?;
        }
        Ok(())
    }

}
