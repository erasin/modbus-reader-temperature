//! 温度

use crate::{
    error::Error,
    protocol::{Function, FunctionCode},
    utils::current_timestamp,
    Result,
};

/// 参看 温度 680通信协议
/// RS232 接口
/// 10 的温度 1 返回温度 u16 * 10.0
/// ```
/// vec![0x01, 0x03, 0x00, 0x0A, 0x00, 0x01, 0xA4, 0x08];
/// ```
///
/// 写入 d60，命令写入，结果和请求相同
/// ```
/// let req = vec![0x01, 0x06, 0x00, 0x3C, 0x02, 0x59, 0x88, 0x9C];
/// let res = vec![0x01, 0x06, 0x00, 0x3C, 0x02, 0x59, 0x88, 0x9C];
/// ```
pub fn request(slave: u8, mode: &TempMode) -> Vec<u8> {
    let mode = mode.params();
    let params = vec![mode.1, mode.2];

    Function::new(slave, mode.0, params).request()
}

pub fn response(data: Vec<u8>) -> Result<Temperature> {
    let data = Function::parse_response(&data)?;
    Temperature::parse_u16(data.data)
}

/// 命令请求类型
pub enum TempMode {
    /// 温度1: 获取温度 * 0.1
    Temp1,
    /// 温度2: 获取温度 * 0.1
    Temp2,
    /// 设定温度1: 温度 * 10.0
    S1(u16),
    /// 设定温度2: 温度 * 10.0
    S2(u16),
    /// 运行状态: 0 停止 1 运行 2 暂停
    Run(u16),
    /// 按键1: 0 on 1 off
    KeyA(u16),
    /// 按键2: 0 on 1 off
    KeyB(u16),
}

impl TempMode {
    /// 获取参数 (功能, 指令, 参数)
    pub fn params(&self) -> (FunctionCode, u16, u16) {
        match self {
            TempMode::Temp1 => (FunctionCode::ReadHoldingRegisters, 10, 1),
            TempMode::Temp2 => (FunctionCode::ReadHoldingRegisters, 14, 1),
            TempMode::S1(n) => (FunctionCode::WriteSingleRegister, 60, *n),
            TempMode::S2(n) => (FunctionCode::WriteSingleRegister, 61, *n),
            TempMode::Run(n) if *n < 3 => (FunctionCode::WriteSingleRegister, 63, *n), // 0 1 2
            TempMode::KeyA(n) if *n < 2 => (FunctionCode::WriteSingleRegister, 46, *n), // 0 1
            TempMode::KeyB(n) if *n < 2 => (FunctionCode::WriteSingleRegister, 47, *n), // 0 1
            TempMode::Run(_) => (FunctionCode::WriteSingleRegister, 63, 0),            // 0 1 2
            TempMode::KeyA(_) => (FunctionCode::WriteSingleRegister, 46, 0),           // 0 1
            TempMode::KeyB(_) => (FunctionCode::WriteSingleRegister, 47, 0),           // 0 1
        }
    }
}

/// 温度
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Temperature {
    pub time: u64,
    pub value: f32,
}

impl Temperature {
    fn parse_u16(data: Vec<u16>) -> Result<Temperature> {
        let data: Vec<f32> = data.iter().map(|&r| r as f32 * 0.1).collect::<Vec<f32>>();
        let value = data.get(0).ok_or(Error::DataNull)?;

        let dur = current_timestamp();
        let temp = Temperature {
            time: dur,
            value: *value,
        };
        Ok(temp)
    }
}
