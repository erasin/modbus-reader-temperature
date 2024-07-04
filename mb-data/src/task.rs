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
    pub task_loop: u32,
    pub items: Vec<TaskItem>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum AB {
    #[default]
    A,
    B,
}

impl AB {
    pub const ALL: [AB; 2] = [AB::A, AB::B];
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PowerConfig {
    pub mode: PowerMode,
    pub voltage: u32,
    pub current: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, strum::AsRefStr, strum::VariantArray)]
pub enum PowerMode {
    #[default]
    Ac,
    Dc,
}

// 人物内容
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum TaskItem {
    #[default]
    Null,
    // 电源充饥
    Power {
        dur: Duration,
        is_on: bool,
    },
    //
}

// fn test() {
// time::Duration::days(1).whole_days()
// }
