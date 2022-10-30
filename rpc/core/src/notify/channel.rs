use std::sync::Arc;

use async_std::channel::{Sender, Receiver, unbounded};
use crate::Notification;

/// Multiple producers multiple consumers channel
#[derive(Clone, Debug)]
pub struct Channel<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

impl<T> Channel<T> {
    pub(crate) fn new(channel: (Sender<T>, Receiver<T>)) -> Channel<T> {
        Self {
            sender: channel.0,
            receiver: channel.1,
        }
    }

    pub(crate) fn sender(&self) ->Sender<T> {
        self.sender.clone()
    }

    pub(crate) fn receiver(&self) -> Receiver<T> {
        self.receiver.clone()
    }

    pub(crate) fn _close(&self) {
        self.receiver.close();
    }
}

/// Default for a [`Channel<T>`] is an unbounded
impl<T> Default for Channel<T> {
    fn default() -> Self {
        let ch = unbounded();
        Self { sender: ch.0, receiver: ch.1 }
    }
}

pub type NotificationChannel = Channel<Arc<Notification>>;

// #[derive(Clone, Debug)]
// pub struct NotificationChannel {
//     pub sender:NotificationSender,
//     pub receiver: NotificationReceiver,
// }

// impl NotificationChannel {
//     pub fn new(channel: (NotificationSender, NotificationReceiver)) -> NotificationChannel {
//         Self {
//             sender: channel.0,
//             receiver: channel.1,
//         }
//     }

//     pub fn sender(&self) ->NotificationSender {
//         self.sender.clone()
//     }

//     pub fn receiver(&self) -> NotificationReceiver {
//         self.receiver.clone()
//     }

//     pub fn close(&self) {
//         self.receiver.close();
//     }
// }

