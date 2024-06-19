use std::time::{SystemTime, UNIX_EPOCH};

/// 获取当前时间戳（秒级）
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
