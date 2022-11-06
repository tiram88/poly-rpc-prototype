use std::sync::Arc;
use crate::{Notification, NotificationType};
use super::{
    listener::{
        ListenerID,
        ListenerSenderSide
    },
};


#[derive(Clone, Debug)]
pub(crate) enum DispatchMessage {
    Send(Arc<Notification>),
    AddListener(ListenerID, Arc<ListenerSenderSide>),
    RemoveListener(ListenerID),
    Shutdown,
}

#[derive(Clone, Debug)]
pub(crate) enum SubscribeMessage {
    StartEvent(NotificationType),
    StopEvent(NotificationType),
    Shutdown,
}