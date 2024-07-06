//! 温度
//! 参看 温度 680通信协议
//! RS232 接口
//! 10 的温度 1 返回温度 u16 * 10.0
//! ```
//! vec![0x01, 0x03, 0x00, 0x0A, 0x00, 0x01, 0xA4, 0x08];
//! ```
//!
//! 写入 d60，命令写入，结果和请求相同
//! ```
//! let req = vec![0x01, 0x06, 0x00, 0x3C, 0x02, 0x59, 0x88, 0x9C];
//! let res = vec![0x01, 0x06, 0x00, 0x3C, 0x02, 0x59, 0x88, 0x9C];
//! ```

use crate::{
    error::Error,
    protocol::{FunRequest, FunResponse, Function, FunctionCode},
    utils::current_timestamp,
};

pub struct Temperature;

impl Temperature {
    pub fn request(slave: u8, mode: &TemperatureMode) -> FunRequest {
        let mode = mode.params();
        Function::new(slave, mode.0, mode.1.to_vec())
    }
}

/// 命令请求类型
pub enum TemperatureMode {
    /// 温度1: 获取温度 * 0.1
    Temp1,
    /// 温度2: 获取温度 * 0.1
    Temp2,
    /// 设定温度1: 温度 * 10.0
    Set1(u16),
    /// 设定温度2: 温度 * 10.0
    Set2(u16),
    /// 运行状态: 0 停止 1 运行 2 暂停
    Run(u16),
    /// 按键1: 0 on 1 off
    KeyA(u16),
    /// 按键2: 0 on 1 off
    KeyB(u16),
}

impl TemperatureMode {
    /// 获取参数 (功能, 指令, 参数)
    pub fn params(&self) -> (FunctionCode, [u16; 2]) {
        match self {
            TemperatureMode::Temp1 => (FunctionCode::ReadHoldingRegisters, [10, 1]),
            TemperatureMode::Temp2 => (FunctionCode::ReadHoldingRegisters, [14, 1]),
            TemperatureMode::Set1(n) => (FunctionCode::WriteSingleRegister, [60, *n]),
            TemperatureMode::Set2(n) => (FunctionCode::WriteSingleRegister, [61, *n]),
            TemperatureMode::Run(n) if *n < 3 => (FunctionCode::WriteSingleRegister, [63, *n]), // 0 1 2
            TemperatureMode::KeyA(n) if *n < 2 => (FunctionCode::WriteSingleRegister, [46, *n]), // 0 1
            TemperatureMode::KeyB(n) if *n < 2 => (FunctionCode::WriteSingleRegister, [47, *n]), // 0 1
            TemperatureMode::Run(_) => (FunctionCode::WriteSingleRegister, [63, 0]), // 0 1 2
            TemperatureMode::KeyA(_) => (FunctionCode::WriteSingleRegister, [46, 0]), // 0 1
            TemperatureMode::KeyB(_) => (FunctionCode::WriteSingleRegister, [47, 0]), // 0 1
        }
    }
}

/// 温度
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct TemperatureData {
    pub time: u64,
    pub value: f32,
}

impl TryFrom<FunResponse> for TemperatureData {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(value: FunResponse) -> std::result::Result<Self, Self::Error> {
        let data: Vec<f32> = value
            .data()
            .iter()
            .map(|&r| r as f32 * 0.1)
            .collect::<Vec<f32>>();
        let value = data.first().ok_or(Box::new(Error::DataNull))?;

        let dur = current_timestamp();
        let temp = TemperatureData {
            time: dur,
            value: *value,
        };
        Ok(temp)
    }
}
