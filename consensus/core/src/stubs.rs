use crate::block::Block;

pub enum Notification {
    BlockAdded(BlockAddedNotification),
}

pub struct BlockAddedNotification {
    pub block: Block,
}