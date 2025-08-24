/// 工具模块
/// 包含常用的工具函数和辅助结构

/// 简单的ID生成器
pub fn generate_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// 时间戳生成器
pub fn current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// 字节向量到字符串的转换工具
pub fn bytes_to_string(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).to_string()
}