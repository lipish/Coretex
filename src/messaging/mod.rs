
use async_trait::async_trait;
use bytes::Bytes;
use futures::Stream;
use std::pin::Pin;

use crate::Result;

/// 消息结构体
#[derive(Clone, Debug)]
pub struct Message {
    pub topic: String,
    pub data: Bytes,
    pub sender: Option<String>,
}

/// 消息事件
#[derive(Clone, Debug)]
pub enum MessagingEvent {
    MessageReceived(Message),
    Subscribed(String),
    Unsubscribed(String),
}

/// 消息通信 trait
#[async_trait]
pub trait MessageBroker: Send + Sync + 'static {
    /// 发布消息到指定 topic
    async fn publish(&self, topic: &str, data: Vec<u8>) -> Result<()>;

    /// 订阅 topic，返回消息流
    async fn subscribe(
        &self,
        topic: &str,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Message>> + Send>>>;

    /// 取消订阅
    async fn unsubscribe(&self, topic: &str) -> Result<()>;

    /// 获取已订阅的 topic 列表
    async fn subscribed_topics(&self) -> Result<Vec<String>>;
}

/// 内存实现（可选，便于测试/单机）
pub mod memory {
    use super::*;
    use dashmap::DashMap;
    use futures::stream;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};

    #[derive(Clone)]
    pub struct InMemoryBroker {
        topics: Arc<DashMap<String, Vec<UnboundedSender<Result<Message>>>>>,
        subscriptions: Arc<Mutex<HashMap<String, Vec<String>>>>, // subscriber_id -> topics
        id: String,
    }

    impl InMemoryBroker {
        pub fn new(id: impl Into<String>) -> Self {
            Self {
                topics: Arc::new(DashMap::new()),
                subscriptions: Arc::new(Mutex::new(HashMap::new())),
                id: id.into(),
            }
        }
    }

    #[async_trait]
    impl MessageBroker for InMemoryBroker {
        async fn publish(&self, topic: &str, data: Vec<u8>) -> Result<()> {
            let msg = Message {
                topic: topic.to_string(),
                data: Bytes::from(data),
                sender: Some(self.id.clone()),
            };
            if let Some(subs) = self.topics.get(topic) {
                for tx in subs.iter() {
                    let _ = tx.send(Ok(msg.clone()));
                }
            }
            Ok(())
        }

        async fn subscribe(
            &self,
            topic: &str,
        ) -> Result<Pin<Box<dyn Stream<Item = Result<Message>> + Send>>> {
            let (tx, rx) = unbounded_channel();
            self.topics.entry(topic.to_string()).or_default().push(tx);
            {
                let mut subs = self.subscriptions.lock().unwrap();
                subs.entry(self.id.clone())
                    .or_default()
                    .push(topic.to_string());
            }
            Ok(Box::pin(stream::unfold(rx, |mut rx| async {
                rx.recv().await.map(|evt| (evt, rx))
            })))
        }

        async fn unsubscribe(&self, topic: &str) -> Result<()> {
            // 简单实现：不移除具体 sender，仅移除订阅记录
            let mut subs = self.subscriptions.lock().unwrap();
            if let Some(topics) = subs.get_mut(&self.id) {
                topics.retain(|t| t != topic);
            }
            Ok(())
        }

        async fn subscribed_topics(&self) -> Result<Vec<String>> {
            let subs = self.subscriptions.lock().unwrap();
            Ok(subs.get(&self.id).cloned().unwrap_or_default())
        }
    }
}
