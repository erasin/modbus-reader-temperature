//! 继电器

use std::fmt::Display;

use crate::{
    error::Error,
    protocol::{Function, FunctionCode},
    utils::current_timestamp,
    Result,
};

/// 继电器
/// 1 2 奇偶校验
/// 3-5 波特率
/// 6-10 地址位 10 -> 6 二进制
/// 继电器 0 ，参数二进制控制开关(8位) 0b00000000;
pub fn request(slave: u8, mode: &RelayMode) -> Vec<u8> {
    // 写入
    let mode = mode.params(); //(0x06, 0, 0b00000000);
    let params = vec![mode.1, mode.2];

    Function::new(slave, mode.0, params).request()
}

pub fn response(data: Vec<u8>) -> Result<RelayData> {
    let data = Function::parse_response(&data)?;
    RelayData::parse_u16(data.data)
}

pub enum RelayMode {
    /// 0b0000_0000 二进制八位占位符表示灯开关
    ONOFF(u16),
    /// 第一个值为读取值，第二个值为位置
    ON(u16, u8),
    /// 第一个值为读取值，第二个值为位置
    OFF(u16, u8),
    /// 读取
    Read,
}

impl RelayMode {
    pub fn params(&self) -> (FunctionCode, u16, u16) {
        match self {
            RelayMode::ONOFF(n) => (FunctionCode::WriteSingleRegister, 0, *n),
            RelayMode::Read => (FunctionCode::ReadHoldingRegisters, 0, 1),
            RelayMode::ON(value, position) => {
                let bit = RelayData::set_bit(*value, *position, true);

                (FunctionCode::WriteSingleRegister, 0, bit)
            }
            RelayMode::OFF(value, position) => {
                let bit = RelayData::set_bit(*value, *position, false);

                (FunctionCode::WriteSingleRegister, 0, bit)
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct RelayData {
    pub time: u64,
    pub value: u16,
}

impl RelayData {
    fn parse_u16(data: Vec<u16>) -> Result<RelayData> {
        let value = data.get(0).ok_or(Error::DataNull)?;

        let dur = current_timestamp();
        let temp = RelayData {
            time: dur,
            value: *value,
        };
        Ok(temp)
    }

    pub fn get_state(&self, position: u8) -> bool {
        let position = if position > 8 { 7 } else { position };
        (self.value & (1 << position)) != 0
    }

    // 修改第 n 位为 state
    pub fn set_bit(value: u16, position: u8, state: bool) -> u16 {
        let position = if position > 8 { 7 } else { position };
        if state {
            value | (1 << position)
        } else {
            value & !(1 << position)
        }
    }
}

impl Display for RelayData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "time:{}\nvalue: {:08b}", self.time, self.value)
    }
}
