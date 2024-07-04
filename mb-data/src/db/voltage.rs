use mb::voltage::VoltageData;
use mb::Result;
use redb::{Database, ReadableTable, TableDefinition};

use crate::error::Error;

const TABLE: TableDefinition<String, &[u8]> = TableDefinition::new("voltage");

pub struct TableVoltage;

impl TableVoltage {
    pub fn set<T: Into<String>>(db: &Database, key: T, data: &VoltageData) -> Result<()> {
        let write_txn = db.begin_write()?;
        {
            let mut table = write_txn.open_table(TABLE)?;
            let value = serde_json::to_vec(data)?;
            table.insert(key.into(), value.as_slice())?;
        }
        write_txn.commit()?;

        Ok(())
    }

    pub fn get<T: Into<String>>(db: &Database, key: T) -> Result<VoltageData> {
        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(TABLE)?;

        let query = table.get(key.into())?;
        let data = match query {
            Some(value) => {
                let data: VoltageData = serde_json::from_slice(value.value())?;
                data
            }
            None => return Error::DbNone.into(),
        };

        Ok(data)
    }

    pub fn delete<T: Into<String>>(db: &Database, key: T) -> Result<()> {
        let write_txn = db.begin_write()?;
        {
            let mut table = write_txn.open_table(TABLE)?;
            table.remove(key.into())?;
        }
        write_txn.commit()?;

        Ok(())
    }

    pub fn list(db: &Database) -> Result<Vec<VoltageData>> {
        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(TABLE)?;

        let mut users = Vec::new();
        for entry in table.iter()? {
            let (_, value) = entry?;
            let user: VoltageData = serde_json::from_slice(value.value())?;
            users.push(user);
        }

        Ok(users)
    }
}

// pub fn clean_old_data(db: &Database) -> Result<()> {
//     let cutoff = time_now()
//         .checked_sub((-15).days())
//         .unwrap()
//         .unix_timestamp() as u64;

//     let write_txn = db.begin_write()?;
//     {
//         let mut table = write_txn.open_table(TABLE)?;

//         let keys: Vec<u64> = table
//             .iter()?
//             .filter_map(|data| match data {
//                 Ok((dur, _)) => {
//                     if dur.value() < cutoff {
//                         Some(dur.value())
//                     } else {
//                         None
//                     }
//                 }
//                 Err(_) => None,
//             })
//             .collect();

//         keys.iter().for_each(|dur| {
//             let _ = table.remove(dur);
//         });
//     }
//     write_txn.commit()?;
//     Ok(())
// }

// pub fn get_value(db: &Database, dur: u64) -> Result<VoltageData> {
//     let read_txn = db.begin_read()?;
//     let table = read_txn.open_table(TABLE)?;

//     if let Some(data) = table.get(dur)? {
//         let data = data.value();
//         let mut voltage: VoltageData = data.into();
//         voltage.time = dur;

//         return Ok(voltage);
//     }

//     Err(Error::DbNone)
// }

// pub fn get_last(db: &Database) -> Result<VoltageData, Error> {
//     let read_txn = db.begin_read()?;
//     let table = read_txn.open_table(TABLE)?;

//     if let Some((dur, data)) = table.last()? {
//         let mut voltage: VoltageData = data.value().into();
//         voltage.time = dur.value();

//         return Ok(voltage);
//     }

//     Err(Error::DbNone)
// }

#[cfg(test)]
mod test {
    // use crate::db::get_db;

    // #[test]
    // fn test_get() {
    //     let db = get_db().lock().unwrap();
    //     let data = get_last(&db);
    //     match data {
    //         Ok(data) => {
    //             println!("{:?}", data.data);
    //             assert_eq!(15, data.data.len());
    //         }
    //         Err(e) => eprintln!("[x] --- {e}"),
    //     }
    // }
}
