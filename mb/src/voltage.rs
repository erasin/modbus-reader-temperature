//! 电流电压
//! RS485
//! b 115200

use serde::{Deserialize, Serialize};
use strum::{EnumIter, VariantArray};

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
    pub time: u64,
    pub data: Vec<VoltageChannel>,
}

impl VoltageData {
    pub fn new(dur: u64, data: Vec<VoltageChannel>) -> Self {
        Self { time: dur, data }
    }

    pub fn set_time(&mut self, dur: u64) {
        self.time = dur;
    }
}

impl TryFrom<FunResponse> for VoltageData {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(data: FunResponse) -> std::result::Result<Self, Self::Error> {
        let data = data.data();

        if data.len() == 0 {
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

        let data = VoltageData::from(chs);
        Ok(data)
    }
}

/// 电压电流
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct VoltageChannel {
    pub index: u32,
    pub voltage: f32,
    pub current: f32,
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
                    index: i as u32,
                    voltage: chunk[0] / 1000.0, // 电压 / 1000. 单位 V
                    current: chunk[1],          // TODO 处理电流单位 mA ？
                };
                ch_list[i] = ch;
            }
        }

        VoltageData::new(dur, ch_list.to_vec())
    }
}

impl Into<VoltageF32> for VoltageData {
    fn into(self) -> VoltageF32 {
        let mut result = [0.0; 30];

        for ch in self.data.iter() {
            let idx = ch.index as usize;
            if idx < 15 {
                result[idx * 2] = ch.voltage * 1000.0;
                result[idx * 2 + 1] = ch.current;
            }
        }

        result
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, EnumIter, VariantArray)]
pub enum VoltageState {
    #[default]
    NoConnected,
    Vacancy,
    Qualified,
    UnderVoltage,
    OverVoltage,
    UnderCurrent,
    OverCurrent,
    NoOutput,
}

// impl VoltageState {
//     pub fn vec() -> [VoltageState; 8] {
//         [
//             VoltageState::NoConnected,
//             VoltageState::Vacancy,
//             VoltageState::Qualified,
//             VoltageState::UnderVoltage,
//             VoltageState::OverVoltage,
//             VoltageState::UnderCurrent,
//             VoltageState::OverCurrent,
//             VoltageState::NoOutput,
//         ]
//     }
// }

impl std::fmt::Display for VoltageState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                VoltageState::NoConnected => "未连接",
                VoltageState::Vacancy => "空位",
                VoltageState::Qualified => "合格",
                VoltageState::UnderVoltage => "欠压",
                VoltageState::UnderCurrent => "欠流",
                VoltageState::OverVoltage => "过压",
                VoltageState::OverCurrent => "过流",
                VoltageState::NoOutput => "无输出",
            }
        )
    }
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
