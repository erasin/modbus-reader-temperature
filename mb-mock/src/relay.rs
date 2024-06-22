use crate::Mock;
use mb::{
    protocol::{FunRequest, FunResponse, Function, FunctionCode},
    relay::{Relay, RelayData, RelayMode},
};
use rand::Rng;

pub struct RelayMock {
    slave: u8,
    mode: RelayMode,
}

impl RelayMock {
    pub fn new(slave: u8, mode: RelayMode) -> Self {
        RelayMock { slave, mode }
    }
}

impl From<&[u8]> for RelayMock {
    fn from(value: &[u8]) -> Self {
        match value[1] {
            n if n == FunctionCode::ReadHoldingRegisters.value() => {
                RelayMock::new(value[0], RelayMode::Read)
            }
            _ => {
                // 解析请求
                let req = Function::parse_request(value).unwrap().data();
                let bit = req.get(1).unwrap();
                RelayMock::new(value[0], RelayMode::ONOFF(*bit))
            }
        }
    }
}

impl Mock for RelayMock {
    fn request(&self) -> FunRequest {
        Relay::request(self.slave, &self.mode)
    }

    fn response(&self) -> FunResponse {
        let mode = self.mode.params();
        let response = match self.mode {
            RelayMode::Read => Function::new(self.slave, mode.0, generate_relay()),
            _ => Relay::request(self.slave, &self.mode),
        };
        response
    }
}

fn generate_relay() -> Vec<u16> {
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..8);
    let res = RelayData::set_bit(0, i, true);
    vec![res]
}
