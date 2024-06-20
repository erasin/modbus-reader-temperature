#![allow(dead_code)]

use mb::{protocol::Builder, relay::RelayMode, temperature::TempMode, utils::print_hex, Result};
use mb_mock::{relay::RelayMock, temperature::TempMock, Mock};

fn main() -> Result<()> {
    let port_name = "/dev/ttyUSB0";
    let baudrate = 9600;

    let builder = Builder::new(port_name, baudrate);

    // slave 1
    run_voltage(&builder)?;
    run_temp(&builder)?;
    run_relay(&builder)?;

    Ok(())
}

/// 电压电流
fn run_voltage(builder: &Builder) -> Result<()> {
    println!("\n----\nstart votage: \n");
    let request = mb::voltage::request(0x01);
    print_hex("request", &request.request_data());

    let response = builder.call(&request)?;
    print_hex("response", &response.response_data());

    if response == request {
        println!("--命令执行\n");
        return Ok(());
    }

    let data = mb::voltage::response(&response)?;
    println!("解析结果:\n{:?}", data);

    Ok(())
}

/// 温度
fn run_temp(builder: &Builder) -> Result<()> {
    println!("\n----\nstart temp: \n");
    let mock = TempMock::new(0x02, TempMode::Temp1);

    let request = mock.request();
    print_hex("request", &request.request_data());

    let response = builder.call(&request)?;
    print_hex("response", &response.response_data());

    if response == request {
        println!("命令执行\n");
        return Ok(());
    }

    let data = mb::temperature::response(&response)?;
    println!("解析结果:\n{:?}", data);

    Ok(())
}

fn run_relay(builder: &Builder) -> Result<()> {
    println!("\n----\nstart relay: \n");
    // let mock = RelayMock::new(0x03, RelayMode::Read);
    let mock = RelayMock::new(0x03, RelayMode::ONOFF(2));

    let request = mock.request();
    print_hex("request", &request.request_data());

    let response = builder.call(&request)?;
    print_hex("response", &response.response_data());

    if response == request {
        println!("命令执行\n");
        return Ok(());
    }

    let data = mb::relay::response(&response)?;
    println!("解析结果:\n{}", data);

    Ok(())
}
