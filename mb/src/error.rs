#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("格式解析失败")]
    MbParseFail,

    #[error("数据为空")]
    DataNull,
}
