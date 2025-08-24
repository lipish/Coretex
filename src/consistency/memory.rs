coretex/src/consistency/memory.rs
```
use super::{ConsistencyManager, ConsistencyResult, VersionedValue};
use crate::Result;
use async_trait::async_trait;
use bytes::Bytes;
use dashmap::DashMap;
use std::collections::HashMap;
use std::sync::Arc;

/// 简单的内存一致性管理，仅支持最终一致性
#[derive(Clone)]
pub struct InMemoryConsistency {
    data: Arc<DashMap<Bytes, VersionedValue>>,
}

impl InMemoryConsistency {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }
}

#[async_trait]
impl ConsistencyManager for InMemoryConsistency {
    async fn get(&self, key: &[u8]) -> Result<Option<VersionedValue>> {
        Ok(self.data.get(key).map(|v| v.value().clone()))
    }

    async fn put(&self, key: &[u8], value: Bytes, version: u64, context: Option<HashMap<String, Bytes>>) -> Result<ConsistencyResult> {
        let vv = VersionedValue {
            value,
            version,
            context,
        };
        self.data.insert(Bytes::copy_from_slice(key), vv.clone());
        Ok(ConsistencyResult::Stored(vv))
    }

    async fn delete(&self, key: &[u8]) -> Result<()> {
        self.data.remove(key);
        Ok(())
    }

    async fn resolve_conflict(&self, key: &[u8], candidates: Vec<VersionedValue>) -> Result<VersionedValue> {
        // 简单策略：选择 version 最大的
        let resolved = candidates.into_iter().max_by_key(|v| v.version)
            .ok_or_else(|| crate::error::Error::Consistency("No candidates for conflict resolution".into()))?;
        self.data.insert(Bytes::copy_from_slice(key), resolved.clone());
        Ok(resolved)
    }
}
