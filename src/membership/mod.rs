mod memory;
// 预留 raft 共识实现
// mod raft;

use async_trait::async_trait;
use futures::Stream;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::pin::Pin;
use std::net::SocketAddr;

use crate::Result;

pub use memory::InMemoryMembership;
// pub use raft::RaftBasedMembership;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub address: SocketAddr,
    pub state: NodeState,
    pub metadata: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeState {
    Joining,
    Active,
    Leaving,
    Down,
}

#[derive(Clone, Debug)]
pub enum MembershipEvent {
    NodeJoined(Node),
    NodeStateChanged { id: String, state: NodeState },
    NodeLeft(String),
}

#[async_trait]
pub trait MembershipManager: Send + Sync + 'static {
    async fn register_node(
        &self,
        address: SocketAddr,
        metadata: HashMap<String, String>
    ) -> Result<String>;

    async fn update_node_state(&self, node_id: &str, state: NodeState) -> Result<()>;
    async fn unregister_node(&self, node_id: &str) -> Result<()>;
    async fn get_nodes(&self) -> Result<Vec<Node>>;
    async fn get_node(&self, node_id: &str) -> Result<Option<Node>>;

    async fn watch_nodes(&self) -> Result<Pin<Box<dyn Stream<Item = Result<MembershipEvent>> + Send>>>;
}
