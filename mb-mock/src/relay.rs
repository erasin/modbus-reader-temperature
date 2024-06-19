use crate::Mock;
use mb::{
    protocol::{parse_modbus_request, FunctionCode},
    relay::{RelayData, RelayMode},
    utils::print_hex,
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

    pub fn from_request(request: &[u8]) -> Self {
        match request[1] {
            n if n == FunctionCode::ReadHoldingRegisters.value() => {
                RelayMock::new(request[0], RelayMode::Read)
            }
            _ => {
                let req = parse_modbus_request(request).unwrap();
                let value = req.get(1).unwrap();
                RelayMock::new(request[0], RelayMode::ONOFF(*value))
            }
        }
    }
}

impl Mock for RelayMock {
    fn request(&self) -> Vec<u8> {
        mb::relay::request(self.slave, &self.mode)
    }

    fn response(&self) -> Vec<u8> {
        let mode = self.mode.params();
        let response = match self.mode {
            RelayMode::Read => mb::protocol::response(self.slave, mode.0, generate_relay()),
            _ => mb::relay::request(self.slave, &self.mode),
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
