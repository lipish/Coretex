use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("存储引擎错误: {0}")]
    Storage(String),

    #[error("通信错误: {0}")]
    Communication(String),

    #[error("一致性错误: {0}")]
    Consistency(String),

    #[error("节点成员错误: {0}")]
    Membership(String),

    #[error("配置错误: {0}")]
    Configuration(String),
}
