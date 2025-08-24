use super::{MembershipManager, Node, NodeState, MembershipEvent};
use crate::Result;
use async_trait::async_trait;
use dashmap::DashMap;
use futures::{Stream, stream};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone)]
pub struct InMemoryMembership {
    nodes: Arc<DashMap<String, Node>>,
    event_subscribers: Arc<Mutex<Vec<tokio::sync::mpsc::UnboundedSender<Result<MembershipEvent>>>>>,
}

impl InMemoryMembership {
    pub fn new() -> Self {
        Self {
            nodes: Arc::new(DashMap::new()),
            event_subscribers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn notify(&self, event: MembershipEvent) {
        let subscribers = self.event_subscribers.lock().unwrap();
        for tx in subscribers.iter() {
            let _ = tx.send(Ok(event.clone()));
        }
    }
}

#[async_trait]
impl MembershipManager for InMemoryMembership {
    async fn register_node(
        &self,
        address: SocketAddr,
        metadata: HashMap<String, String>,
    ) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        let node = Node {
            id: id.clone(),
            address,
            state: NodeState::Joining,
            metadata,
        };
        self.nodes.insert(id.clone(), node.clone());
        self.notify(MembershipEvent::NodeJoined(node));
        Ok(id)
    }

    async fn update_node_state(&self, node_id: &str, state: NodeState) -> Result<()> {
        if let Some(mut node) = self.nodes.get_mut(node_id) {
            node.state = state.clone();
            self.notify(MembershipEvent::NodeStateChanged {
                id: node_id.to_string(),
                state,
            });
            Ok(())
        } else {
            Err(crate::error::Error::Membership(format!(
                "Node {} not found",
                node_id
            )))
        }
    }

    async fn unregister_node(&self, node_id: &str) -> Result<()> {
        if self.nodes.remove(node_id).is_some() {
            self.notify(MembershipEvent::NodeLeft(node_id.to_string()));
            Ok(())
        } else {
            Err(crate::error::Error::Membership(format!(
                "Node {} not found",
                node_id
            )))
        }
    }

    async fn get_nodes(&self) -> Result<Vec<Node>> {
        Ok(self.nodes.iter().map(|entry| entry.value().clone()).collect())
    }

    async fn get_node(&self, node_id: &str) -> Result<Option<Node>> {
        Ok(self.nodes.get(node_id).map(|entry| entry.value().clone()))
    }

    async fn watch_nodes(
        &self,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<MembershipEvent>> + Send>>> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        {
            let mut subscribers = self.event_subscribers.lock().unwrap();
            subscribers.push(tx);
        }
        Ok(Box::pin(
            stream::unfold(rx, |mut rx| async {
                rx.recv().await.map(|evt| (evt, rx))
            }),
        ))
    }
}
