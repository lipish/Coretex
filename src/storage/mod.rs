mod memory;
// 预留 rocksdb、sled 实现模块
// mod rocks;
// mod sled;

use async_trait::async_trait;
use bytes::Bytes;
use futures::Stream;
use std::pin::Pin;

pub use memory::InMemoryEngine;
// #[cfg(feature = "rocksdb")]
// pub use rocks::RocksDBEngine;
// #[cfg(feature = "sled")]
// pub use sled::SledEngine;

use crate::Result;

pub struct KeyValue {
    pub key: Bytes,
    pub value: Bytes,
}

#[async_trait]
pub trait StorageEngine: Send + Sync + 'static {
    async fn get(&self, key: &[u8]) -> Result<Option<Bytes>>;
    async fn put(&self, key: &[u8], value: &[u8]) -> Result<()>;
    async fn delete(&self, key: &[u8]) -> Result<()>;

    async fn scan(
        &self,
        start: &[u8],
        end: Option<&[u8]>,
        limit: Option<usize>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<KeyValue>> + Send>>>;

    async fn batch_write(&self, operations: Vec<WriteOperation>) -> Result<()>;

    fn name(&self) -> &str;
}

pub enum WriteOperation {
    Put { key: Bytes, value: Bytes },
    Delete { key: Bytes },
}
