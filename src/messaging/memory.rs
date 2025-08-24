coretex/src/messaging/memory.rs
```
use super::{MessageBroker, Message};
use crate::Result;
use async_trait::async_trait;
use bytes::Bytes;
use futures::{Stream, stream};
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver, unbounded_channel};

#[derive(Clone)]
pub struct InMemoryMessaging {
    topics: Arc<Mutex<HashMap<String, Vec<UnboundedSender<Message>>>>>,
}

impl InMemoryMessaging {
    pub fn new() -> Self {
        Self {
            topics: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl MessageBroker for InMemoryMessaging {
    async fn publish(&self, topic: &str, data: Vec<u8>) -> Result<()> {
        let mut topics = self.topics.lock().unwrap();
        if let Some(subscribers) = topics.get_mut(topic) {
            let msg = Message {
                topic: topic.to_string(),
                data: Bytes::from(data),
            };
            for sub in subscribers.iter() {
                let _ = sub.send(msg.clone());
            }
        }
        Ok(())
    }

    async fn subscribe(&self, topic: &str) -> Result<Pin<Box<dyn Stream<Item = Message> + Send>>> {
        let (tx, rx): (UnboundedSender<Message>, UnboundedReceiver<Message>) = unbounded_channel();
        {
            let mut topics = self.topics.lock().unwrap();
            topics.entry(topic.to_string()).or_default().push(tx);
        }
        Ok(Box::pin(stream::unfold(rx, |mut rx| async {
            rx.recv().await.map(|msg| (msg, rx))
        })))
    }
}

// 消息结构体
#[derive(Clone, Debug)]
pub struct Message {
    pub topic: String,
    pub data: Bytes,
}
