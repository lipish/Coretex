
use async_trait::async_trait;
use std::collections::HashMap;

/// 分布式哈希环节点信息
#[derive(Clone, Debug)]
pub struct DistributionNode {
    pub id: String,
    pub weight: u64,
}

/// 数据分布策略 trait
#[async_trait]
pub trait DistributionStrategy: Send + Sync + 'static {
    /// 添加节点到分布环
    async fn add_node(&mut self, node: DistributionNode);

    /// 移除节点
    async fn remove_node(&mut self, node_id: &str);

    /// 根据 key 选择主节点
    async fn get_primary(&self, key: &[u8]) -> Option<DistributionNode>;

    /// 根据 key 选择副本节点（含主节点）
    async fn get_replicas(&self, key: &[u8], replica_count: usize) -> Vec<DistributionNode>;

    /// 获取所有节点
    fn all_nodes(&self) -> Vec<DistributionNode>;
}

/// 一致性哈希分布策略（简单实现）
pub struct ConsistentHashRing {
    nodes: HashMap<String, DistributionNode>,
    ring: Vec<String>,
}

impl ConsistentHashRing {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            ring: Vec::new(),
        }
    }
}

#[async_trait]
impl DistributionStrategy for ConsistentHashRing {
    async fn add_node(&mut self, node: DistributionNode) {
        self.ring.push(node.id.clone());
        self.nodes.insert(node.id.clone(), node);
        self.ring.sort(); // 简单排序，实际可用 hash 排序
    }

    async fn remove_node(&mut self, node_id: &str) {
        self.ring.retain(|id| id != node_id);
        self.nodes.remove(node_id);
    }

    async fn get_primary(&self, key: &[u8]) -> Option<DistributionNode> {
        if self.ring.is_empty() {
            return None;
        }
        let hash = fxhash::hash64(key);
        let idx = (hash as usize) % self.ring.len();
        let node_id = &self.ring[idx];
        self.nodes.get(node_id).cloned()
    }

    async fn get_replicas(&self, key: &[u8], replica_count: usize) -> Vec<DistributionNode> {
        let mut result = Vec::new();
        if self.ring.is_empty() {
            return result;
        }
        let hash = fxhash::hash64(key);
        let mut idx = (hash as usize) % self.ring.len();
        for _ in 0..replica_count {
            let node_id = &self.ring[idx % self.ring.len()];
            if let Some(node) = self.nodes.get(node_id) {
                result.push(node.clone());
            }
            idx += 1;
        }
        result
    }

    fn all_nodes(&self) -> Vec<DistributionNode> {
        self.ring
            .iter()
            .filter_map(|id| self.nodes.get(id).cloned())
            .collect()
    }
}

// 简单 hash 函数依赖
mod fxhash {
    pub fn hash64(data: &[u8]) -> u64 {
        use std::hash::Hasher;
        use std::collections::hash_map::DefaultHasher;
        let mut hasher = DefaultHasher::new();
        hasher.write(data);
        hasher.finish()
    }
}
