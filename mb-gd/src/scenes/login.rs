use godot::{
    engine::{IPanelContainer, PanelContainer},
    prelude::*,
};
use strum::AsRefStr;

use mb_data::user::UserConfig;

#[derive(GodotClass)]
#[class(init,base=PanelContainer)]
pub struct LoginView {
    /// 用户列表
    users: UserConfig,

    base: Base<PanelContainer>,
}

#[godot_api]
impl IPanelContainer for LoginView {
    fn ready(&mut self) {
        godot_print!("user ready");
    }
}

#[godot_api]
impl LoginView {}

impl LoginView {}

#[derive(AsRefStr, Debug)]
#[strum(prefix = "%")]
enum UniqueName {
    Name,
    Pwd,
    Submit,
}
