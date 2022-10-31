use std::sync::Arc;
use crate::{Notification};
use super::listener::{
    ListenerID, ListenerSenderSide
};


#[derive(Clone, Debug, )]
pub(crate) enum DispatchMessage {
    Send(Arc<Notification>),
    AddListener(ListenerID, Arc<ListenerSenderSide>),
    RemoveListener(ListenerID),
    Shutdown,
}