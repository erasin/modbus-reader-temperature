use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// 获取当前时间戳（秒级）
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn time_from_hms(hours: u64, minutes: u64, seconds: u64) -> Duration {
    let total_seconds = hours * 3600 + minutes * 60 + seconds;
    Duration::new(total_seconds, 0)
}

pub fn hms_from_duration(duration: Duration) -> (u64, u64, u64) {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    (hours, minutes, seconds)
}

pub fn hms_from_duration_string(duration: Duration) -> String {
    let (hours, minutes, seconds) = hms_from_duration(duration);
    format!("{hours}时 {minutes}分 {seconds}秒")
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

    println!("{} ({n}): {:?} \n{hex} \n", name.as_ref(), &data);
}
