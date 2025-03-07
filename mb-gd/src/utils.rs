use mb::Result;
use mb_data::{
    dirs,
    utils::{time_human, time_now},
};

/// 日志
pub fn init_logging(verbosity: u64) -> Result<()> {
    let mut base_config = fern::Dispatch::new();

    base_config = match verbosity {
        0 => base_config.level(log::LevelFilter::Warn),
        1 => base_config.level(log::LevelFilter::Info),
        2 => base_config.level(log::LevelFilter::Debug),
        _3_or_more => base_config.level(log::LevelFilter::Trace),
    };

    let file_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} {} [{}] {}",
                time_human(time_now()),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(std::io::stdout())
        .chain(fern::log_file(dirs::log_file())?);

    base_config.chain(file_config).apply()?;

    Ok(())
}

pub fn string_number_only(text: String) -> String {
    text.chars().filter(|c| c.is_ascii_digit()).collect()
}

pub fn string_cut(input: &str, length: usize) -> String {
    let mut result = input.to_string();

    match result.len().cmp(&length) {
        std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
            result.push_str(&" ".repeat(length - result.len()));
        }
        std::cmp::Ordering::Greater => {
            result.truncate(length - 3); // Leave space for "..."
            result.push_str("...");
        }
    }

    result
}
