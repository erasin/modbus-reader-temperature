use etcetera::{choose_app_strategy, AppStrategy, AppStrategyArgs};
use redb::{Database, TableDefinition};
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

pub mod config;
pub mod data;

pub fn get_db_conn() -> &'static Database {
    static DB: OnceLock<Box<Database>> = OnceLock::new();
    DB.get_or_init(|| init_db().unwrap())
}

// 定义表结构
const DATA_TABLE: TableDefinition<u64, String> = TableDefinition::new("test");

fn init_db() -> Result<Box<Database>, redb::Error> {
    let db_file = data_dir().join("data.db");
    let db = Database::create(db_file)?;
    let write_txn = db.begin_write()?;
    {
        let mut _table = write_txn.open_table(DATA_TABLE)?;
        // 初始数据插入逻辑
    }
    write_txn.commit()?;
    Ok(Box::new(db))
}

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
