use crate::Mock;

pub struct TempMock {
    slave: u8,
}

impl TempMock {
    pub fn new(slave: u8) -> Self {
        TempMock { slave }
    }
}

impl Mock for TempMock {
    fn request(&self) -> Vec<u8> {
        let mode = mb::temperature::TempMode::W1;
        mb::temperature::request(self.slave, mode)
    }

    fn response(&self) -> Vec<u8> {
        let mode = mb::temperature::TempMode::W1;
        let mode = mode.params();

        let response = mb::protocol::response(self.slave, mode.0, vec![60 * 10]);

        response
    }
}
