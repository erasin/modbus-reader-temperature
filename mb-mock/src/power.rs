use crate::Mock;
use mb::{
    power::{f32_u16, Power, PowerMode},
    protocol::Function,
};

pub struct PowerMock {
    slave: u8,
    mode: PowerMode,
}

impl PowerMock {
    pub fn new(slave: u8, mode: PowerMode) -> Self {
        PowerMock { slave, mode }
    }
}

impl From<&[u8]> for PowerMock {
    fn from(value: &[u8]) -> Self {
        // match value[1] {}
        PowerMock::new(value[0], PowerMode::GetVoltage)
    }
}

impl Mock for PowerMock {
    fn request(&self) -> Function {
        Power::request(self.slave, &self.mode)
    }

    fn response(&self) -> Function {
        let mode = self.mode.params();

        let data = f32_u16(60.);
        let response = Function::new(self.slave, mode.0, data.to_vec());

        response
    }
}
