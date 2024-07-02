use godot::{
    engine::{Button, IPanelContainer, Label, LineEdit, PanelContainer, Popup},
    prelude::*,
};
use strum::AsRefStr;

use mb_data::{
    db::{get_db, user::TableUser},
    user::UserConfig,
};

use super::my_global::{self, get_global_config, MyGlobal};

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

        let mut pwd = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::Pwd.as_ref());
        pwd.connect("text_submitted".into(), self.base().callable("on_pwd"));

        let mut submit = self
            .base()
            .get_node_as::<Button>(UniqueName::Submit.as_ref());

        submit.connect("pressed".into(), self.base().callable("on_submit"));
    }
}

#[godot_api]
impl LoginView {
    #[func]
    fn on_pwd(&mut self, _text: String) {
        self.on_submit();
    }

    #[func]
    fn on_submit(&mut self) {
        let name = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::Name.as_ref());

        let pwd = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::Pwd.as_ref());
        let mut err = self.base().get_node_as::<Label>(UniqueName::Error.as_ref());

        let user_name = name.get_text().to_string().trim().to_string();
        let user_pwd = pwd.get_text().to_string().trim().to_string();

        // let user = UserConfig::new(user_name, user_pwd);

        let db = get_db().lock().unwrap();
        let user = match TableUser::get(&db, user_name.clone()) {
            Ok(user) => user,
            Err(e) => {
                log::error!("用户 {user_name} 登录错误：{e}");
                UserConfig::default()
            }
        };

        if user_name != user.name || !user.check_pwd(user_pwd) {
            err.set_text("账户或密码错误，请重试！".into_godot());
            return;
        }

        // ok
        let mut my_global = MyGlobal::singleton();
        my_global.bind_mut().set_login(user);

        // close
        if let Some(mut win) = self.base_mut().get_window() {
            win.emit_signal("close_requested".into(), &[]);
        }
    }
}

impl LoginView {}

#[derive(AsRefStr, Debug)]
#[strum(prefix = "%")]
enum UniqueName {
    Name,
    Pwd,
    Submit,
    Error,
}
