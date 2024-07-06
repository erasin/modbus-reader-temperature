use mb::Result;
use redb::{Database, ReadableTable, TableDefinition};

use crate::{
    error::Error,
    task::{Task, AB},
};

pub const TABLE_A: TableDefinition<String, &[u8]> = TableDefinition::new("task_a");

pub const TABLE_B: TableDefinition<String, &[u8]> = TableDefinition::new("task_b");

pub struct TableTask;

impl TableTask {
    pub fn set(db: &Database, data: &Task) -> Result<()> {
        let table_ab = match data.ab {
            AB::A => TABLE_A,
            AB::B => TABLE_B,
        };

        let write_txn = db.begin_write()?;
        {
            let mut table = write_txn.open_table(table_ab)?;
            let value = serde_json::to_vec(data)?;
            table.insert(data.title.clone(), value.as_slice())?;
        }
        write_txn.commit()?;

        Ok(())
    }

    pub fn get<T: Into<String>>(db: &Database, key: T, ab: &AB) -> Result<Task> {
        let table_ab = match ab {
            AB::A => TABLE_A,
            AB::B => TABLE_B,
        };

        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(table_ab)?;

        let query = table.get(key.into())?;
        let data = match query {
            Some(value) => {
                let data: Task = serde_json::from_slice(value.value())?;
                data
            }
            None => return Error::DbNone.into(),
        };

        Ok(data)
    }

    pub fn delete<T: Into<String>>(db: &Database, key: T, ab: &AB) -> Result<()> {
        let table_ab = match ab {
            AB::A => TABLE_A,
            AB::B => TABLE_B,
        };

        let write_txn = db.begin_write()?;
        {
            let mut table = write_txn.open_table(table_ab)?;
            table.remove(key.into())?;
        }
        write_txn.commit()?;

        Ok(())
    }

    pub fn list(db: &Database, ab: &AB) -> Result<Vec<Task>> {
        let table_ab = match ab {
            AB::A => TABLE_A,
            AB::B => TABLE_B,
        };

        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(table_ab)?;

        let mut users = Vec::new();
        for entry in table.iter()? {
            let (_, value) = entry?;
            let user: Task = serde_json::from_slice(value.value())?;
            users.push(user);
        }

        Ok(users)
    }
}
