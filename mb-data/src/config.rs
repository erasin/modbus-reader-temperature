use mb::{protocol::default_port_name, voltage::Verify};
use serde::{Deserialize, Serialize};

use crate::dirs;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    /// 老化柜名称
    pub pro_name: String,

    pub enable_a_panel: bool,
    pub enable_b_panel: bool,

    pub voltage_a: VoltageConfig,
    pub voltage_b: VoltageConfig,

    pub temperature: TemperatureConfig,
    pub relay: RelayConfig,

    /// 电源
    pub power_a: PowerConfig,
    pub power_b: PowerConfig,

    /// 不良品判定
    pub defective: DefectiveConfig,

    /// 历史数据
    pub history: HistoryConfig,
}

// 端口
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialPortConfig {
    pub name: String,
    pub port: String,       // com or tty
    pub baudrate: Baudrate, //
}

impl Default for SerialPortConfig {
    fn default() -> Self {
        Self {
            name: String::default(),
            port: default_port_name(),
            baudrate: Baudrate::default(),
        }
    }
}

// 电流
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VoltageConfig {
    pub name: String,
    pub serial_port: SerialPortConfig,

    // 站
    pub slave_start: u8,
    pub slave_end: u8,

    // 验证
    pub verify: Verify,
}

// 温度
// 双温控
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TemperatureConfig {
    pub name: String,
    pub serial_port: SerialPortConfig,
    pub slave: u8,
}

// 继电器
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RelayConfig {
    pub name: String,
    pub serial_port: SerialPortConfig,
    pub slave: u8,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PowerConfig {
    pub name: String,
    pub serial_port: SerialPortConfig,
    pub slave: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Baudrate {
    R1200,
    R2400,
    R4800,
    #[default]
    R9600,
    R19200,
    R38400,
    R57600,
    R115200,
}

impl Baudrate {
    pub const ALL: [Baudrate; 8] = [
        Baudrate::R1200,
        Baudrate::R2400,
        Baudrate::R4800,
        Baudrate::R9600,
        Baudrate::R19200,
        Baudrate::R38400,
        Baudrate::R57600,
        Baudrate::R115200,
    ];
}

impl From<Baudrate> for u32 {
    fn from(val: Baudrate) -> Self {
        match val {
            Baudrate::R1200 => 1200,
            Baudrate::R2400 => 2400,
            Baudrate::R4800 => 4800,
            Baudrate::R9600 => 9600,
            Baudrate::R19200 => 19200,
            Baudrate::R38400 => 38400,
            Baudrate::R57600 => 57600,
            Baudrate::R115200 => 115200,
        }
    }
}

impl From<u32> for Baudrate {
    fn from(value: u32) -> Self {
        match value {
            1200 => Baudrate::R1200,
            2400 => Baudrate::R2400,
            4800 => Baudrate::R4800,
            9600 => Baudrate::R9600,
            19200 => Baudrate::R19200,
            38400 => Baudrate::R38400,
            57600 => Baudrate::R57600,
            115200 => Baudrate::R115200,
            _ => Baudrate::default(),
        }
    }
}

impl std::fmt::Display for Baudrate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Baudrate::R1200 => "1200",
                Baudrate::R2400 => "2400",
                Baudrate::R4800 => "4800",
                Baudrate::R9600 => "9600",
                Baudrate::R19200 => "19200",
                Baudrate::R38400 => "38400",
                Baudrate::R57600 => "57600",
                Baudrate::R115200 => "115200",
            }
        )
    }
}

/// 不良品设置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DefectiveConfig {
    pub rule: DefectiveRule,
    pub dur: u64,
}

#[derive(Debug, PartialEq, PartialOrd, Default, Copy, Clone, Serialize, Deserialize)]
pub enum DefectiveRule {
    #[default]
    RealTime,
    InTime,
}

impl DefectiveRule {
    pub const ALL: [DefectiveRule; 2] = [DefectiveRule::RealTime, DefectiveRule::InTime];

    pub fn title(self) -> String {
        match self {
            DefectiveRule::RealTime => "实时".to_owned(),
            DefectiveRule::InTime => "时间段".to_owned(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryConfig {
    /// 记录时间频率
    pub log_dur: u32,
    /// 延时计算良品率
    pub defective_lazy_dur: u32,
    /// 导出路径
    pub export_dir: String,
}

impl Default for HistoryConfig {
    fn default() -> Self {
        Self {
            log_dur: 5,
            defective_lazy_dur: 0,
            export_dir: dirs::doc_dir().to_string_lossy().to_string(),
        }
    }
}
