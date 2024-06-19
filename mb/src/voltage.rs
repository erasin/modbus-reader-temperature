//! 电流电压

use crate::Result;

use crate::{
    error::Error,
    protocol::{self, parse_modbus_response},
    utils::current_timestamp,
};

/// 电压电流
/// RS485
/// b 115200
/// 返回
/// ```
/// // 请求
/// let request: [u8; 8] = [0x01, 0x04, 0x00, 0x00, 0x00, 0x1E, 0x70, 0x02];
///
/// ```
///
pub fn request(slave: u8) -> Vec<u8> {
    protocol::request(
        slave as u8,
        protocol::FunctionCode::ReadInputRegisters,
        vec![0x00, 0x1E],
    )
}

pub fn response(data: Vec<u8>) -> Result<Voltage> {
    let data = parse_modbus_response(&data).ok_or(Error::MbParseFail)?;

    let chs: [f32; 30] = data
        .iter()
        .map(|&r| r as f32)
        .collect::<Vec<f32>>()
        .try_into()
        .unwrap_or([0.0; 30]);

    let dur = current_timestamp();
    let chs = mb_f32_ch(chs);
    let data = Voltage::new(dur, chs);

    Ok(data)
}

#[derive(Debug, Clone, Copy)]
pub struct Voltage {
    pub time: u64,
    pub data: [VoltageChannel; 15],
}

#[derive(Debug, Clone, Copy, Default)]
pub struct VoltageChannel {
    pub index: u32,
    pub voltage: f32,
    pub current: f32,
}

impl Voltage {
    pub fn new(dur: u64, data: [VoltageChannel; 15]) -> Self {
        Self { time: dur, data }
    }
}

/// 格式转换
pub fn mb_f32_ch(data: [f32; 30]) -> [VoltageChannel; 15] {
    let mut ch_list: [VoltageChannel; 15] = [VoltageChannel::default(); 15];

    for (i, chunk) in data.chunks(2).enumerate() {
        if chunk.len() == 2 {
            let ch = VoltageChannel {
                index: i as u32,
                voltage: chunk[0] / 1000.0,
                current: chunk[1],
            };
            ch_list[i] = ch;
        }
    }

    ch_list
}

/// 格式转换
pub fn mb_ch_f32(data: [VoltageChannel; 15]) -> [f32; 30] {
    let mut result = [0.0; 30];

    for ch in data.iter() {
        let idx = ch.index as usize;
        if idx < 15 {
            result[idx * 2] = ch.voltage * 1000.0;
            result[idx * 2 + 1] = ch.current;
        }
    }

    result
}
#[derive(Debug, Clone, Default, PartialEq, Eq)]
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

impl VoltageState {
    pub fn vec() -> [VoltageState; 8] {
        [
            VoltageState::NoConnected,
            VoltageState::Vacancy,
            VoltageState::Qualified,
            VoltageState::UnderVoltage,
            VoltageState::OverVoltage,
            VoltageState::UnderCurrent,
            VoltageState::OverCurrent,
            VoltageState::NoOutput,
        ]
    }

    // pub fn style(&self) -> (Color, Color) {
    //     match self {
    //         VoltageState::NoConnected => (colors::BLACK, colors::GREY),
    //         VoltageState::Vacancy => (colors::BLACK, colors::WHITE),
    //         VoltageState::Qualified => (colors::BLACK, colors::GREEN),
    //         VoltageState::UnderVoltage => (colors::BLACK, colors::RED),
    //         VoltageState::UnderCurrent => (colors::BLACK, colors::YELLOW),
    //         VoltageState::OverVoltage => (colors::BLACK, colors::BLUE),
    //         VoltageState::OverCurrent => (colors::BLACK, colors::PUPLE),
    //         VoltageState::NoOutput => (colors::BLACK, colors::CYAN),
    //     }
    // }
}

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
#[derive(Debug, Clone)]
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
