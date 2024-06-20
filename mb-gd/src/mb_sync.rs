use mb::voltage::VoltageData;
use mb::Result;

use crate::data::config::Config;

/// 获取电压电流
pub fn get_voltage_data(config: &Config) -> Result<VoltageData> {
    let port_name = config.port();
    let baudrate = config.baudrate().to_u32();

    let request = mb::voltage::request(1);
    let response = mb::protocol::call(&port_name, baudrate, request.clone())?;

    mb::voltage::response(response)
}
