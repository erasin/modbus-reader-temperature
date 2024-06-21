use mb::{protocol::default_port_name, voltage::Verify};

#[derive(Debug, Clone)]
pub struct Config {
    pub port: String,       // com or tty
    pub baudrate: Baudrate, //
    pub verify: Verify,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: default_port_name(),
            baudrate: Baudrate::R115200,
            verify: Verify::default(),
        }
    }
}

impl Config {
    pub fn port(&self) -> String {
        self.port.clone()
    }

    pub fn baudrate(&self) -> Baudrate {
        self.baudrate
    }

    pub async fn load() -> Option<Config> {
        let config = Config::default();

        Some(config)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Baudrate {
    R1200,
    R2400,
    R4800,
    R9600,
    R19200,
    R38400,
    R57600,
    #[default]
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
