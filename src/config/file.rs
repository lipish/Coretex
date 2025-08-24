
use super::{ConfigProvider, Config, ConfigChange};
use crate::Result;
use async_trait::async_trait;
use futures::{Stream, stream};
use std::fs;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;

pub struct FileConfigProvider {
    path: PathBuf,
    last_content: Arc<Mutex<Option<String>>>,
}

impl FileConfigProvider {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            last_content: Arc::new(Mutex::new(None)),
        }
    }

    fn read_config_file(&self) -> Result<String> {
        let content = fs::read_to_string(&self.path)
            .map_err(|e| crate::error::Error::Configuration(format!("读取配置文件失败: {}", e)))?;
        Ok(content)
    }

    fn parse_config(&self, content: &str) -> Result<Config> {
        let config: Config = toml::from_str(content)
            .map_err(|e| crate::error::Error::Configuration(format!("解析 TOML 配置失败: {}", e)))?;
        Ok(config)
    }
}

#[async_trait]
impl ConfigProvider for FileConfigProvider {
    async fn get_config(&self) -> Result<Config> {
        let content = self.read_config_file()?;
        self.parse_config(&content)
    }

    async fn watch_config(
        &self,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<ConfigChange>> + Send>>> {
        let path = self.path.clone();
        let last_content = self.last_content.clone();

        // 轮询文件变化（简单实现，生产环境可用 inotify 等）
        let stream = stream::unfold((), move |_| {
            let path = path.clone();
            let last_content = last_content.clone();
            async move {
                sleep(Duration::from_secs(2)).await;
                let content = fs::read_to_string(&path).ok();
                let mut last = last_content.lock().unwrap();
                if let Some(ref content) = content {
                    if last.as_ref() != Some(content) {
                        *last = Some(content.clone());
                        // 尝试解析
                        if let Ok(config) = toml::from_str::<Config>(content) {
                            return Some((Ok(ConfigChange::Full(config)), ()));
                        }
                    }
                }
                Some((Err(crate::error::Error::Configuration("无变化或解析失败".to_string())), ()))
            }
        });
        Ok(Box::pin(stream))
    }
}
