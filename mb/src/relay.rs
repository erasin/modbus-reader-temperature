//! 继电器

use crate::protocol::calculate_crc;

/// 继电器
/// 1 2 奇偶校验
/// 3-5 波特率
/// 6-10 地址位 10 -> 6 二进制
/// 继电器 0 ，参数二进制控制开关(8位) 0b00000000;
fn relay_sender() -> Vec<u8> {
    // 写入
    let slave = 0x02;
    let mode = (0x06, 0, 0b00000000);

    // 读取
    // let mode = (0x03, 0, 1);

    let mut data: Vec<u8> = Vec::with_capacity(4);

    let key = mode.1;
    data.push((key >> 8) as u8); // 高位字节
    data.push(key as u8); // 低位字节

    let wf = mode.2;

    data.push((wf >> 8) as u8); // 高位字节
    data.push(wf as u8); // 低位字节

    let mut response: Vec<u8> = vec![0x01, mode.0];
    response.extend_from_slice(&data);

    // 计算并添加 CRC 校验码
    let crc = calculate_crc(&response);
    response.push(crc as u8);
    response.push((crc >> 8) as u8);

    response
}
