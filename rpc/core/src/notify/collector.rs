use async_trait::async_trait;
use std::sync::Arc;
extern crate derive_more;
use derive_more::Deref;
use super::{result::Result, notifier::Notifier};

#[async_trait]
pub trait Collector {
    fn start(self: Arc<Self>) -> Result<()>;
    async fn stop(self: Arc<Self>) -> Result<()>;
    fn notifier(self: Arc<Self>) -> Arc<Notifier>;
}

/// A newtype allowing conversion from Arc<T> to Arc<Notification>.
/// See [`super::collector_from::CollectorFrom`]
#[derive(Clone, Debug, Deref)]
pub struct ArcConvert<T>(Arc<T>);

impl<T> From<Arc<T>> for ArcConvert<T> {
    fn from(item: Arc<T>) -> Self {
        ArcConvert(item)
    }
}