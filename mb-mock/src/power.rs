use crate::Mock;
use mb::{
    power::{Power, PowerMode},
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
        PowerMock::new(value[0], PowerMode::GetOnOff)
    }
}

impl Mock for PowerMock {
    fn request(&self) -> Function {
        Power::request(self.slave, &self.mode)
    }

    fn response(&self) -> Function {
        let mode = self.mode.params();

        let response = Function::new(self.slave, mode.0, vec![60 * 10]);

        response
    }
}
