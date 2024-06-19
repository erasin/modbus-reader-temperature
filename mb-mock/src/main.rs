use std::time::Duration;

use mb_mock::{temperature::TempMock, Mock};

fn main() -> std::io::Result<()> {
    let port_name = "/dev/ttyUSB1";
    let baudrate = 9600;

    let timeout = Duration::from_millis(1000);

    // 打开串口

    let mut port = serialport::new(port_name, baudrate)
        .timeout(Duration::from_millis(1000))
        .open()?;

    // ch340 发包限制 32为
    // let mock = VoltageMock::new(0x01);
    let mock = TempMock::new(0x02);

    loop {
        let mut buffer = [0; 1024];
        match port.read(&mut buffer) {
            Ok(n) => {
                let buffer = &buffer[..n];

                if buffer == mock.request().as_slice() {
                    print_hex("request", &buffer.to_vec());
                    let response = mock.response();
                    port.write_all(response.as_slice())?;
                    print_hex("response", &&response);
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
fn print_hex<T: AsRef<str>>(name: T, data: &Vec<u8>) {
    let n = data.len();
    let hex = data
        .iter()
        .map(|v| format!("{:02X} ", v))
        .fold(String::new(), |mut x, y| {
            x.push_str(&y);
            x
        });

    println!("{} ({n}): {:?} \n{hex} \n", name.as_ref(), &data);
}
