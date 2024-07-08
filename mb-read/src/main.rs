#![allow(dead_code)]

use mb::{
    power::PowerData,
    protocol::Builder,
    relay::{RelayData, RelayMode},
    temperature::{TemperatureData, TemperatureMode},
    utils::print_hex,
    voltage::VoltageData,
    Result,
};
use mb_mock::{
    power::PowerMock, relay::RelayMock, temperature::TempMock, voltage::VoltageMock, Mock,
};

fn main() -> Result<()> {
    let req = MockReq::new();

    // req.run_temp(TempMock::new(0x01, TemperatureMode::Temp1))?;

    // req.run_relay(RelayMock::new(0x02, RelayMode::Read))?;
    // req.run_relay(RelayMock::new(0x02, RelayMode::ON(0, 1)))?;
    // req.run_relay(RelayMock::new(0x02, RelayMode::Read))?;

    req.run_power(PowerMock::new(0x03, mb::power::PowerMode::SetOnOff))?;
    req.run_power(PowerMock::new(0x03, mb::power::PowerMode::GetVoltage))?;

    // req.run_voltage(VoltageMock::new(0x05))?;

    Ok(())
}

struct MockReq {
    builder: Builder,
}

impl MockReq {
    fn new() -> Self {
        let port_name = "/dev/ttyUSB0";
        let baudrate = 9600;
        let builder = Builder::new(port_name, baudrate);
        Self { builder }
    }

    define_run_mock![
        (run_voltage, VoltageMock, VoltageData),
        (run_temp, TempMock, TemperatureData),
        (run_relay, RelayMock, RelayData),
        (run_power, PowerMock, PowerData),
    ];
}

#[macro_export]
macro_rules! define_run_mock{
    [$(($fn_name:ident, $mock:ty, $data_type:ty)),* $(,)?] => {
        $(
            fn $fn_name(&self, mock:$mock) -> Result<()>{

                println!("\n----\nstart $fn_name: \n",);

                let request = mock.request();
                print_hex("request", &request.request_data());

                let response = self.builder.call(&request)?;
                print_hex("response", &response.response_data());

                if response == request {
                    println!("命令执行\n");
                    return Ok(());
                }

                println!("u16:\n{:?}", response.data());
                let data: $data_type = response.try_into()?;
                println!("解析结果:\n{:?}", data);

                Ok(())
            }
        )*
    };
}
/// 电压电流
fn run_voltage(builder: &Builder, slave: u8) -> Result<()> {
    println!("\n----\nstart votage: \n");

    let mock = VoltageMock::new(slave);
    let request = mock.request();
    print_hex("request", &request.request_data());

    let response = builder.call(&request)?;
    print_hex("response", &response.response_data());

    if response == request {
        println!("--命令执行\n");
        return Ok(());
    }

    let data: VoltageData = response.try_into()?;
    println!("解析结果:\n{:?}", data);

    Ok(())
}

/// 温度
fn run_temp(builder: &Builder, slave: u8) -> Result<()> {
    println!("\n----\nstart temp: \n");
    let mock = TempMock::new(slave, TemperatureMode::Temp1);

    let request = mock.request();
    print_hex("request", &request.request_data());

    let response = builder.call(&request)?;
    print_hex("response", &response.response_data());

    if response == request {
        println!("命令执行\n");
        return Ok(());
    }

    let data: TemperatureData = response.try_into()?;
    println!("解析结果:\n{:?}", data);

    Ok(())
}

fn run_relay(builder: &Builder, slave: u8) -> Result<()> {
    println!("\n----\nstart relay: \n");
    let mock = RelayMock::new(slave, RelayMode::Read);
    let request = mock.request();
    print_hex("request", &request.request_data());

    let response = builder.call(&request)?;
    print_hex("response", &response.response_data());

    if response == request {
        println!("命令执行\n");
        return Ok(());
    }

    let data: RelayData = response.try_into()?;
    println!("解析结果:\n{}", data);
    Ok(())
}

fn run_power(builder: &Builder, slave: u8) -> Result<()> {
    println!("\n----\nstart power: \n");

    let mock = PowerMock::new(slave, mb::power::PowerMode::GetVoltage);

    let request = mock.request();
    print_hex("request", &request.request_data());

    let response = builder.call(&request)?;
    print_hex("response", &response.response_data());

    if response == request {
        println!("命令执行\n");
        return Ok(());
    }

    println!("u16:\n{:?}", response.data());
    let data: PowerData = response.try_into()?;
    println!("解析结果:\n{:?}", data);

    Ok(())
}
