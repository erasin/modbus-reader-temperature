use mb::{
    protocol::{Function, FunctionCode},
    voltage::Voltage,
};
use rand::Rng;

use crate::Mock;

pub struct VoltageMock {
    slave: u8,
}

impl VoltageMock {
    pub fn new(slave: u8) -> Self {
        VoltageMock { slave }
    }
}

impl From<&[u8]> for VoltageMock {
    fn from(value: &[u8]) -> Self {
        VoltageMock::new(value[0])
    }
}

impl Mock for VoltageMock {
    fn request(&self) -> Function {
        // let request: [u8; 8] = [0x01, 0x04, 0x00, 0x00, 0x00, 0x1E, 0x70, 0x02];
        Voltage::request(self.slave)
    }

    fn response(&self) -> Function {
        let code = FunctionCode::ReadInputRegisters;
        let data = generate_response_voltage();
        // let data = static_response();

        Function::new(self.slave, code, data)
    }
}

#[allow(dead_code)]
fn static_response() -> Vec<u16> {
    let data: Vec<u16> = vec![
        0, 0, // 1
        10, 10, // 2
        20, 20, // 3
        30, 30, // 4
        40, 40, // 5
        50, 50, // 6
        0, 10, // 7
        20, 40, // 8
        50, 0, // 9
        0, 0, // 10
        25, 10, // 11
        0, 0, // 12
        0, 0, // 13
        0, 0, // 14
        0, 0, // 15
    ];

    data.iter()
        .enumerate()
        .map(|(i, &x)| if i % 2 == 0 { x * 1000 } else { x })
        .collect()
}

#[allow(dead_code)]
fn generate_response_voltage() -> Vec<u16> {
    let mut rng = rand::thread_rng();

    // 生成 15 对电压和电流数据
    let mut data: Vec<u16> = Vec::with_capacity(30);

    (0..=14)
        .map(|_| {
            let voltage = match rng.gen_range(0..3) {
                0 => rng.gen_range(0..25),
                1 => rng.gen_range(25..30),
                _ => rng.gen_range(30..100),
            } * 1000;
            let current = match rng.gen_range(0..3) {
                0 => rng.gen_range(0..20),
                1 => rng.gen_range(20..100),
                _ => rng.gen_range(100..500),
            } * 1000;

            (voltage as u16, current as u16)
        })
        .for_each(|(voltage, current)| {
            data.push(voltage);
            data.push(current);
        });

    data
}

#[cfg(test)]
mod test {

    #[test]
    fn gen_data() {
        let r = super::generate_response_voltage();
        // assert_eq!(30, r.len());
        println!("{:?}", r);
        r.iter().for_each(|x| print!("{:02X} ", x));
    }
}
