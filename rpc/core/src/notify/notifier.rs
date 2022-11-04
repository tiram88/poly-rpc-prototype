use std::{
    sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}, 
};
use ahash::AHashMap;
use async_std::channel::{Receiver, Sender};
use crate::{Notification, NotificationType};
use super::{
    channel::{
        Channel,
        NotificationChannel
    },
    events::{
        EventArray,
        EventType,
        EVENT_TYPE_ARRAY,
    },
    listener::{
        ListenerID,
        Listener,
        ListenerReceiverSide,
        ListenerSenderSide, SendingChangedUtxo
    },
    message::{DispatchMessage, FeedbackMessage},
    result::Result,
};

/// A notification sender
/// 
/// Manage a collection of [Listener] and, for each one, a set of events to be notified.
/// Actually notify the listeners of incoming events.
#[derive(Debug)]
pub struct Notifier {
    inner: Arc<Inner>,
}

impl Notifier {
    pub fn new(
        source_notifier: Option<Arc<Notifier>>,
        source_listener: Option<Arc<ListenerReceiverSide>>,
        sending_changed_utxos: SendingChangedUtxo
    ) -> Self {
        Self {
            inner: Arc::new(Inner::new(source_notifier, source_listener, sending_changed_utxos)),
        }
    }

    pub fn start(&self) {
        self.inner.clone().start();
    }

    pub fn register_new_listener(&self, channel: Option<NotificationChannel>) -> ListenerReceiverSide {
        self.inner.clone().register_new_listener(channel)
    }

    pub fn unregister_listener(&self, id: ListenerID) -> Result<()> {
        self.inner.clone().unregister_listener(id)
    }

    pub fn start_notify(&self, id: ListenerID, notification_type: NotificationType) -> Result<()> {
        self.inner.clone().start_notify(id, notification_type)
    }

    pub fn notifiy(self: Arc<Self>, notification: Arc<Notification>) -> Result<()> {
        self.inner.clone().notifiy(notification)
    }

    pub fn stop_notify(&self, id: ListenerID, notification_type: NotificationType) -> Result<()> {
        self.inner.clone().stop_notify(id, notification_type)
    }

    pub async fn stop(&self) -> Result<()> {
        self.inner.clone().stop().await
    }

}

#[derive(Debug)]
struct Inner {
    /// Source notifier
    source_notifier: Option<Arc<Notifier>>,
    source_listener: Option<Arc<ListenerReceiverSide>>,

    /// Map of registered listeners
    listeners: Arc<Mutex<AHashMap<ListenerID, Listener>>>,

    /// Dispatcher channels by event type
    dispatcher_channel: EventArray<Channel<DispatchMessage>>,
    dispatcher_shutdown_listener: Arc<Mutex<EventArray<Option<triggered::Listener>>>>,
    dispatcher_is_running: EventArray<Arc<AtomicBool>>,

    /// Feedback channel
    feedback_channel: Channel<FeedbackMessage>,
    feedback_shutdown_listener: Arc<Mutex<Option<triggered::Listener>>>,
    feedback_is_running: Arc<AtomicBool>,
    
    /// How to handle UtxoChanged notifications
    sending_changed_utxos: SendingChangedUtxo,
}

impl Inner {
    fn new(
        source_notifier: Option<Arc<Notifier>>,
        source_listener: Option<Arc<ListenerReceiverSide>>,
        sending_changed_utxos: SendingChangedUtxo
    ) -> Self {
        Self {
            source_notifier,
            source_listener,
            listeners: Arc::new(Mutex::new(AHashMap::new())),
            dispatcher_channel: EventArray::default(),
            dispatcher_shutdown_listener: Arc::new(Mutex::new(EventArray::default())),
            dispatcher_is_running: EventArray::default(),
            feedback_channel: Channel::default(),
            feedback_shutdown_listener: Arc::new(Mutex::new(None)),
            feedback_is_running: Arc::new(AtomicBool::default()),
            sending_changed_utxos,
        }
    }

    fn has_source(&self) -> bool {
        self.clone().source_listener.is_some() &&
        self.clone().source_notifier.is_some()
    }

    fn start(self: Arc<Self>) {
        if self.clone().feedback_is_running.load(Ordering::SeqCst) != true &&
           self.clone().has_source()
        {
            let (shutdown_trigger, shutdown_listener) = triggered::trigger();
            let mut feedback_shutdown_listener = self.feedback_shutdown_listener.lock().unwrap();
            *feedback_shutdown_listener = Some(shutdown_listener);
            self.feedback_task(shutdown_trigger, self.feedback_channel.receiver());
        }

        for event in EVENT_TYPE_ARRAY.clone().into_iter() {
            if self.clone().dispatcher_is_running[event].load(Ordering::SeqCst) != true {
                let (shutdown_trigger, shutdown_listener) = triggered::trigger();
                let mut dispatcher_shutdown_listener = self.dispatcher_shutdown_listener.lock().unwrap();
                dispatcher_shutdown_listener[event] = Some(shutdown_listener);
                self.dispatch_task(event, shutdown_trigger, self.dispatcher_channel[event].receiver());
            }
        }
    }

