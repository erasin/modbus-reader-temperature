#![allow(dead_code)]

use mb::{
    power::PowerData,
    protocol::Builder,
    relay::{RelayData, RelayMode},
    temperature::{TemperatureData, TemperatureMode},
    utils::print_hex,
    voltage::{Voltage, VoltageData},
    Result,
};
use mb_mock::{power::PowerMock, relay::RelayMock, temperature::TempMock, Mock};

fn main() -> Result<()> {
    let port_name = "/dev/ttyUSB0";
    let baudrate = 9600;

    let builder = Builder::new(port_name, baudrate);

    run_temp(&builder, 0x01)?;
    run_relay(&builder, 0x02)?;
    run_power(&builder, 0x03)?;
    // run_voltage(&builder, 0x05)?;

    Ok(())
}

/// 电压电流
fn run_voltage(builder: &Builder, slave: u8) -> Result<()> {
    println!("\n----\nstart votage: \n");
    let request = Voltage::request(slave);
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
    // let mock = RelayMock::new(0x03, RelayMode::Read);
    let mock = RelayMock::new(slave, RelayMode::ONOFF(2));

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
