//! modbus 协议相关

use core::fmt;
use std::{fmt::Display, thread, time::Duration};

use crate::Result;
use serialport::{SerialPort, SerialPortType};

pub fn call<T: Into<String>>(port_name: T, baudrate: u32, request: Vec<u8>) -> Result<Vec<u8>> {
    let mut port = serialport::new(port_name.into(), baudrate)
        .timeout(Duration::from_secs(5))
        .open()?;

    let _i = port.write(&request)?;

    port.flush().unwrap();
    thread::sleep(Duration::from_millis(100));

    let mut response = vec![0u8; 1024];
    let n = read_full_response(&mut port, &mut response)?;
    port.flush().unwrap();

    // 只保留实际读取到的字节
    response.truncate(n);

    Ok(response)
}

// ch340 32位字节缓存读取
fn read_full_response(port: &mut Box<dyn SerialPort>, buffer: &mut Vec<u8>) -> Result<usize> {
    let mut total_read = 0;
    let mut read_buffer = [0u8; 32];

    loop {
        match port.read(&mut read_buffer) {
            Ok(n) => {
                // println!("--- {n} {:?}", read_buffer);
                buffer[total_read..total_read + n].copy_from_slice(&read_buffer[..n]);
                total_read += n;
                if n < read_buffer.len() {
                    break; // 读取完成
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                break; // 读取超时，退出循环
            }
            Err(e) => return Err(Box::new(e)),
        }
    }

    Ok(total_read)
}

// 解析Modbus响应数据，将其转换为Vec<u16>
pub fn parse_modbus_response(response: &[u8]) -> Option<Vec<u16>> {
    if response.len() < 5 {
        return None; // 响应数据太短
    }

    let byte_count = response[2] as usize;
    if response.len() < 3 + byte_count || byte_count % 2 != 0 {
        return None; // 数据长度不匹配
    }

    let mut result = Vec::with_capacity(byte_count / 2);
    for i in 0..(byte_count / 2) {
        let high_byte = response[3 + 2 * i] as u16;
        let low_byte = response[3 + 2 * i + 1] as u16;
        result.push((high_byte << 8) | low_byte);
    }

    Some(result)
}

pub fn parse_modbus_request(request: &[u8]) -> Option<Vec<u16>> {
    if request.len() < 4 {
        return None; // 响应数据太短
    }

    // 掐头去尾
    let byte_count = request.len() - 4 as usize;
    if byte_count < 2 || byte_count % 2 != 0 {
        return None; // 数据长度不匹配
    }
    println!("byte_count: {}, {}", byte_count, request.len());

    let mut result = Vec::with_capacity(byte_count / 2);
    for i in 0..(byte_count / 2) {
        let high_byte = request[2 + 2 * i] as u16;
        let low_byte = request[2 + 2 * i + 1] as u16;
        result.push((high_byte << 8) | low_byte);
    }

    Some(result)
}

/// 请求结构
pub fn request(slave: u8, code: FunctionCode, params: Vec<u16>) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();

    for i in params {
        data.push((i >> 8) as u8); // 高位字节
        data.push(i as u8); // 低位字节
    }

    let mut response: Vec<u8> = vec![slave, code.value()];
    response.extend_from_slice(&data);

    // 计算并添加 CRC 校验码
    let crc = calculate_crc(&response);
    response.push(crc as u8);
    response.push((crc >> 8) as u8);

    response
}

pub fn response(slave: u8, code: FunctionCode, params: Vec<u16>) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();

    for i in params {
        data.push((i >> 8) as u8); // 高位字节
        data.push(i as u8); // 低位字节
    }

    let mut response: Vec<u8> = vec![slave, code.value(), data.len() as u8];
    response.extend_from_slice(&data);

    // 计算并添加 CRC 校验码
    let crc = calculate_crc(&response);
    response.push(crc as u8);
    response.push((crc >> 8) as u8);

    response
}

/// 计算 Modbus RTU CRC 校验码
pub fn calculate_crc(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;

    for &byte in data {
        crc ^= byte as u16;
        for _ in 0..8 {
            if crc & 0x0001 != 0 {
                crc >>= 1;
                crc ^= 0xA001;
            } else {
                crc >>= 1;
            }
        }
    }

    crc
}

/// A Modbus function code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionCode {
    /// Modbus Function Code: `01` (`0x01`).
    ReadCoils,
    /// Modbus Function Code: `02` (`0x02`).
    ReadDiscreteInputs,

    /// Modbus Function Code: `05` (`0x05`).
    WriteSingleCoil,
    /// Modbus Function Code: `06` (`0x06`).
    WriteSingleRegister,

    /// Modbus Function Code: `03` (`0x03`).
    ReadHoldingRegisters,
    /// Modbus Function Code: `04` (`0x04`).
    ReadInputRegisters,

    /// Modbus Function Code: `15` (`0x0F`).
    WriteMultipleCoils,
    /// Modbus Function Code: `16` (`0x10`).
    WriteMultipleRegisters,

    /// Modbus Function Code: `22` (`0x16`).
    MaskWriteRegister,

    /// Modbus Function Code: `23` (`0x17`).
    ReadWriteMultipleRegisters,

    /// Custom Modbus Function Code.
    Custom(u8),
}

impl FunctionCode {
    /// Create a new [`FunctionCode`] with `value`.
    #[must_use]
    pub const fn new(value: u8) -> Self {
        match value {
            0x01 => FunctionCode::ReadCoils,
            0x02 => FunctionCode::ReadDiscreteInputs,
            0x05 => FunctionCode::WriteSingleCoil,
            0x06 => FunctionCode::WriteSingleRegister,
            0x03 => FunctionCode::ReadHoldingRegisters,
            0x04 => FunctionCode::ReadInputRegisters,
            0x0F => FunctionCode::WriteMultipleCoils,
            0x10 => FunctionCode::WriteMultipleRegisters,
            0x16 => FunctionCode::MaskWriteRegister,
            0x17 => FunctionCode::ReadWriteMultipleRegisters,
            code => FunctionCode::Custom(code),
        }
    }

    /// Get the [`u8`] value of the current [`FunctionCode`].
    #[must_use]
    pub const fn value(self) -> u8 {
        match self {
            FunctionCode::ReadCoils => 0x01,
            FunctionCode::ReadDiscreteInputs => 0x02,
            FunctionCode::WriteSingleCoil => 0x05,
            FunctionCode::WriteSingleRegister => 0x06,
            FunctionCode::ReadHoldingRegisters => 0x03,
            FunctionCode::ReadInputRegisters => 0x04,
            FunctionCode::WriteMultipleCoils => 0x0F,
            FunctionCode::WriteMultipleRegisters => 0x10,
            FunctionCode::MaskWriteRegister => 0x16,
            FunctionCode::ReadWriteMultipleRegisters => 0x17,
            FunctionCode::Custom(code) => code,
        }
    }
}

impl Display for FunctionCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value().fmt(f)
    }
}

/// 获取当前串口列表
pub fn get_ports() -> Vec<String> {
    let mut list = match serialport::available_ports() {
        Ok(ports) => {
            if ports.is_empty() {
                Vec::new()
            } else {
                ports
                    .iter()
                    .filter(|info| info.port_type != SerialPortType::Unknown)
                    .map(|info| info.port_name.clone())
                    .collect()
            }
        }
        Err(_e) => Vec::new(),
    };

    list.push("test".to_string());
    list
}

#[cfg(not(target_os = "windows"))]
fn default_port_name() -> String {
    "/dev/ttyUSB0".to_owned()
}

#[cfg(target_os = "windows")]
fn default_port_name() -> String {
    "COM1".to_owned()
}
