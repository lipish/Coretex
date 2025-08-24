mod file;
mod memory;

use async_trait::async_trait;
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::pin::Pin;

use crate::Result;

pub use file::FileConfigProvider;
pub use memory::InMemoryConfigProvider;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub node: NodeConfig,
    pub storage: StorageConfig,
    pub replication: ReplicationConfig,
    pub consistency: ConsistencyConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeConfig {
    pub bind_address: SocketAddr,
    pub data_dir: PathBuf,
    pub seed_nodes: Vec<SocketAddr>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StorageConfig {
    pub engine: String,
    pub rocksdb_options: Option<HashMap<String, String>>,
    pub sled_options: Option<HashMap<String, String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReplicationConfig {
    pub factor: usize,
    pub read_quorum: usize,
    pub write_quorum: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsistencyConfig {
    pub mode: ConsistencyMode,
    pub vector_clock_enabled: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ConsistencyMode {
    Eventual,
    Strong,
    Causal,
}

#[derive(Clone, Debug)]
pub enum ConfigChange {
    Full(Config),
    Node(NodeConfig),
    Storage(StorageConfig),
    Replication(ReplicationConfig),
    Consistency(ConsistencyConfig),
}

#[async_trait]
pub trait ConfigProvider: Send + Sync + 'static {
    async fn get_config(&self) -> Result<Config>;
    async fn watch_config(
        &self,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<ConfigChange>> + Send>>>;
}
