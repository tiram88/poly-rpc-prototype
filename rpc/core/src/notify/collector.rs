use async_trait::async_trait;
use std::sync::Arc;
use super::{result::Result, notifier::Notifier};

#[async_trait]
pub trait Collector {
    fn start(self: Arc<Self>) -> Result<()>;
    async fn stop(self: Arc<Self>) -> Result<()>;
    fn notifier(self: Arc<Self>) -> Arc<Notifier>;
}