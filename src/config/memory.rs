
use super::{ConfigProvider, Config, ConfigChange};
use crate::Result;
use async_trait::async_trait;
use futures::{Stream, stream};
use std::pin::Pin;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct InMemoryConfigProvider {
    config: Arc<Mutex<Config>>,
}

impl InMemoryConfigProvider {
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(Mutex::new(config)),
        }
    }

    /// 允许动态更新配置（仅用于测试或特殊场景）
    pub fn set_config(&self, new_config: Config) {
        let mut cfg = self.config.lock().unwrap();
        *cfg = new_config;
    }
}

#[async_trait]
impl ConfigProvider for InMemoryConfigProvider {
    async fn get_config(&self) -> Result<Config> {
        let cfg = self.config.lock().unwrap();
        Ok(cfg.clone())
    }

    async fn watch_config(
        &self,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<ConfigChange>> + Send>>> {
        // 内存实现不支持真正的 watch，直接返回空流
        Ok(Box::pin(stream::empty()))
    }
}
