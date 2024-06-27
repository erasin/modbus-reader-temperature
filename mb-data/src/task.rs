//! task 序列，任务列表

use std::{default, time::Duration};

use mb::voltage::Verify;
use serde::{Deserialize, Serialize};

/// 序列
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Task {
    /// 序列名称
    pub title: String,

    /// ab 面
    pub ab: AB,

    /// 电流上下限制
    pub voltage_verify: Verify,

    /// 电源设置
    pub acc: AccConfig,

    /// 时序表
    pub list: Vec<TaskItem>,
    pub list_count_time: Duration,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum AB {
    #[default]
    A,
    B,
}

impl AB {
    pub fn all() -> [AB; 2] {
        [AB::A, AB::B]
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccConfig {
    pub mode: AccMode,
    pub voltage: Duration,
    pub current: Duration,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum AccMode {
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
    Acc {
        dur: Duration,
        is_on: bool,
    },
    //
}

// fn test() {
// time::Duration::days(1).whole_days()
// }
