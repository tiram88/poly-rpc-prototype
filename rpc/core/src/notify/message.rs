use crate::{Notification};
use super::listener::{
    ListenerID, ListenerSenderSide
};



pub(crate) enum DispatchMessage {
    Send(Notification),
    AddListener(ListenerID, ListenerSenderSide),
    RemoveListener(ListenerID),
    Shutdown,
}