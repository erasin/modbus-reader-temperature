use crate::Mock;
use mb::temperature::TempMode;

pub struct TempMock {
    slave: u8,
    mode: TempMode,
}

impl TempMock {
    pub fn new(slave: u8, mode: TempMode) -> Self {
        TempMock { slave, mode }
    }

    pub fn from_request(request: &[u8]) -> Self {
        TempMock::new(0x02, TempMode::Temp1)
    }
}

impl Mock for TempMock {
    fn request(&self) -> Vec<u8> {
        // let mode = mb::temperature::TempMode::W1;
        mb::temperature::request(self.slave, &self.mode)
    }

    fn response(&self) -> Vec<u8> {
        let mode = self.mode.params();

        let response = mb::protocol::response(self.slave, mode.0, vec![60 * 10]);

        response
    }
}
