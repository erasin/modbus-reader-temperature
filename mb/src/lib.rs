// #![allow(dead_code)]
// #![allow(unused_variables)]

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub mod error;
pub mod protocol;
pub mod relay;
pub mod temperature;
pub mod utils;
pub mod voltage;
