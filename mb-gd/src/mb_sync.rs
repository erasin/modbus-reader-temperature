use mb::protocol::Builder;
use mb::relay::{Relay, RelayData, RelayMode};
use mb::temperature::{Temperature, TemperatureData, TemperatureMode};
use mb::voltage::{Voltage, VoltageData};
use mb::Result;

use mb_data::config::{RelayConfig, TemperatureConfig, VoltageConfig};

use crate::data::AB;

/// 获取电压电流
pub fn get_voltage_data(config: &VoltageConfig, slave: u8) -> Result<VoltageData> {
    let port_name = config.serial_port.port.clone();
    let baudrate = config.serial_port.baudrate.into();
    // let slave = config.voltage.salve_a;

    let builder = Builder::new(port_name, baudrate);
    let request = Voltage::request(slave);
    let response = builder.call(&request)?;
    response.try_into()
}

// 获取温度
pub fn get_temperature(config: &TemperatureConfig, ab: AB) -> Result<TemperatureData> {
    let port_name = config.serial_port.port.clone();
    let baudrate = config.serial_port.baudrate.into();
    let slave = config.slave;

    let mode = if ab.is_a() {
        TemperatureMode::Temp1
    } else {
        TemperatureMode::Temp2
    };

    let builder = Builder::new(port_name, baudrate);
    let request = Temperature::request(slave, &mode);
    let response = builder.call(&request)?;
    response.try_into()
}

// 设置取温度
pub fn set_temperature(config: &TemperatureConfig, ab: AB, temp: u16) -> Result<TemperatureData> {
    let port_name = config.serial_port.port.clone();
    let baudrate = config.serial_port.baudrate.into();
    let slave = config.slave;

    let mode = if ab.is_a() {
        TemperatureMode::Set1(temp)
    } else {
        TemperatureMode::Set2(temp)
    };

    let builder = Builder::new(port_name, baudrate);
    let request = Temperature::request(slave, &mode);
    let response = builder.call(&request)?;
    response.try_into()
}

// 获取继电器开关

pub fn get_relay(config: &RelayConfig, ab: AB) -> Result<RelayData> {
    let port_name = config.serial_port.port.clone();
    let baudrate = config.serial_port.baudrate.into();
    let slave = config.slave;

    let builder = Builder::new(port_name, baudrate);
    let request = Relay::request(slave, &RelayMode::Read);
    let response = builder.call(&request)?;
    response.try_into()
}

// 设定继电器
pub fn set_relay(config: &RelayConfig, ab: AB, mode: &RelayMode) -> Result<RelayData> {
    let port_name = config.serial_port.port.clone();
    let baudrate = config.serial_port.baudrate.into();
    let slave = config.slave;

    let builder = Builder::new(port_name, baudrate);
    let request = Relay::request(slave, mode);
    let response = builder.call(&request)?;
    response.try_into()
}
