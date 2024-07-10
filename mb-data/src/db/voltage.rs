use std::collections::HashMap;
use std::ops::Sub;
use std::time::Duration;

use mb::voltage::{Verify, VoltageChannel, VoltageState};
use mb::Result;
use mb::{utils::current_timestamp, voltage::VoltageData};
use redb::{Database, ReadableTable, ReadableTableMetadata, TableDefinition};
use serde::{Deserialize, Serialize};

use crate::utils::time_dur_odt;
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
        // log::debug!("start: {} --> end: {}", start, now);

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

// 用来存储打印
#[derive(Debug, Clone, Copy)]
pub struct VoltageChannelItem {
    pub index: usize,
    pub time: Duration,
    pub ch: VoltageChannel,
}

/// 获取 n 秒内的良品率
pub fn check_defective_in_secs(
    groups: Vec<VoltageDataGroup>,
    secs: u64,
    verify: &Verify,
) -> Vec<VoltageChannel> {
    let after = current_timestamp().as_secs() - secs;
    //存储渠道长度
    let mut channel_len: usize = 0;
    // 转化列表
    let list = groups
        .iter()
        .enumerate()
        .flat_map(|(index_group, group)| {
            let group_len = group.data.len();
            group
                .data
                .iter()
                .flat_map(|slave| {
                    let ch_len = slave.data.len() * group_len;
                    channel_len = channel_len.max(ch_len);
                    slave
                        .data
                        .iter()
                        .map(|c| VoltageChannelItem {
                            index: index_group * ch_len + c.index,
                            time: group.time,
                            ch: *c,
                        })
                        .collect::<Vec<VoltageChannelItem>>()
                })
                .collect::<Vec<VoltageChannelItem>>()
        })
        .collect::<Vec<VoltageChannelItem>>();

    // 按照渠道分组
    let channels = (0..channel_len)
        .map(|index| {
            let chs = list
                .iter()
                // 获取同渠道，时间为 n 秒内的 数据
                .filter(|&item| item.ch.index == index && item.time.as_secs() >= after)
                .cloned()
                .collect::<Vec<VoltageChannelItem>>();
            if chs.len() as u64 >= secs {
                chs
            } else {
                Vec::new()
            }
        })
        .collect::<Vec<Vec<VoltageChannelItem>>>();

    // 合并数据
    channels
        .iter()
        .map(|chs| {
            let l = chs.len();

            let mut item = chs.iter().fold(
                VoltageChannelItem {
                    index: 0,
                    time: Duration::from_secs(0),
                    ch: VoltageChannel {
                        index: 0,
                        voltage: 0.,
                        current: 0.,
                        state: Default::default(),
                    },
                },
                |mut a, b| {
                    a.index = b.index;
                    a.ch.index = b.ch.index;
                    a.ch.voltage += b.ch.voltage;
                    a.ch.current += b.ch.current;
                    a
                },
            );
            item.ch.voltage /= l as f32;
            item.ch.current /= l as f32;
            item.ch.set_state(verify);

            item
        })
        .map(|item| item.ch)
        .filter(|ch| ch.state != VoltageState::Qualified)
        .collect::<Vec<VoltageChannel>>()
}

/// 获取每 n 分钟的平均数据
pub fn voltage_average_every_n_minutes(
    groups: Vec<VoltageDataGroup>,
    minutes: u64,
) -> Vec<VoltageChannelItem> {
    // let mut current_interval_start = 0;

    //存储渠道长度
    let mut channel_len: usize = 0;

    let list: Vec<VoltageChannelItem> = groups
        .iter()
        .enumerate()
        .flat_map(|(index_group, group)| {
            let group_len = group.data.len();
            group
                .data
                .iter()
                .flat_map(|slave| {
                    let ch_len = slave.data.len() * group_len;
                    channel_len = channel_len.max(ch_len);
                    slave
                        .data
                        .iter()
                        .map(|c| VoltageChannelItem {
                            index: index_group * ch_len + c.index,
                            time: group.time,
                            ch: *c,
                        })
                        .collect::<Vec<VoltageChannelItem>>()
                })
                .collect::<Vec<VoltageChannelItem>>()
        })
        .collect::<Vec<VoltageChannelItem>>();

    // 按照渠道分组
    let channels = (0..channel_len)
        .map(|index| {
            list.iter()
                .filter(|&item| item.ch.index == index)
                .cloned()
                .collect::<Vec<VoltageChannelItem>>()
        })
        .collect::<Vec<Vec<VoltageChannelItem>>>();

    // 直接返回
    if minutes == 0 {
        let list = channels.iter().flatten().cloned().collect();
        return list;
    }

    // 存储分钟节点
    let list = channels
        .iter()
        .flat_map(|chs| {
            //  每个频道的数据组
            let mut min_list: HashMap<u64, Vec<VoltageChannelItem>> = HashMap::new();
            // 这里使用 keys 主要是因为 hashmap 无序
            let mut keys: Vec<u64> = Vec::new();

            chs.iter()
                .map(|item| {
                    let item_minute = time_dur_odt(item.time).replace_second(0).unwrap();
                    let key = item_minute.unix_timestamp() as u64;

                    // 分钟内
                    let key_start = item_minute
                        .sub(time::Duration::minutes(minutes as i64))
                        .unix_timestamp() as u64;
                    // 查询最近的 key

                    let key = if keys.is_empty() {
                        keys.push(key);
                        key
                    } else if let Some(k) = keys.iter().find(|&key| *key > key_start) {
                        *k
                    } else {
                        keys.push(key);
                        key
                    };
                    (key, item)
                })
                .for_each(|(key, &item)| {
                    min_list.entry(key).or_default().push(item);
                });

            // 合并
            keys.iter()
                .map(|&key| {
                    let item = min_list.get(&key).unwrap();
                    let l = item.len();

                    let mut item = item.iter().fold(
                        VoltageChannelItem {
                            index: 0,
                            time: Duration::from_secs(key),
                            ch: VoltageChannel {
                                index: 0,
                                voltage: 0.,
                                current: 0.,
                                state: Default::default(),
                            },
                        },
                        |mut a, b| {
                            a.index = b.index;
                            a.ch.index = b.ch.index;
                            a.ch.voltage += b.ch.voltage;
                            a.ch.current += b.ch.current;
                            a
                        },
                    );
                    item.ch.voltage /= l as f32;
                    item.ch.current /= l as f32;

                    item
                })
                .collect::<Vec<VoltageChannelItem>>()
        })
        .collect::<Vec<VoltageChannelItem>>();

    // 为 excel 重新排序
    let mut list = list;
    list.iter_mut()
        .enumerate()
        .for_each(|(index, item)| item.index = index);

    list
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
        // return;

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
                eprintln!("set 失败: {}", e);
            }
        };

        match TableVoltage::get_len(&db, AB::A) {
            Ok(data) => {
                println!("{:?}", data);
            }
            Err(e) => {
                eprintln!("len 失败: {}", e);
            }
        };

        match TableVoltage::get(&db, dur.as_secs(), AB::A) {
            Ok(data) => {
                println!("{:?}", data);
            }
            Err(e) => {
                eprintln!("获取失败: {}", e);
            }
        };

        match TableVoltage::list(&db, AB::A) {
            Ok(data) => {
                println!("{:?}", data);
            }
            Err(e) => {
                eprintln!("获取失败: {}", e);
            }
        };
    }
}
