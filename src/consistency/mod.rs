
use async_trait::async_trait;
use bytes::Bytes;
use crate::Result;

/// 一致性管理事件
#[derive(Debug, Clone)]
pub enum ConsistencyEvent {
    WriteCommitted { key: Bytes, value: Bytes },
    WriteConflict { key: Bytes, old: Bytes, new: Bytes },
    ReadRepair { key: Bytes, repaired_value: Bytes },
}

/// 一致性管理 trait
#[async_trait]
pub trait ConsistencyManager: Send + Sync + 'static {
    /// 写入数据，返回是否成功
    async fn put(&self, key: &[u8], value: &[u8]) -> Result<()>;

    /// 读取数据，返回值
    async fn get(&self, key: &[u8]) -> Result<Option<Bytes>>;

    /// 删除数据
    async fn delete(&self, key: &[u8]) -> Result<()>;

    /// 触发读修复
    async fn read_repair(&self, key: &[u8]) -> Result<()>;

    /// 监听一致性事件（如冲突、修复等）
    async fn watch_events(
        &self,
    ) -> Result<futures::stream::BoxStream<'static, ConsistencyEvent>>;
}

/// 一个简单的占位实现（可用于测试）
pub struct DummyConsistencyManager;

#[async_trait]
impl ConsistencyManager for DummyConsistencyManager {
    async fn put(&self, _key: &[u8], _value: &[u8]) -> Result<()> {
        Ok(())
    }
    async fn get(&self, _key: &[u8]) -> Result<Option<Bytes>> {
        Ok(None)
    }
    async fn delete(&self, _key: &[u8]) -> Result<()> {
        Ok(())
    }
    async fn read_repair(&self, _key: &[u8]) -> Result<()> {
        Ok(())
    }
    async fn watch_events(
        &self,
    ) -> Result<futures::stream::BoxStream<'static, ConsistencyEvent>> {
        use futures::stream;
        Ok(Box::pin(stream::empty()))
    }
}
