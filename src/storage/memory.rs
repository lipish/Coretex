use crate::{Result, Error};
use super::{KeyValue, StorageEngine, WriteOperation};
use async_trait::async_trait;
use bytes::Bytes;
use dashmap::DashMap;
use futures::{Stream, stream};
use std::pin::Pin;
use std::sync::Arc;

pub struct InMemoryEngine {
    data: Arc<DashMap<Bytes, Bytes>>,
    name: String,
}

impl InMemoryEngine {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            data: Arc::new(DashMap::new()),
            name: name.into(),
        }
    }
}

#[async_trait]
impl StorageEngine for InMemoryEngine {
    async fn get(&self, key: &[u8]) -> Result<Option<Bytes>> {
        Ok(self.data.get(key).map(|val| val.value().clone()))
    }

    async fn put(&self, key: &[u8], value: &[u8]) -> Result<()> {
        self.data.insert(Bytes::copy_from_slice(key), Bytes::copy_from_slice(value));
        Ok(())
    }

    async fn delete(&self, key: &[u8]) -> Result<()> {
        self.data.remove(key);
        Ok(())
    }

    async fn scan(
        &self,
        start: &[u8],
        end: Option<&[u8]>,
        limit: Option<usize>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<KeyValue>> + Send>>> {
        let start_key = Bytes::copy_from_slice(start);
        let end_key = end.map(Bytes::copy_from_slice);

        // 收集符合条件的键值对
        let mut items = self.data
            .iter()
            .filter(|entry| {
                let key = entry.key();
                key >= &start_key && end_key.as_ref().map_or(true, |end| key < end)
            })
            .map(|entry| {
                Ok(KeyValue {
                    key: entry.key().clone(),
                    value: entry.value().clone(),
                })
            })
            .collect::<Vec<_>>();

        // 应用限制
        if let Some(limit) = limit {
            items.truncate(limit);
        }

        Ok(Box::pin(stream::iter(items)))
    }

    async fn batch_write(&self, operations: Vec<WriteOperation>) -> Result<()> {
        for op in operations {
            match op {
                WriteOperation::Put { key, value } => {
                    self.data.insert(key, value);
                }
                WriteOperation::Delete { key } => {
                    self.data.remove(&key);
                }
            }
        }
        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }
}
