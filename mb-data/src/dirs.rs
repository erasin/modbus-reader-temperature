use std::{fs, path::PathBuf};

use etcetera::{choose_app_strategy, AppStrategy, AppStrategyArgs};

pub fn data_dir() -> PathBuf {
    let strategy = choose_app_strategy(AppStrategyArgs {
        top_level_domain: "com".to_string(),
        author: "erasin".to_string(),
        app_name: "mbreader".to_string(),
    })
    .unwrap();
    let data_dir = strategy.data_dir();

    if !data_dir.exists() {
        fs::create_dir(data_dir.clone()).unwrap();
    }

    data_dir
}
