use mb::{protocol::default_port_name, voltage::Verify};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    pub voltage: VoltageConfig,
    pub temperature: TemperatureConfig,
    pub relay: RelayConfig,
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
    pub salve_a: Vec<u8>,
    pub salve_b: Vec<u8>,

    // 验证
    pub verify: Verify,
}

// 温度
// 双温控
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TemperatureConfig {
    pub name: String,
    pub serial_port: SerialPortConfig,
    pub salve: u8,
}

// 继电器
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RelayConfig {
    pub name: String,
    pub serial_port: SerialPortConfig,
    pub salve: u8,
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

    pub fn to_u32(&self) -> u32 {
        match self {
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
