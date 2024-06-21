use std::{fs, path::PathBuf};

use etcetera::{app_strategy::Xdg, choose_app_strategy, AppStrategy, AppStrategyArgs};
use mb::Result;

fn base_dir() -> Result<Xdg> {
    Ok(choose_app_strategy(AppStrategyArgs {
        top_level_domain: "com".to_string(),
        author: "erasin".to_string(),
        app_name: "mbreader".to_string(),
    })?)
}

pub fn data_dir() -> PathBuf {
    let strategy = base_dir().unwrap();
    let data_dir = strategy.data_dir();
    if !data_dir.exists() {
        fs::create_dir(data_dir.clone()).unwrap();
    }
    data_dir
}

pub fn log_file() -> PathBuf {
    let strategy = base_dir().unwrap();
    let cache_dir = strategy.cache_dir();
    if !cache_dir.exists() {
        fs::create_dir(cache_dir.clone()).unwrap();
    }
    cache_dir.join("mbreader.log")
}
