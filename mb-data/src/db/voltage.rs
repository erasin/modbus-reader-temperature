use std::time::Duration;

use mb::Result;
use mb::{utils::current_timestamp, voltage::VoltageData};
use redb::{Database, ReadableTable, ReadableTableMetadata, TableDefinition};
use serde::{Deserialize, Serialize};

use crate::{error::Error, task::AB};

const TABLE_A: TableDefinition<u64, &[u8]> = TableDefinition::new("voltage_a");
const TABLE_B: TableDefinition<u64, &[u8]> = TableDefinition::new("voltage_b");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoltageDataGroup {
    pub time: Duration,
    // 产品名称
    pub ab: AB,
    pub good_name: String,
    pub task_name: String,
    pub start_at: Duration,
    pub task_age_time: Duration,
    pub temperature: f32,
    pub data: Vec<VoltageData>,
}

impl VoltageDataGroup {
    pub fn voltage(&self) -> f32 {
        let l = self.data.len();

        self.data
            .iter()
            .map(|c| c.voltage())
            .fold(0_f32, |s, a| s + a)
            / l as f32
    }

    pub fn current(&self) -> f32 {
        let l = self.data.len();

        self.data
            .iter()
            .map(|c| c.current())
            .fold(0_f32, |s, a| s + a)
            / l as f32
    }
}

/// 存储数据
/// 老化结束后读取全部数据，存储到文件，然后清理
pub struct TableVoltage;

impl TableVoltage {
    pub fn set(db: &Database, data: &VoltageDataGroup) -> Result<()> {
        let table = match data.ab {
            AB::A => TABLE_A,
            AB::B => TABLE_B,
        };

        let write_txn = db.begin_write()?;
        {
            let mut table = write_txn.open_table(table)?;
            let value = serde_json::to_vec(data)?;
            table.insert(data.time.as_secs(), value.as_slice())?;
        }
        write_txn.commit()?;

        Ok(())
    }

    pub fn get(db: &Database, key: u64, ab: AB) -> Result<VoltageDataGroup> {
        let table = match ab {
            AB::A => TABLE_A,
            AB::B => TABLE_B,
        };

        println!("0");
        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(table)?;
        println!("1");

        let query = table.get(key)?;
        let data: VoltageDataGroup = match query {
            Some(value) => serde_json::from_slice(value.value())?,
            None => return Error::DbNone.into(),
        };

        println!("2");
        Ok(data)
    }

    pub fn get_len(db: &Database, ab: AB) -> Result<u64> {
        let table = match ab {
            AB::A => TABLE_A,
            AB::B => TABLE_B,
        };

        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(table)?;

        let len = table.len()?;
        Ok(len)
    }

    pub fn get_first(db: &Database, ab: AB) -> Result<VoltageDataGroup> {
        let table = match ab {
            AB::A => TABLE_A,
            AB::B => TABLE_B,
        };

        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(table)?;

        let query = table.first()?;
        let data = match query {
            Some((_key, value)) => {
                let data: VoltageDataGroup = serde_json::from_slice(value.value())?;
                data
            }
            None => return Error::DbNone.into(),
        };

        Ok(data)
    }

    pub fn get_last(db: &Database, ab: AB) -> Result<VoltageDataGroup> {
        let table = match ab {
            AB::A => TABLE_A,
            AB::B => TABLE_B,
        };

        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(table)?;

        let query = table.last()?;
        let data = match query {
            Some((_key, value)) => {
                let data: VoltageDataGroup = serde_json::from_slice(value.value())?;
                data
            }
            None => return Error::DbNone.into(),
        };

        Ok(data)
    }

    pub fn delete(db: &Database, key: u64, ab: AB) -> Result<()> {
        let table = match ab {
            AB::A => TABLE_A,
            AB::B => TABLE_B,
        };

        let write_txn = db.begin_write()?;
        {
            let mut table = write_txn.open_table(table)?;
            table.remove(key)?;
        }
        write_txn.commit()?;

        Ok(())
    }

    pub fn list(db: &Database, ab: AB) -> Result<Vec<VoltageDataGroup>> {
        let table = match ab {
            AB::A => TABLE_A,
            AB::B => TABLE_B,
        };

        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(table)?;

        let mut users = Vec::new();
        for entry in table.iter()? {
            let (_, value) = entry?;
            let user: VoltageDataGroup = serde_json::from_slice(value.value())?;
            users.push(user);
        }

        Ok(users)
    }

    pub fn range_last(db: &Database, ab: AB, dur: u64) -> Result<Vec<VoltageDataGroup>> {
        let table = match ab {
            AB::A => TABLE_A,
            AB::B => TABLE_B,
        };
        let now = current_timestamp().as_secs();
        let start = now - dur;

        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(table)?;

        let mut users = Vec::new();
        for entry in table.range(start..now)? {
            let (_, value) = entry?;
            let user: VoltageDataGroup = serde_json::from_slice(value.value())?;
            users.push(user);
        }

        Ok(users)
    }

    pub fn clean(db: &Database, ab: AB) -> Result<()> {
        let table = match ab {
            AB::A => TABLE_A,
            AB::B => TABLE_B,
        };

        let mut keys = Vec::new();

        {
            let write_txn = db.begin_write()?;
            {
                let table = write_txn.open_table(table)?;
                for entry in table.iter()? {
                    let (key, _value) = entry?;
                    let key = key.value();
                    keys.push(key)
                }
            }
            write_txn.commit()?;
        }

        {
            let write_txn = db.begin_write()?;
            {
                let mut table = write_txn.open_table(table)?;
                for key in keys.iter() {
                    table.remove(key)?;
                }
            }
            write_txn.commit()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use mb::{
        utils::current_timestamp,
        voltage::{VoltageChannel, VoltageData},
    };

    use crate::{db::get_db, task::AB};

    use super::{TableVoltage, VoltageDataGroup};

    #[test]
    fn test_list() {
        let db = get_db().lock().unwrap();
        // assert_eq!((), TableVoltage::clean(&db, AB::A).unwrap());

        let dur = current_timestamp();

        let re = VoltageDataGroup {
            time: dur,
            ab: AB::A,
            good_name: "test".into(),
            task_name: "test".into(),
            start_at: dur,
            task_age_time: dur,
            temperature: 30.0,
            data: vec![VoltageData {
                time: dur,
                slave: 100,
                data: vec![
                    VoltageChannel::default(),
                    VoltageChannel::default(),
                    VoltageChannel::default(),
                ],
            }],
        };

        match TableVoltage::set(&db, &re) {
            Ok(data) => {
                println!("{:?}", data);
            }
            Err(e) => {
                eprintln!("set 失败: {}", e.to_string());
            }
        };

        match TableVoltage::get_len(&db, AB::A) {
            Ok(data) => {
                println!("{:?}", data);
            }
            Err(e) => {
                eprintln!("len 失败: {}", e.to_string());
            }
        };

        match TableVoltage::get(&db, dur.as_secs(), AB::A) {
            Ok(data) => {
                println!("{:?}", data);
            }
            Err(e) => {
                eprintln!("获取失败: {}", e.to_string());
            }
        };

        match TableVoltage::list(&db, AB::A) {
            Ok(data) => {
                println!("{:?}", data);
            }
            Err(e) => {
                eprintln!("获取失败: {}", e.to_string());
            }
        };
    }
}
