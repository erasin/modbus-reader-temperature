use std::sync::OnceLock;

use time::{format_description, OffsetDateTime, UtcOffset};

pub fn get_time_offset() -> &'static UtcOffset {
    static TIME_OFFSET: OnceLock<UtcOffset> = OnceLock::new();

    TIME_OFFSET.get_or_init(|| match OffsetDateTime::now_local() {
        Ok(t) => t.offset(),
        Err(_) => UtcOffset::from_hms(8, 0, 0).unwrap(),
    })
}

pub fn time_format_human(unix_time: i64, s: &str) -> Result<String, Box<dyn std::error::Error>> {
    let format = format_description::parse(s)?;

    let s = OffsetDateTime::from_unix_timestamp(unix_time)?
        .to_offset(*get_time_offset())
        .format(&format)?;

    Ok(s)
}

pub fn time_now() -> OffsetDateTime {
    OffsetDateTime::now_utc().to_offset(*get_time_offset())
}

pub fn time_dur_odt(dur: std::time::Duration) -> OffsetDateTime {
    OffsetDateTime::from_unix_timestamp(dur.as_secs() as i64)
        .unwrap()
        .to_offset(*get_time_offset())
}

pub fn time_human(dur: OffsetDateTime) -> String {
    let format = "[year]-[month]-[day] [hour]:[minute]:[second]";
    let format = format_description::parse(format).unwrap();

    dur.to_offset(*get_time_offset()).format(&format).unwrap()
}
