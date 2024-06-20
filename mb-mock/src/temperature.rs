use crate::Mock;
use mb::{protocol::Function, temperature::TempMode};

pub struct TempMock {
    slave: u8,
    mode: TempMode,
}

impl TempMock {
    pub fn new(slave: u8, mode: TempMode) -> Self {
        TempMock { slave, mode }
    }
}

impl From<&[u8]> for TempMock {
    fn from(_value: &[u8]) -> Self {
        TempMock::new(0x02, TempMode::Temp1)
    }
}

impl Mock for TempMock {
    fn request(&self) -> Function {
        mb::temperature::request(self.slave, &self.mode)
    }

    fn response(&self) -> Function {
        let mode = self.mode.params();

        let response = Function::new(self.slave, mode.0, vec![60 * 10]);

        response
    }
}
