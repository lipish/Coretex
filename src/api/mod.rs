
use async_trait::async_trait;
use bytes::Bytes;
use crate::Result;

/// 客户端 API trait
#[async_trait]
pub trait ClientApi: Send + Sync + 'static {
    async fn get(&self, key: &[u8]) -> Result<Option<Bytes>>;
    async fn put(&self, key: &[u8], value: &[u8]) -> Result<()>;
    async fn delete(&self, key: &[u8]) -> Result<()>;
    async fn batch_put(&self, items: Vec<(Bytes, Bytes)>) -> Result<()>;
}

/// 一个简单的本地客户端实现（直接调用存储引擎）
use crate::storage::StorageEngine;
use std::sync::Arc;

pub struct LocalClient {
    storage: Arc<dyn StorageEngine>,
}

impl LocalClient {
    pub fn new(storage: Arc<dyn StorageEngine>) -> Self {
        Self { storage }
    }
}

#[async_trait]
impl ClientApi for LocalClient {
    async fn get(&self, key: &[u8]) -> Result<Option<Bytes>> {
        self.storage.get(key).await
    }

    async fn put(&self, key: &[u8], value: &[u8]) -> Result<()> {
        self.storage.put(key, value).await
    }

    async fn delete(&self, key: &[u8]) -> Result<()> {
        self.storage.delete(key).await
    }

    async fn batch_put(&self, items: Vec<(Bytes, Bytes)>) -> Result<()> {
        use crate::storage::WriteOperation;
        let ops = items
            .into_iter()
            .map(|(k, v)| WriteOperation::Put { key: k, value: v })
            .collect();
        self.storage.batch_write(ops).await
    }
}
