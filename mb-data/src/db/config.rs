use mb::Result;
use redb::{Database, TableDefinition};

use crate::config::Config;

pub const TABLE: TableDefinition<String, &[u8]> = TableDefinition::new("global");

pub struct TableGlobal {}

impl TableGlobal {
    pub fn set_config(db: &Database, data: &Config) -> Result<()> {
        let write_txn = db.begin_write()?;
        {
            let mut table = write_txn.open_table(TABLE)?;
            let data = serde_json::to_vec(data)?;
            table.insert("config".to_string(), data.as_slice())?;
        }
        write_txn.commit()?;

        Ok(())
    }

    pub fn get_config(db: &Database) -> Result<Config> {
        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(TABLE)?;

        let query = table.get("config".to_string())?;
        let data = match query {
            Some(d) => {
                let c: Config = serde_json::from_slice(d.value())?;
                c
            }
            None => Config::default(),
        };

        Ok(data)
    }

    // pub fn get_config(db: &Database) -> Result<Config> {
    //     get_global("config")
    // }
}
