pub mod api;
pub mod config;
pub mod consistency;
pub mod distribution;
pub mod error;
pub mod membership;
pub mod messaging;
pub mod storage;
pub mod utils;

use std::sync::Arc;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

/// Coretex 系统实例
pub struct Coretex {
    pub config: Arc<config::Config>,
    pub storage: Arc<dyn storage::StorageEngine>,
    pub membership: Arc<dyn membership::MembershipManager>,
    pub messaging: Arc<dyn messaging::MessageBroker>,
    pub consistency: Arc<dyn consistency::ConsistencyManager>,
}

pub async fn start(_config_path: &str) -> Result<Coretex> {
    // 初始化系统的实现将在这里
    unimplemented!()
}
