use crate::Mock;
use mb::{
    protocol::{Function, FunctionCode},
    temperature::{Temperature, TemperatureMode},
};

pub struct TempMock {
    slave: u8,
    mode: TemperatureMode,
    req: Option<Function>,
}

impl TempMock {
    pub fn new(slave: u8, mode: TemperatureMode) -> Self {
        TempMock {
            slave,
            mode,
            req: None,
        }
    }

    pub fn set_fc(&mut self, req: Function) {
        self.req = Some(req);
    }
}

impl From<&[u8]> for TempMock {
    fn from(value: &[u8]) -> Self {
        let req = Function::parse_request(value).unwrap();

        match req.code() {
            FunctionCode::ReadHoldingRegisters => {
                if req.data()[0] == 10 {
                    TempMock::new(req.slave(), TemperatureMode::Temp1)
                } else {
                    TempMock::new(req.slave(), TemperatureMode::Temp2)
                }
            }
            FunctionCode::WriteSingleRegister => {
                let mut mock = TempMock::new(req.slave(), TemperatureMode::Run(0));
                mock.set_fc(req);
                mock
            }
            _ => todo!(),
        }
    }
}

impl Mock for TempMock {
    fn request(&self) -> Function {
        Temperature::request(self.slave, &self.mode)
    }

    fn response(&self) -> Function {
        let (mode, _data) = self.mode.params();
        if mode == FunctionCode::ReadHoldingRegisters {
            Function::new(self.slave, mode, vec![60 * 10])
        } else {
            self.req.clone().unwrap()
        }
    }
}
