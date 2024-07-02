//!  电源

use crate::{
    error::Error,
    protocol::{FunRequest, FunResponse, Function, FunctionCode},
    utils::current_timestamp,
};

pub struct Power;

impl Power {
    pub fn request(slave: u8, mode: &PowerMode) -> FunRequest {
        let mode = mode.params();
        Function::new(slave, mode.0, mode.1.to_vec())
    }
}

/// 命令请求类型
pub enum PowerMode {
    ///实际温度
    Temp,
    /// 实际电压
    Voltage,
    /// 实际电流
    Current,

    /// 电源是否启动
    GetOnOff,
    /// 获取设置的电压
    GetVoltage,
    /// 获取设置的电流
    GetCurrent,

    /// 远程启动
    SetOnOff,
    /// 设定电压
    SetVoltage(u16),
    /// 设定电流
    SetCurrent(u16),
}

impl PowerMode {
    pub fn params(&self) -> (FunctionCode, [u16; 2]) {
        match self {
            PowerMode::Temp => (FunctionCode::ReadHoldingRegisters, [2, 0]),
            PowerMode::Voltage => (FunctionCode::ReadHoldingRegisters, [4, 0]),
            PowerMode::Current => (FunctionCode::ReadHoldingRegisters, [6, 0xC000]),

            PowerMode::GetOnOff => (FunctionCode::ReadHoldingRegisters, [9, 0x0003]),
            PowerMode::GetVoltage => (FunctionCode::ReadHoldingRegisters, [0x000A, 0]),
            PowerMode::GetCurrent => (FunctionCode::ReadHoldingRegisters, [0x000C, 0]),

            PowerMode::SetOnOff => (FunctionCode::WriteMultipleRegisters, [9, 0x0003]),
            PowerMode::SetVoltage(n) => (FunctionCode::WriteMultipleRegisters, [0x000A, *n]),
            PowerMode::SetCurrent(n) => (FunctionCode::WriteMultipleRegisters, [0x000C, *n]),
        }
    }
}

/// 温度
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct PowerData {
    pub time: u64,
    pub value: f32,
}

impl TryFrom<FunResponse> for PowerData {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(value: FunResponse) -> std::result::Result<Self, Self::Error> {
        let data: Vec<f32> = value
            .data()
            .iter()
            .map(|&r| r as f32 * 0.1)
            .collect::<Vec<f32>>();
        let value = data.get(0).ok_or(Box::new(Error::DataNull))?;

        let dur = current_timestamp();
        let temp = PowerData {
            time: dur,
            value: *value,
        };
        Ok(temp)
    }
}
