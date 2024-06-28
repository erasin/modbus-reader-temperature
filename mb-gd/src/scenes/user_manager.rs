use godot::{
    engine::{Button, IPanelContainer, LineEdit, OptionButton, PanelContainer},
    prelude::*,
};
use strum::AsRefStr;

use mb_data::user::UserConfig;

#[derive(GodotClass)]
#[class(init,base=PanelContainer)]
pub struct UserManagerView {
    /// 用户列表
    users: Vec<UserConfig>,

    base: Base<PanelContainer>,
}

#[godot_api]
impl IPanelContainer for UserManagerView {
    fn ready(&mut self) {
        godot_print!("user manager ready");

        // 添加权限list and connect

        // 设置用户list
    }
}

#[godot_api]
impl UserManagerView {}

impl UserManagerView {}

#[derive(AsRefStr, Debug)]
#[strum(prefix = "%")]
enum UniqueName {
    Name,
    Pwd,
    Purview,
    Submit,

    UserList,
}
