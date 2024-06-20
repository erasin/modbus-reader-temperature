//! modbus 协议相关实现

use core::fmt;
use std::{fmt::Display, thread, time::Duration};

use crate::{error::Error, Result};
use serialport::{SerialPort, SerialPortType};

#[derive(Debug, Clone)]
pub struct Builder {
    pub port_name: String,
    pub baudrate: u32,
}

impl Builder {
    pub fn new<T: Into<String>>(port_name: T, baudrate: u32) -> Self {
        Self {
            port_name: port_name.into(),
            baudrate,
        }
    }

    pub fn call(&self, request: &FunRequest) -> Result<FunResponse> {
        let mut port = serialport::new(self.port_name.clone(), self.baudrate)
            .timeout(Duration::from_secs(5))
            .open()?;

        let _i = port.write(&request.request_data())?;

        port.flush().unwrap();
        thread::sleep(Duration::from_millis(100));

        let mut response = vec![0u8; 1024];
        let n = read_full_response(&mut port, &mut response)?;
        port.flush().unwrap();

        // 只保留实际读取到的字节
        response.truncate(n);

        let response = Function::parse_response(&response)?;

        Ok(response)
    }
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

/// Modbus Function
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    slave: u8,
    code: FunctionCode,
    data: Vec<u16>,
}

pub type FunRequest = Function;
pub type FunResponse = Function;

impl Function {
    pub fn new(slave: u8, code: FunctionCode, data: Vec<u16>) -> Self {
        Self { slave, code, data }
    }

    pub fn slave(&self) -> u8 {
        self.slave
    }

    pub fn code(&self) -> FunctionCode {
        self.code
    }

    pub fn data(&self) -> Vec<u16> {
        self.data.clone()
    }

    // 解析Modbus响应数据，将其转换为 Function
    pub fn parse_response(response: &[u8]) -> Result<Self> {
        if response.len() < 5 {
            return Err(Box::new(Error::DataShort)); // 响应数据太短
        }

        let byte_count = response[2] as usize;
        if response.len() < 3 + byte_count || byte_count % 2 != 0 {
            return Err(Box::new(Error::DataLenError)); // 数据长度不匹配
        }

        let mut result = Vec::with_capacity(byte_count / 2);
        for i in 0..(byte_count / 2) {
            let high_byte = response[3 + 2 * i] as u16;
            let low_byte = response[3 + 2 * i + 1] as u16;
            result.push((high_byte << 8) | low_byte);
        }

        let fp = Function {
            slave: response[0],
            code: FunctionCode::new(response[1]),
            data: result,
        };

        Ok(fp)
    }

    /// 将数据解析为请求 Function
    pub fn parse_request(request: &[u8]) -> Result<Self> {
        if request.len() < 4 {
            return Err(Box::new(Error::DataShort)); // 响应数据太短
        }

        // 掐头去尾
        let byte_count = request.len() - 4 as usize;
        if byte_count < 2 || byte_count % 2 != 0 {
            return Err(Box::new(Error::DataLenError)); // 数据长度不匹配
        }
        // println!("byte_count: {}, {}", byte_count, request.len());

        let mut result = Vec::with_capacity(byte_count / 2);
        for i in 0..(byte_count / 2) {
            let high_byte = request[2 + 2 * i] as u16;
            let low_byte = request[2 + 2 * i + 1] as u16;
            result.push((high_byte << 8) | low_byte);
        }

        let fp = Function {
            slave: request[0],
            code: FunctionCode::new(request[1]),
            data: result,
        };

        Ok(fp)
    }

    /// 生成请求数据
    pub fn request_data(&self) -> Vec<u8> {
        let len = self.data.len() * 2;
        let mut data: Vec<u8> = Vec::with_capacity(len);

        for &i in &self.data {
            data.push((i >> 8) as u8); // 高位字节
            data.push(i as u8); // 低位字节
        }

        let mut response: Vec<u8> = vec![self.slave, self.code.value()];
        response.extend_from_slice(&data);

        // 计算并添加 CRC 校验码
        let crc = calculate_crc(&response);
        response.push(crc as u8);
        response.push((crc >> 8) as u8);

        response
    }

    /// 生成返回数据
    pub fn response_data(&self) -> Vec<u8> {
        let len = self.data.len() * 2;
        let mut data: Vec<u8> = Vec::with_capacity(len);

        for &i in &self.data {
            data.push((i >> 8) as u8); // 高位字节
            data.push(i as u8); // 低位字节
        }

        let mut response: Vec<u8> = vec![self.slave, self.code.value(), data.len() as u8];
        response.extend_from_slice(&data);

        // 计算并添加 CRC 校验码
        let crc = calculate_crc(&response);
        response.push(crc as u8);
        response.push((crc >> 8) as u8);

        response
    }
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
    /// 读取线圈: `01` (`0x01`).
    ReadCoils,
    /// 读取离散输入: `02` (`0x02`).
    ReadDiscreteInputs,

    /// 读取保持寄存器: `03` (`0x03`).
    ReadHoldingRegisters,
    /// 读取输入寄存器: `04` (`0x04`).
    ReadInputRegisters,

    /// 写入单线圈: `05` (`0x05`).
    WriteSingleCoil,
    /// 写入单个寄存器: `06` (`0x06`).
    WriteSingleRegister,

    // 诊断（限 serial port ) `08`（`0x08`)
    // 获取通信事件计时器 (only serial port): `11` (`0x0B`)
    // ----
    /// 写入多个线圈: `15` (`0x0F`).
    WriteMultipleCoils,
    /// 写入多个寄存器: `16` (`0x10`).
    WriteMultipleRegisters,

    /// 掩码写入寄存器: `22` (`0x16`).
    MaskWriteRegister,

    /// 读/写多个寄存器: `23` (`0x17`).
    ReadWriteMultipleRegisters,

    // 读取设备标识 `43/14` (`0x2B/0x0E`)
    // ----
    /// 自定义 Function Code.
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
pub fn default_port_name() -> String {
    "/dev/ttyUSB0".to_owned()
}

#[cfg(target_os = "windows")]
pub fn default_port_name() -> String {
    "COM1".to_owned()
}
