//! 用户定义

use mb::utils::current_timestamp;
use serde::{Deserialize, Serialize};
use strum::VariantArray;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    pub name: String,
    pub pwd: String,
    pub created_at: u64,
    pub purview: Vec<UserPurview>,
}

impl UserConfig {
    pub fn new<T: Into<String>>(name: T, pwd: T, purview: Vec<UserPurview>) -> Self {
        Self {
            name: name.into(),
            pwd: pwd.into(),
            created_at: current_timestamp(),
            purview,
        }
    }

    pub fn check_pwd(&self, pwd: String) -> bool {
        if self.pwd.is_empty() {
            return false;
        }
        self.pwd.clone().into_bytes().as_slice() == pwd.into_bytes().as_slice()
    }
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            name: "root".into(),
            pwd: "9600".into(),
            created_at: 0,
            purview: UserPurview::VARIANTS.to_vec(),
        }
    }
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    strum::EnumIter,
    strum::Display,
    strum::VariantArray,
    PartialEq,
    Eq,
)]
pub enum UserPurview {
    #[strum(to_string = "用户管理")]
    UserManager,

    #[strum(to_string = "系统设置")]
    SysSetting,

    #[strum(to_string = "程序编辑")]
    Programs,

    #[strum(to_string = "运行")]
    Run,
}
