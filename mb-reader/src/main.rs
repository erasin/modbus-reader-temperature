use mb::Result;
use mb_mock::{temperature::TempMock, Mock};

fn main() -> Result<()> {
    let port_name = "/dev/ttyUSB0";
    let baudrate = 9600;

    // slave 1
    // run_voltage(port_name, baudrate);
    run_temp(port_name, baudrate)
}

/// 电压电流
fn run_voltage<T: Into<String>>(port_name: T, baudrate: u32) -> Result<()> {
    let request = mb::voltage::request(1);

    print_hex("request", &request);

    let response = mb::protocol::call(port_name, baudrate, request.clone())?;
    print_hex("response", &response);

    if response.as_slice() == request.as_slice() {
        println!("--命令执行\n");
        return Ok(());
    }

    let data = mb::voltage::response(response)?;
    println!("解析结果: {:?}", data);

    Ok(())
}

/// 温度

/// 电压电流
fn run_temp<T: Into<String>>(port_name: T, baudrate: u32) -> Result<()> {
    let mock = TempMock::new(0x02);

    let request = mock.request();

    print_hex("request", &request);

    let response = mb::protocol::call(port_name, baudrate, request.clone())?;
    print_hex("response", &response);

    if response.as_slice() == request.as_slice() {
        println!("--命令执行\n");
        return Ok(());
    }

    let data = mb::temperature::response(response)?;
    println!("解析结果: {:?}", data);

    Ok(())
}

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
