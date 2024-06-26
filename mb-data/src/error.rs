use std::num::ParseIntError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Time(#[from] std::time::SystemTimeError),

    #[error(transparent)]
    DB(#[from] redb::Error),

    #[error(transparent)]
    DB1(#[from] redb::TableError),

    #[error(transparent)]
    DB2(#[from] redb::CommitError),

    #[error(transparent)]
    DB3(#[from] redb::StorageError),

    #[error(transparent)]
    DB4(#[from] redb::DatabaseError),

    #[error(transparent)]
    DB5(#[from] redb::SavepointError),

    #[error(transparent)]
    DB6(#[from] redb::CompactionError),

    #[error(transparent)]
    DB7(#[from] redb::TransactionError),

    #[error(transparent)]
    Prase(#[from] ParseIntError),

    #[error("没有对应的数据")]
    DbNone,

    #[error("设备: {device} 连接失败")]
    ConnectFail { device: String },

    #[error("设备: {device} 链接丢失")]
    ConnectLost { device: String },

    #[error("{msg}")]
    Msg { msg: String },

    #[error("配置失败")]
    Fail,
}

use mb::Result;

impl<T> Into<Result<T>> for Error {
    fn into(self) -> Result<T> {
        Err(Box::new(self))
    }
}
