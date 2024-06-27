use std::num::ParseIntError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Time(#[from] std::time::SystemTimeError),

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

impl<T> Into<mb::Result<T>> for Error {
    fn into(self) -> mb::Result<T> {
        Err(Box::new(self))
    }
}
