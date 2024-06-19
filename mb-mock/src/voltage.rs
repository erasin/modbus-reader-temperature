use mb::protocol::calculate_crc;
use rand::Rng;

use crate::Mock;

pub struct VoltageMock {
    slave: u8,
}

impl VoltageMock {
    pub fn new(slave: u8) -> Self {
        VoltageMock { slave }
    }

    pub fn from_request(request: &[u8]) -> Self {
        VoltageMock::new(0x01)
    }
}

impl Mock for VoltageMock {
    fn request(&self) -> Vec<u8> {
        // let request: [u8; 8] = [0x01, 0x04, 0x00, 0x00, 0x00, 0x1E, 0x70, 0x02];
        let request = mb::voltage::request(self.slave);
        request

        // 01040000001E7002
        // let response: [u8; 53] = [
        //     0x01, 0x04, 0x36, // 响应头，设备地址 + 功能码 + 数据字节数
        //     // 模拟数据部分
        //     0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00, 0x05, 0x00, 0x06, 0x00, 0x07, 0x00,
        //     0x08, 0x00, 0x09, 0x00, 0x0A, 0x00, 0x0B, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x0E, 0x00, 0x0F,
        //     0x00, 0x10, 0x00, 0x11, 0x00, 0x12, 0x00, 0x13, 0x00, 0x14, 0x00, 0x15, 0x00, 0x16, 0x00,
        //     0x17, 0x00, 0x18, // CRC 校验
        //     0xFA, 0xCE, 0x01, 0x04, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        //     0x65, 0x28, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        //     0x00, 0x00, 0x00, 0x00, 0x00, 0x54, 0xCB,
        // ];

        // let response = generate_modbus_response();
    }

    fn response(&self) -> Vec<u8> {
        let response = generate_response_voltage(self.slave);

        response
    }
}

fn generate_response_voltage(slave: u8) -> Vec<u8> {
    let mut rng = rand::thread_rng();

    // 生成 15 对电压和电流数据
    let mut data: Vec<u8> = Vec::with_capacity(30);
    for _ in 0..=14 {
        let voltage = match rng.gen_range(0..3) {
            0 => rng.gen_range(0..25),
            1 => rng.gen_range(25..30),
            _ => rng.gen_range(30..100),
        } * 1000;
        let current = match rng.gen_range(0..3) {
            0 => rng.gen_range(0..20),
            1 => rng.gen_range(20..100),
            _ => rng.gen_range(100..500),
        };

        data.push((voltage >> 8) as u8); // 高位字节
        data.push(voltage as u8); // 低位字节
        data.push((current >> 8) as u8); // 高位字节
        data.push(current as u8); // 低位字节
    }

    // 构建 Modbus 响应
    let mut response: Vec<u8> = vec![slave, 0x04, data.len() as u8];
    response.extend_from_slice(&data);

    // 计算并添加 CRC 校验码
    let crc = calculate_crc(&response);
    response.push(crc as u8);
    response.push((crc >> 8) as u8);

    response
}

#[cfg(test)]
mod test {

    #[test]
    fn gen() {
        let r = super::generate_response_voltage(1);
        assert_eq!(65, r.len());
        println!("{:?}", r);
        r.iter().for_each(|x| print!("{:02X} ", x));
    }
}
