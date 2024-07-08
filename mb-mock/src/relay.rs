use std::sync::{Mutex, OnceLock};

use crate::Mock;
use mb::{
    protocol::{FunRequest, FunResponse, Function, FunctionCode},
    relay::{Relay, RelayData, RelayMode},
};
use rand::Rng;

fn fake_data() -> &'static Mutex<u16> {
    static MOCK_DATA: OnceLock<Mutex<u16>> = OnceLock::new();
    MOCK_DATA.get_or_init(|| Mutex::new(0))
}

#[derive(Debug)]
pub struct RelayMock {
    slave: u8,
    mode: RelayMode,
    req: Option<Function>,
}

impl RelayMock {
    pub fn new(slave: u8, mode: RelayMode) -> Self {
        RelayMock {
            slave,
            mode,
            req: None,
        }
    }

    pub fn set_req(&mut self, req: Function) {
        self.req = Some(req);
    }
}

impl From<&[u8]> for RelayMock {
    fn from(value: &[u8]) -> Self {
        let req = Function::parse_request(value).unwrap();

        match req.code() {
            FunctionCode::ReadHoldingRegisters => RelayMock::new(value[0], RelayMode::Read),
            _ => {
                // 解析请求

                let mut data = fake_data().lock().unwrap();
                let req_data = req.data();
                // 因为为请求数据 0 为线圈， 1 为数据
                *data = req_data[1];
                let mut mock = RelayMock::new(value[0], RelayMode::ONOFF(0));
                mock.set_req(req);
                mock
            }
        }
    }
}

impl Mock for RelayMock {
    fn request(&self) -> FunRequest {
        match self.mode {
            RelayMode::Read => Relay::request(self.slave, &self.mode),
            _ => self
                .req
                .clone()
                .unwrap_or(Relay::request(self.slave, &self.mode)),
        }
    }

    fn response(&self) -> FunResponse {
        let data = fake_data().lock().unwrap();
        let mode = self.mode.params();
        match self.mode {
            RelayMode::Read => Function::new(self.slave, mode.0, vec![*data]),
            _ => self.req.clone().unwrap(),
        }
    }
}

fn generate_relay() -> Vec<u16> {
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..2);
    let res = RelayData::set_bit(0, i, true);
    vec![res]
}
