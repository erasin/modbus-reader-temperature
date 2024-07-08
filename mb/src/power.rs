//!  电源

use std::time::Duration;

use crate::{
    error::Error,
    protocol::{FunRequest, FunResponse, Function, FunctionCode},
    utils::current_timestamp,
};

pub struct Power;

impl Power {
    pub fn request(slave: u8, mode: &PowerMode) -> FunRequest {
        let mode = mode.params();
        Function::new(slave, mode.0, mode.1)
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
    SetVoltage(f32),
    /// 设定电流
    SetCurrent(f32),
}

impl PowerMode {
    pub fn params(&self) -> (FunctionCode, Vec<u16>) {
        match self {
            PowerMode::Temp => (FunctionCode::ReadHoldingRegisters, [2, 0].to_vec()),
            PowerMode::Voltage => (FunctionCode::ReadHoldingRegisters, [4, 0].to_vec()),
            PowerMode::Current => (FunctionCode::ReadHoldingRegisters, [6, 0xC000].to_vec()),

            PowerMode::GetOnOff => (FunctionCode::ReadHoldingRegisters, [9, 0x0003].to_vec()),
            PowerMode::GetVoltage => (FunctionCode::ReadHoldingRegisters, [0x000A, 0].to_vec()),
            PowerMode::GetCurrent => (FunctionCode::ReadHoldingRegisters, [0x000C, 0].to_vec()),

            PowerMode::SetOnOff => (FunctionCode::WriteMultipleRegisters, [9, 0x0003].to_vec()),
            PowerMode::SetVoltage(n) => {
                let f = f32_u16(*n);
                let data = vec![0x000A, f[0], f[1]];
                (FunctionCode::WriteMultipleRegisters, data)
            }
            PowerMode::SetCurrent(n) => {
                let f = f32_u16(*n);
                let data = vec![0x000C, f[0], f[1]];
                (FunctionCode::WriteMultipleRegisters, data)
            }
        }
    }
}

// 32 转 16
pub fn f32_u16(v: f32) -> [u16; 2] {
    let b = v.to_bits();
    let bytes = b.to_be_bytes();
    let u16_array: [u16; 2] = [
        u16::from_be_bytes(bytes[0..2].try_into().unwrap()),
        u16::from_be_bytes(bytes[2..4].try_into().unwrap()),
    ];
    u16_array
}

/// 电源
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct PowerData {
    pub time: Duration,
    pub value: f32,
}

impl TryFrom<FunResponse> for PowerData {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(value: FunResponse) -> std::result::Result<Self, Self::Error> {
        let bytes = value.data_u8();

        let data: Vec<f32> = bytes
            .as_slice()
            .chunks(4)
            .filter_map(|bs| {
                if bs.len() == 4 {
                    let b: [u8; 4] = bs.try_into().unwrap();
                    Some(f32::from_be_bytes(b))
                } else {
                    None
                }
            })
            .collect();

        let data = data.first().ok_or(Box::new(Error::DataNull))?;

        let dur = current_timestamp();
        let temp = PowerData {
            time: dur,
            value: *data,
        };
        Ok(temp)
    }
}