    /// Launch a dispatch task for an event type.
    /// 
    /// Implementation note:
    /// The separation by event type allows to keep an internal map
    /// with all listeners willing to receive notification of the
    /// corresponding type. The dispatcher receives and execute messages
    /// instructing to modify the map. This happens without blocking
    /// the whole notifier.
    fn dispatch_task(
        &self,
        event: EventType,
        shutdown_trigger: triggered::Trigger,
        dispatch_rx: Receiver<DispatchMessage>)
    {
        let dispatcher_is_running = self.dispatcher_is_running[event].clone();
        dispatcher_is_running.store(true, Ordering::SeqCst);

        // Feedback
        let sending_changed_utxos = self.sending_changed_utxos;
        let has_source = self.has_source();
        let feedback_tx = self.feedback_channel.sender();

        // This holds the map of all active listeners for the event type
        let mut listeners: AHashMap<ListenerID, Arc<ListenerSenderSide>> = AHashMap::new();

        // TODO: feed the listeners map with pre-existing self.listeners having event active
        // This is necessary for the correct handling of repeating start/stop cycles.
        
        workflow_core::task::spawn(async move {
            
            fn send_feedback(has_source: bool, feedback_tx: Sender<FeedbackMessage>, message: FeedbackMessage) {
                if has_source {

                    // TODO: handle actual utxo addresse set

                    match feedback_tx.try_send(message) {
                        Ok(_) => {},
                        Err(err) => {
                            println!("[Notifier] sending feedback error: {:?}", err);
                        },
                    }
                }
            }

            // We will send feedback for all dispatch messages if event is a filtered UtxosChanged.
            // Otherwise, feedback is only sent when needed when applying the dispatched message.
            let report_all_changes = event == EventType::UtxosChanged && sending_changed_utxos == SendingChangedUtxo::FilteredByAddress;

            let mut need_feedback: bool = true;
            loop {
                // If needed, send feedback based on listener being empty or not
                if need_feedback {
                    if listeners.len() > 0 {
                        send_feedback(has_source, feedback_tx.clone(), FeedbackMessage::StartEvent(event.into()));
                    } else {
                        send_feedback(has_source, feedback_tx.clone(), FeedbackMessage::StopEvent(event.into()));
                    }
                }
                let dispatch = dispatch_rx.recv().await.unwrap();

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

                        // Feedback needed if purge does empty listeners or if reporting any change
                        need_feedback = (purge.len() == listeners.len()) || report_all_changes;

                        // Remove closed listeners
                        for id in purge {
                            listeners.remove(&id);
                        }
                    },

                    DispatchMessage::AddListener(id, listener) => {
                        // We don't care whether this is an insertion or a replacement
                        listeners.insert(id, listener.clone());

                        // Feedback needed if a first listener was added or if reporting any change
                        need_feedback = listeners.len() == 1 || report_all_changes;
                    },

                    DispatchMessage::RemoveListener(id) => {
                        listeners.remove(&id);

                        // Feedback needed if no more listeners are present or if reporting any change
                        need_feedback = listeners.len() == 0 || report_all_changes;
                    },

                    DispatchMessage::Shutdown => {
                        break;
                    },

                }

            }
            dispatcher_is_running.store(false, Ordering::SeqCst);
            shutdown_trigger.trigger();
        });
    }

    /// Launch the feedback task
    fn feedback_task(
        &self,
        shutdown_trigger: triggered::Trigger,
        feedback_rx: Receiver<FeedbackMessage>)
    {
        let feedback_is_running = self.feedback_is_running.clone();
        feedback_is_running.store(true, Ordering::SeqCst);
        let has_source = self.has_source();
        let source_notifier = self.source_notifier.clone();
        let source_listener = self.source_listener.clone();

        workflow_core::task::spawn(async move {
            loop {
                let feedback = feedback_rx.recv().await.unwrap();

                match feedback {

                    FeedbackMessage::StartEvent(ref notification_type) => {
                        if has_source {
                            match source_notifier.clone().unwrap().start_notify(source_listener.clone().unwrap().id, notification_type.clone()) {
                                Ok(_) => (),
                                Err(err) => {
                                    println!("[Notifier] start notify error: {:?}", err);
                                }
                            }
                        }
                    },

                    FeedbackMessage::StopEvent(ref notification_type) => {
                        if has_source {
                            match source_notifier.clone().unwrap().stop_notify(source_listener.clone().unwrap().id, notification_type.clone()) {
                                Ok(_) => (),
                                Err(err) => {
                                    println!("[Notifier] start notify error: {:?}", err);
                                }
                            }
                        }
                    },

                    FeedbackMessage::Shutdown => {
                        break;
                    },

                }

            }
            feedback_is_running.store(false, Ordering::SeqCst);
            shutdown_trigger.trigger();
        });
    }

    fn register_new_listener(self: Arc<Self>, channel: Option<NotificationChannel>) -> ListenerReceiverSide {
        let mut listeners = self.listeners.lock().unwrap();
        loop {
            let id = u64::from_le_bytes(rand::random::<[u8; 8]>());

            // This is very unlikely to happen but still, check for duplicates
            if !listeners.contains_key(&id) {
                let listener = Listener::new(id, channel);
                let registration: ListenerReceiverSide = (&listener).into();
                listeners.insert(id, listener);
                return registration;
            }
        }
    }

    fn unregister_listener(self: Arc<Self>, id: ListenerID) -> Result<()> {
        let mut listeners = self.listeners.lock().unwrap();
        if let Some(mut listener) = listeners.remove(&id) {
            drop(listeners);
            let active_events: Vec<EventType> = EVENT_TYPE_ARRAY.clone().into_iter().filter(|event| listener.has(*event)).collect();
            for event in active_events.iter() {
                self.clone().stop_notify(listener.id(), (*event).into())?;
            }
            listener.close();
        }
        Ok(())
    }

    fn start_notify(self: Arc<Self>, id: ListenerID, notification_type: NotificationType) -> Result<()> {
        let event: EventType = (&notification_type).into();
        let mut listeners = self.listeners.lock().unwrap();
        if let Some(listener) = listeners.get_mut(&id) {

            // Any mutation in the listener will trigger a dispatch of a brand new ListenerSenderSide
            // eventually creating or replacing this listener in the matching dispatcher.

            if listener.toggle(notification_type, true) {
                let listener_sender_side = ListenerSenderSide::new(
                    listener,
                    self.sending_changed_utxos,
                    event);
                let msg = DispatchMessage::AddListener(listener.id(), Arc::new(listener_sender_side));
                self.clone().try_send_dispatch(event, msg)?;
            }

        }
        Ok(())
    }

    fn notifiy(self: Arc<Self>, notification: Arc<Notification>) -> Result<()> {
        let event: EventType = notification.as_ref().into();
        let msg = DispatchMessage::Send(notification);
        self.try_send_dispatch(event, msg)?;
        Ok(())
    }

    fn stop_notify(self: Arc<Self>, id: ListenerID, notification_type: NotificationType) -> Result<()> {
        let event: EventType = (&notification_type).into();
        let mut listeners = self.listeners.lock().unwrap();
        if let Some(listener) = listeners.get_mut(&id) {
            if listener.toggle(notification_type, false) {
                let msg = DispatchMessage::RemoveListener(listener.id());
                self.clone().try_send_dispatch(event, msg)?;
            }
        }
        Ok(())
    }

    fn try_send_dispatch(self: Arc<Self>, event: EventType, msg: DispatchMessage) -> Result<()> {
        self.dispatcher_channel[event].sender().try_send(msg)?;
        Ok(())
    }

    fn try_send_feedback(self: Arc<Self>, msg: FeedbackMessage) -> Result<()> {
        self.feedback_channel.sender().try_send(msg)?;
        Ok(())
    }

    async fn stop_dispatch(self: Arc<Self>) -> Result<()> {
        let mut result: Result<()> = Ok(());
        for event in EVENT_TYPE_ARRAY.clone().into_iter() {
            if self.dispatcher_is_running[event].load(Ordering::SeqCst) == true {
                match self.clone().try_send_dispatch(event, DispatchMessage::Shutdown) {
                    Ok(_) => {
                        let mut dispatcher_shutdown_listener = self.dispatcher_shutdown_listener.lock().unwrap();
                        let shutdown_listener = dispatcher_shutdown_listener[event].take().unwrap();
                        shutdown_listener.await;    
                    },
                    Err(err) => { result = Err(err) },
                }
            }
        }
        result
    }

    async fn stop_feedback(self: Arc<Self>) -> Result<()> {
        let mut result: Result<()> = Ok(());
            if self.feedback_is_running.load(Ordering::SeqCst) == true {
                match self.clone().try_send_feedback(FeedbackMessage::Shutdown) {
                    Ok(_) => {
                        let mut feedback_shutdown_listener = self.feedback_shutdown_listener.lock().unwrap();
                        let shutdown_listener = feedback_shutdown_listener.take().unwrap();
                        shutdown_listener.await;
                    },
                    Err(err) => { result = Err(err) },
                }
            }
        result
    }

    async fn stop(self: Arc<Self>) -> Result<()> {
        self.clone().stop_dispatch().await?;
        self.clone().stop_feedback().await
    }

}