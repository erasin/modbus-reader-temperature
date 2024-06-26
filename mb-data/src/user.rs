//! 用户定义

use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserConfig {
    pub name: String,
    pub pwd: String,
    pub created_at: u64,
}

impl UserConfig {
    pub fn check_pwd(&self, pwd: String) -> bool {
        if self.pwd.is_empty() {
            return false;
        }
        self.pwd.clone().into_bytes().as_slice() == pwd.into_bytes().as_slice()
    }
}
