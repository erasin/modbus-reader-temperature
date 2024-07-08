use std::time::Duration;

use mb::utils::print_hex;
use mb_mock::{
    power::PowerMock, relay::RelayMock, temperature::TempMock, voltage::VoltageMock, Mock,
};

pub enum Slave {
    None,
    Temp,
    Relay,
    PowerA,
    PowerB,
    Voltage,
}

fn main() -> std::io::Result<()> {
    let port_name = "/dev/ttyUSB1";
    let baudrate = 9600;
    let timeout = Duration::from_millis(1000);

    // 打开串口

    let mut port = serialport::new(port_name, baudrate)
        .timeout(timeout)
        .open()?;

    // ch340 发包限制 32为

    loop {
        let mut buffer = [0; 1024];
        match port.read(&mut buffer) {
            Ok(n) => {
                let buffer = &buffer[..n];

                let mock: Box<dyn Mock> = match &buffer[0] {
                    0x01 => Box::new(TempMock::from(buffer)),
                    0x02 => Box::new(RelayMock::from(buffer)),
                    0x03 => Box::new(PowerMock::from(buffer)),
                    0x04 => Box::new(PowerMock::from(buffer)),

                    // 4 之后都是电流电压
                    0x05 => Box::new(VoltageMock::from(buffer)),
                    _ => Box::new(VoltageMock::from(buffer)),
                };

                if buffer == mock.request().request_data().as_slice() {
                    print_hex("request", &buffer.to_vec());
                    let response = mock.response().origin();
                    port.write_all(response.as_slice())?;
                    print_hex("response", &response);

                    port.flush().unwrap();
                } else {
                    println!("接收到未知请求: {:?}", buffer);
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                // 读取超时，不进行处理
            }
            Err(e) => {
                eprintln!("读取失败: {:?}", e);
            }
        }
    }
}

// [1, 4, 60,
// 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 101, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
// 84, 203]

// [1, 4, 128,
// 0, 2, 0, 15, 0, 89, 1, 156, 0, 29, 0, 18, 0, 23, 0, 12, 0, 28, 0, 1, 0, 28, 1, 71, 0, 34, 0, 15, 0, 27, 0, 25, 0, 11, 0, 27, 0, 74, 0, 1, 0, 28, 0, 16, 0, 76, 0, 6, 0, 27, 0, 11, 0, 28, 1, 52, 0, 26, 0, 233, 0, 60, 0, 12,
// 194, 133]
