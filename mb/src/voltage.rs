//! 电流电压
//! RS485
//! b 115200

use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::protocol::{FunRequest, FunResponse, Function, FunctionCode};
use crate::utils::current_timestamp;

/// 电压电流
/// 返回
/// ```
/// // 请求
/// let request: [u8; 8] = [0x01, 0x04, 0x00, 0x00, 0x00, 0x1E, 0x70, 0x02];
///
/// ```
///

pub struct Voltage;
pub const VOLTAGE_CHANNEL: usize = 15;

impl Voltage {
    pub fn request(slave: u8) -> FunRequest {
        let mode = FunctionCode::ReadInputRegisters;
        let params = vec![0x00, 0x1E];
        Function::new(slave, mode, params)
    }
}

/// 电压数据集合
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoltageData {
    pub time: Duration,
    pub slave: u8,
    pub data: Vec<VoltageChannel>,
}

impl VoltageData {
    pub fn new(dur: Duration, slave: u8, data: Vec<VoltageChannel>) -> Self {
        Self {
            time: dur,
            slave,
            data,
        }
    }

    pub fn set_slave(&mut self, slave: u8) {
        self.slave = slave;
    }

    pub fn set_time(&mut self, dur: Duration) {
        self.time = dur;
    }

    pub fn update_channel_index(&mut self, index: usize) {
        self.data.iter_mut().for_each(|c| {
            c.index = c.index + index * VOLTAGE_CHANNEL;
        });
    }

    pub fn update_channel_state(&mut self, verify: &Verify) {
        self.data.iter_mut().for_each(|c| {
            c.state = get_mb_state(c, verify);
        });
    }

    pub fn voltage(&self) -> f32 {
        let l = self.data.len();

        self.data
            .iter()
            .map(|c| c.voltage)
            .fold(0_f32, |s, a| s + a)
            / l as f32
    }

    pub fn current(&self) -> f32 {
        let l = self.data.len();

        self.data
            .iter()
            .map(|c| c.current)
            .fold(0_f32, |s, a| s + a)
            / l as f32
    }
}

impl TryFrom<FunResponse> for VoltageData {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(data: FunResponse) -> std::result::Result<Self, Self::Error> {
        let slave = data.slave();
        let data = data.data();

        if data.is_empty() {
            return Err(Box::new(Error::DataNull));
        }

        if data.len() < 30 {
            return Err(Box::new(Error::DataLenError));
        }

        let chs: [f32; 30] = data
            .iter()
            .map(|&r| r as f32)
            .collect::<Vec<f32>>()
            .try_into()
            .unwrap_or([0.0; 30]);

        let mut data = VoltageData::from(chs);
        data.set_slave(slave);

        Ok(data)
    }
}

/// 电压电流
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct VoltageChannel {
    pub index: usize,
    pub voltage: f32,
    pub current: f32,
    pub state: VoltageState,
}

/// 15对 电压&电流 值
pub type VoltageF32 = [f32; 30];

impl From<VoltageF32> for VoltageData {
    fn from(value: VoltageF32) -> Self {
        let dur = current_timestamp();

        // TODO vec ?
        let mut ch_list: [VoltageChannel; 15] = [VoltageChannel::default(); 15];

        for (i, chunk) in value.chunks(2).enumerate() {
            if chunk.len() == 2 {
                let ch = VoltageChannel {
                    index: i,
                    voltage: chunk[0] / 1000.0,     // 电压 / 1000. 单位 V
                    current: chunk[1] / 1000.0,     // TODO 处理电流单位 mA ？
                    state: VoltageState::Qualified, // 默认正常
                };
                ch_list[i] = ch;
            }
        }

        VoltageData::new(dur, 0, ch_list.to_vec())
    }
}

impl From<VoltageData> for VoltageF32 {
    fn from(val: VoltageData) -> Self {
        let mut result = [0.0; 30];

        for ch in val.data.iter() {
            let idx = ch.index;
            if idx < 15 {
                result[idx * 2] = ch.voltage * 1000.0;
                result[idx * 2 + 1] = ch.current;
            }
        }

        result
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    Default,
    PartialEq,
    Eq,
    strum::EnumIter,
    strum::VariantArray,
    strum::Display,
)]
pub enum VoltageState {
    #[default]
    #[strum(to_string = "未连接")]
    NoConnected,
    #[strum(to_string = "空位")]
    Vacancy,
    #[strum(to_string = "合格")]
    Qualified,
    #[strum(to_string = "欠压")]
    UnderVoltage,
    #[strum(to_string = "欠流")]
    OverVoltage,
    #[strum(to_string = "过压")]
    UnderCurrent,
    #[strum(to_string = "过流")]
    OverCurrent,
    #[strum(to_string = "无输出")]
    NoOutput,
}

/// 获取电压状态
pub fn get_voltage_state(data: &VoltageChannel, verify: &Verify) -> VoltageState {
    match data.voltage {
        i if i >= verify.voltage_down && i <= verify.voltage_top => VoltageState::Qualified,
        i if i < verify.voltage_down && i > 0. => VoltageState::UnderVoltage,
        i if i > verify.voltage_top => VoltageState::OverVoltage,

        _ => VoltageState::NoOutput,
    }
}

/// 获取电流状态
pub fn get_current_state(data: &VoltageChannel, verify: &Verify) -> VoltageState {
    match data.current {
        i if i >= verify.current_down && i <= verify.current_top => VoltageState::Qualified,
        i if i < verify.current_down && i > 0. => VoltageState::UnderCurrent,
        i if i > verify.current_top => VoltageState::OverCurrent,

        _ => VoltageState::NoOutput,
    }
}

// 获取电压电流的总状态 电压优先
pub fn get_mb_state(data: &VoltageChannel, verify: &Verify) -> VoltageState {
    let state_voltage = get_voltage_state(data, verify);
    let state_current = get_current_state(data, verify);

    if state_voltage != VoltageState::Qualified {
        state_voltage
    } else {
        state_current
    }
}

/// 合格校验
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verify {
    /// 电压上限 V
    pub voltage_top: f32,
    /// 电压下限
    pub voltage_down: f32,
    /// 电流上限 A
    pub current_top: f32,
    /// 电流下限
    pub current_down: f32,
}

impl Default for Verify {
    fn default() -> Self {
        Self {
            voltage_top: 25.0,
            voltage_down: 1.0,
            current_top: 10.0,
            current_down: 1.0,
        }
    }
}
