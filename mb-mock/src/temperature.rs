use crate::Mock;
use mb::{
    protocol::{Function, FunctionCode},
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
        let req = Function::parse_request(value).unwrap();

        match req.code() {
            FunctionCode::ReadHoldingRegisters => {
                if req.data()[0] == 10 {
                    TempMock::new(req.slave(), TemperatureMode::Temp1)
                } else {
                    TempMock::new(req.slave(), TemperatureMode::Temp2)
                }
            }
            FunctionCode::ReadCoils => todo!(),
            FunctionCode::ReadDiscreteInputs => todo!(),
            FunctionCode::ReadInputRegisters => todo!(),
            FunctionCode::WriteSingleCoil => todo!(),
            FunctionCode::WriteSingleRegister => todo!(),
            FunctionCode::WriteMultipleCoils => todo!(),
            FunctionCode::WriteMultipleRegisters => todo!(),
            FunctionCode::MaskWriteRegister => todo!(),
            FunctionCode::ReadWriteMultipleRegisters => todo!(),
            FunctionCode::Custom(_) => todo!(),
        }
    }
}

impl Mock for TempMock {
    fn request(&self) -> Function {
        Temperature::request(self.slave, &self.mode)
    }

    fn response(&self) -> Function {
        let mode = self.mode.params();

        Function::new(self.slave, mode.0, vec![60 * 10])
    }
}
