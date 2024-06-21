use mb::protocol::Builder;
use mb::voltage::{self, VoltageData};
use mb::Result;

use mb_data::config::Config;

/// 获取电压电流
pub fn get_voltage_data(config: &Config) -> Result<VoltageData> {
    let port_name = config.port();
    let baudrate = config.baudrate().to_u32();

    let builder = Builder::new(port_name, baudrate);
    let request = voltage::request(0x01);
    let response = builder.call(&request)?;
    voltage::response(&response)
}
