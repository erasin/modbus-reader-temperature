use std::time::{SystemTime, UNIX_EPOCH};

/// 获取当前时间戳（秒级）
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn print_hex<T: AsRef<str>>(name: T, data: &Vec<u8>) {
    let n = data.len();
    let hex = data
        .iter()
        .map(|v| format!("{:02X} ", v))
        .fold(String::new(), |mut x, y| {
            x.push_str(&y);
            x
        });

    log::debug!("{} ({n}): {:?} \n{hex} \n", name.as_ref(), &data);
}
