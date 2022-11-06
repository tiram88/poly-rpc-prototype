use rpc_core::{
    NotificationType,
    api::ops::SubscribeCommand,
};

use crate::protowire::{
    kaspad_request, KaspadRequest,
    NotifyBlockAddedRequestMessage,
    kaspad_response, KaspadResponse, 
};

impl KaspadRequest {
    pub fn from_notification_type(notification_type: &NotificationType, command: SubscribeCommand) -> Self {
        KaspadRequest { payload: Some(kaspad_request::Payload::from_notification_type(notification_type, command)) }
    }
}

impl kaspad_request::Payload {
    // FIXME: Enhance protowire with Subscribe Commands
    pub fn from_notification_type(notification_type: &NotificationType, _command: SubscribeCommand) -> Self {
        match notification_type {
            NotificationType::BlockAdded =>
            kaspad_request::Payload::NotifyBlockAddedRequest(NotifyBlockAddedRequestMessage{}),
            NotificationType::VirtualSelectedParentChainChanged => todo!(),
            NotificationType::FinalityConflicts => todo!(),
            NotificationType::FinalityConflictResolved => todo!(),
            NotificationType::UtxosChanged(_) => todo!(),
            NotificationType::VirtualSelectedParentBlueScoreChanged => todo!(),
            NotificationType::VirtualDaaScoreChanged => todo!(),
            NotificationType::PruningPointUTXOSetOverride => todo!(),
            NotificationType::NewBlockTemplate => todo!(),
        }
    }
}

impl KaspadResponse {
    pub fn is_notification(&self) -> bool {
        match self.payload {
            Some(ref payload) => payload.is_notification(),
            None => false,
        }
    }
}

impl kaspad_response::Payload {
    pub fn is_notification(&self) -> bool {
        match self {
            kaspad_response::Payload::BlockAddedNotification(_) => true,
            _ => false
        }
    }
}