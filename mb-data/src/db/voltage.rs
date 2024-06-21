use mb::voltage::VoltageData;
use redb::{Database, ReadableTable, TableDefinition};
use time::ext::NumericalDuration;

use crate::{error::Error, utils::time_now};

const TABLE: TableDefinition<u64, [f32; 30]> = TableDefinition::new("data");

pub fn insert_data(db: &Database, dur: u64, data: VoltageData) -> Result<(), Error> {
    // let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    // let minute_index = (timestamp / 60) % 30;
    let data = data.into();

    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(TABLE)?;
        table.insert(&dur, &data)?;
    }
    write_txn.commit()?;
    Ok(())
}

pub fn clean_old_data(db: &Database) -> Result<(), Error> {
    let cutoff = time_now()
        .checked_sub((-15).days())
        .unwrap()
        .unix_timestamp() as u64;

    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(TABLE)?;

        let keys: Vec<u64> = table
            .iter()?
            .filter_map(|data| match data {
                Ok((dur, _)) => {
                    if dur.value() < cutoff {
                        Some(dur.value())
                    } else {
                        None
                    }
                }
                Err(_) => None,
            })
            .collect();

        keys.iter().for_each(|dur| {
            let _ = table.remove(dur);
        });
    }
    write_txn.commit()?;
    Ok(())
}

pub fn get_value(db: &Database, dur: u64) -> Result<VoltageData, Error> {
    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(TABLE)?;

    if let Some(data) = table.get(dur)? {
        let data = data.value();
        let mut voltage: VoltageData = data.into();
        voltage.time = dur;

        return Ok(voltage);
    }

    Err(Error::DbNone)
}

pub fn get_last(db: &Database) -> Result<VoltageData, Error> {
    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(TABLE)?;

    if let Some((dur, data)) = table.last()? {
        let mut voltage: VoltageData = data.value().into();
        voltage.time = dur.value();

        return Ok(voltage);
    }

    Err(Error::DbNone)
}

#[cfg(test)]
mod test {

    use crate::db::get_db;

    use super::get_last;

    #[test]
    fn test_get() {
        let db = get_db().lock().unwrap();
        let data = get_last(&db);
        match data {
            Ok(data) => {
                println!("{:?}", data.data);
                assert_eq!(15, data.data.len());
            }
            Err(e) => eprintln!("[x] --- {e}"),
        }
    }
}
