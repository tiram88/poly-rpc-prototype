use crate::protowire::{
    kaspad_response::Payload, KaspadResponse,
};


impl KaspadResponse {
    pub fn is_notification(&self) -> bool {
        match self.payload {
            Some(ref payload) => payload.is_notification(),
            None => false,
        }
    }
}

impl Payload {
    pub fn is_notification(&self) -> bool {
        match self {
            Payload::BlockAddedNotification(_) => true,
            _ => false
        }
    }
}