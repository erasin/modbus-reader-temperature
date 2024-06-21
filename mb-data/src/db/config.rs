use redb::{Database, TableDefinition};

use crate::{config::Config, error::Error};

use super::get_db;

pub const TABLE: TableDefinition<String, String> = TableDefinition::new("user");

pub fn set_config(db: &Database, data: Config) -> Result<(), Error> {
    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(TABLE)?;

        table.insert("port".to_string(), &data.port)?;
        table.insert("baudrate".to_string(), &data.baudrate.to_string())?;
    }
    write_txn.commit()?;

    Ok(())
}

pub fn get_config(db: &Database) -> Result<Config, Error> {
    let mut data = Config::default();

    let read_txn = db.begin_read()?;
    {
        let table = read_txn.open_table(TABLE)?;

        data.port = table.get("port".to_string())?.unwrap().value();
        data.baudrate = table
            .get("baudrate".to_string())?
            .unwrap()
            .value()
            .parse::<u32>()?
            .into();
    }

    Ok(data)
}
