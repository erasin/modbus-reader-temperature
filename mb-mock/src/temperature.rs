use crate::Mock;
use mb::{
    protocol::Function,
    temperature::{Temperature, TemperatureMode},
};

pub struct TempMock {
    slave: u8,
    mode: TemperatureMode,
}

impl TempMock {
    pub fn new(slave: u8, mode: TemperatureMode) -> Self {
        TempMock { slave, mode }
    }
}

impl From<&[u8]> for TempMock {
    fn from(value: &[u8]) -> Self {
        TempMock::new(value[0], TemperatureMode::Temp1)
    }
}

impl Mock for TempMock {
    fn request(&self) -> Function {
        Temperature::request(self.slave, &self.mode)
    }

    fn response(&self) -> Function {
        let mode = self.mode.params();

        let response = Function::new(self.slave, mode.0, vec![60 * 10]);

        response
    }
}
