#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("格式解析失败")]
    MbParseFail,

    #[error("响应数据太短")]
    DataShort,

    #[error("响应数据长度不匹配")]
    DataLenError,

    #[error("数据为空")]
    DataNull,
}

impl<T> From<Error> for crate::Result<T> {
    fn from(val: Error) -> Self {
        Err(Box::new(val))
    }
}
