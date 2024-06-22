use mb::protocol::Builder;
use mb::temperature::{Temperature, TemperatureData, TemperatureMode};
use mb::voltage::{Voltage, VoltageData};
use mb::Result;

use mb_data::config::Config;

use crate::data::AB;

/// 获取电压电流
pub fn get_voltage_data(config: &Config) -> Result<VoltageData> {
    let port_name = config.voltage.serial_port.port.clone();
    let baudrate = config.voltage.serial_port.baudrate.to_u32();
    // let slave = config.voltage.salve_a;

    let builder = Builder::new(port_name, baudrate);
    let request = Voltage::request(0x01);
    let response = builder.call(&request)?;
    response.try_into()
}

// 获取温度
pub fn get_temperature(config: &Config, ab: AB) -> Result<TemperatureData> {
    let port_name = config.temperature.serial_port.port.clone();
    let baudrate = config.temperature.serial_port.baudrate.to_u32();
    let slave = config.temperature.slave;

    let builder = Builder::new(port_name, baudrate);
    let request = Temperature::request(slave, &TemperatureMode::Temp1);
    let response = builder.call(&request)?;
    response.try_into()
}

// 获取继电器开关

// 设定继电器
