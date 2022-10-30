use std::{ops::{Index, IndexMut}};

use crate::{Notification, NotificationType};

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum EventType {
    BlockAdded = 0,
    VirtualSelectedParentChainChanged,
    FinalityConflicts,
    UtxosChanged,
    VirtualSelectedParentBlueScoreChanged,
    DaaScoreChanged,
    PruningPointUTXOSetOverride,
    NewBlockTemplate,
}
pub(crate) const EVENT_COUNT: usize = 8;

pub(crate) const EVENT_TYPE_ARRAY: [EventType; EVENT_COUNT] = [
    EventType::BlockAdded,
    EventType::VirtualSelectedParentChainChanged,
    EventType::FinalityConflicts,
    EventType::UtxosChanged,
    EventType::VirtualSelectedParentBlueScoreChanged,
    EventType::DaaScoreChanged,
    EventType::PruningPointUTXOSetOverride,
    EventType::NewBlockTemplate,
];

#[derive(Default, Clone, Copy, Debug)]
pub(crate) struct EventArray<T> ([T; EVENT_COUNT]);

impl<T> Index<EventType> for EventArray<T> {
    type Output = T;

    fn index(&self, index: EventType) -> &Self::Output {
        let idx = index as usize;
        &self.0[idx]
    }
}

impl<T> IndexMut<EventType> for EventArray<T> {

    fn index_mut(&mut self, index: EventType) -> &mut Self::Output {
        let idx = index as usize;
        &mut self.0[idx]
    }
}

impl From<&Notification> for EventType {
    fn from(item: &Notification) -> Self {
        match item {
            Notification::BlockAdded(_) => EventType::BlockAdded,
        }
    }
}

impl From<&NotificationType> for EventType {
    fn from(item: &NotificationType) -> Self {
        match item {
            NotificationType::BlockAdded => EventType::BlockAdded,
            NotificationType::VirtualSelectedParentChainChanged => EventType::VirtualSelectedParentBlueScoreChanged,
            NotificationType::FinalityConflicts => EventType::FinalityConflicts,
            NotificationType::UtxosChanged(_) => EventType::UtxosChanged,
            NotificationType::VirtualSelectedParentBlueScoreChanged => EventType::VirtualSelectedParentBlueScoreChanged,
            NotificationType::DaaScoreChanged => EventType::DaaScoreChanged,
            NotificationType::PruningPointUTXOSetOverride => EventType::PruningPointUTXOSetOverride,
            NotificationType::NewBlockTemplate => EventType::NewBlockTemplate,
        }
    }
}