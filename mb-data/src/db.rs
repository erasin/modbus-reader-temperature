use std::sync::{Mutex, OnceLock};

use mb::Result;
use redb::{Database, TableDefinition};

pub mod config;
pub mod voltage;

use crate::dirs::data_dir;

pub fn get_db() -> &'static Mutex<Database> {
    log::debug!("------ log db------------");
    static DB: OnceLock<Mutex<Database>> = OnceLock::new();
    DB.get_or_init(|| init_db().unwrap())
}

// 定义表结构
const DATA_TABLE: TableDefinition<u64, String> = TableDefinition::new("test");

fn init_db() -> Result<Mutex<Database>> {
    log::debug!("------ log db------------2");
    let db_file = data_dir().join("data.db");
    let db = Database::create(db_file)?;
    let write_txn = db.begin_write()?;
    {
        let mut _table = write_txn.open_table(DATA_TABLE)?;
        // 初始数据插入逻辑
    }
    write_txn.commit()?;
    Ok(Mutex::new(db))
}
