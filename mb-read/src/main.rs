#![allow(dead_code)]

use mb::{relay::RelayMode, temperature::TempMode, utils::print_hex, Result};
use mb_mock::{relay::RelayMock, temperature::TempMock, Mock};

fn main() -> Result<()> {
    let port_name = "/dev/ttyUSB0";
    let baudrate = 9600;

    // slave 1
    run_voltage(port_name, baudrate)?;
    // run_temp(port_name, baudrate)?;
    // run_relay(port_name, baudrate)?;

    Ok(())
}

/// 电压电流
fn run_voltage<T: Into<String>>(port_name: T, baudrate: u32) -> Result<()> {
    println!("\n----\nstart votage: \n");
    let request = mb::voltage::request(1);

    print_hex("request", &request);

    let response = mb::protocol::call(port_name, baudrate, request.clone())?;
    print_hex("response", &response);

    if response.as_slice() == request.as_slice() {
        println!("--命令执行\n");
        return Ok(());
    }

    let data = mb::voltage::response(response)?;
    println!("解析结果:\n{:?}", data);

    Ok(())
}

/// 温度
fn run_temp<T: Into<String>>(port_name: T, baudrate: u32) -> Result<()> {
    println!("\n----\nstart temp: \n");
    let mock = TempMock::new(0x02, TempMode::Temp1);

    let request = mock.request();

    print_hex("request", &request);

    let response = mb::protocol::call(port_name, baudrate, request.clone())?;
    print_hex("response", &response);

    if response.as_slice() == request.as_slice() {
        println!("命令执行\n");
        return Ok(());
    }

    let data = mb::temperature::response(response)?;
    println!("解析结果:\n{:?}", data);

    Ok(())
}

fn run_relay<T: Into<String>>(port_name: T, baudrate: u32) -> Result<()> {
    println!("\n----\nstart relay: \n");
    // let mock = RelayMock::new(0x03, RelayMode::Read);
    let mock = RelayMock::new(0x03, RelayMode::ONOFF(2));

    let request = mock.request();

    print_hex("request", &request);

    let response = mb::protocol::call(port_name, baudrate, request.clone())?;
    print_hex("response", &response);

    if response.as_slice() == request.as_slice() {
        println!("命令执行\n");
        return Ok(());
    }

    let data = mb::relay::response(response)?;
    println!("解析结果:\n{}", data);

    Ok(())
}
