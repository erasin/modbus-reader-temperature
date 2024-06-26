use mb::Result;
use redb::{Database, TableDefinition};

use crate::{error::Error, user::UserConfig};

pub const TABLE: TableDefinition<String, &[u8]> = TableDefinition::new("user");

pub struct TableUser {}

impl TableUser {
    pub fn set(db: &Database, data: &UserConfig) -> Result<()> {
        let write_txn = db.begin_write()?;
        {
            let mut table = write_txn.open_table(TABLE)?;
            let value = serde_json::to_vec(data)?;
            table.insert(data.name.clone(), value.as_slice())?;
        }
        write_txn.commit()?;

        Ok(())
    }

    pub fn get(db: &Database) -> Result<UserConfig> {
        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(TABLE)?;

        let query = table.get("config".to_string())?;
        let data = match query {
            Some(value) => {
                let data: UserConfig = serde_json::from_slice(value.value())?;
                data
            }
            None => return Error::DbNone.into(),
        };

        Ok(data)
    }
}
