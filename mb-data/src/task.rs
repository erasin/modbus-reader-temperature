//! task 序列，任务列表
use std::time::Duration;

use mb::voltage::Verify;
use serde::{Deserialize, Serialize};

/// 序列
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Task {
    /// 序列名称
    pub title: String,
    /// ab 面
    pub ab: AB,

    /// 温度
    pub temperature: u16,

    /// 电流上下限制
    pub voltage_verify: Verify,

    /// 电源设置
    pub power: PowerConfig,

    /// 时序表
    pub count_time: Duration,
    /// 时序循环次数
    pub task_loop: u32,
    /// 时序内容
    pub items: Vec<TaskItem>,

    #[serde(skip_deserializing)]
    pub product: Product,
}

/// 产品
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Product {
    /// 产品名称
    pub title: String,
    /// 产品序列
    pub index: String,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, strum::AsRefStr)]
pub enum AB {
    #[default]
    A,
    B,
}

impl AB {
    pub const ALL: [AB; 2] = [AB::A, AB::B];
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct PowerConfig {
    pub mode: PowerMode,
    pub voltage: u32,
    pub current: u32,
}

#[derive(
    Debug, Clone, Copy, Default, Serialize, Deserialize, strum::AsRefStr, strum::VariantArray,
)]
pub enum PowerMode {
    #[default]
    Ac,
    Dc,
}

// 人物内容
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct TaskItem {
    pub index: usize,
    pub power_on: bool,
    pub voltage: u32,
    pub dur: Duration,
}
