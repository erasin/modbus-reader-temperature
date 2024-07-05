use godot::{
    engine::{Button, IPanelContainer, Label, LineEdit, PanelContainer},
    prelude::*,
};
use strum::AsRefStr;

use mb_data::{
    db::{get_db, user::TableUser},
    user::UserConfig,
};

use crate::define_get_nodes;

use super::my_global::MyGlobal;

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
        self.get_name_node().grab_focus();

        self.get_pwd_node()
            .connect("text_submitted".into(), self.base().callable("on_pwd"));

        self.get_submit_node()
            .connect("pressed".into(), self.base().callable("on_submit"));
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
        let name_node = self.get_name_node();
        let pwd_node = self.get_pwd_node();
        let mut err_node = self.get_error_node();

        let user_name = name_node.get_text().to_string().trim().to_string();
        let user_pwd = pwd_node.get_text().to_string().trim().to_string();

        // let user = UserConfig::new(user_name, user_pwd);

        let user = {
            let db = get_db().lock().unwrap();
            let user = match TableUser::get(&db, user_name.clone()) {
                Ok(user) => user,
                Err(e) => {
                    log::error!("用户 {user_name} 登录错误：{e}");
                    UserConfig::default()
                }
            };
            user
        };

        if user_name != user.name || !user.check_pwd(user_pwd) {
            err_node.set_text("账户或密码错误，请重试！".into_godot());
            return;
        }

        // ok
        let mut my_global = MyGlobal::singleton();
        my_global.bind_mut().set_login(user);
        // my_global.emit_signal("login_updated".into(), &[]);

        // close
        if let Some(mut win) = self.base_mut().get_window() {
            win.emit_signal("close_requested".into(), &[]);
        }
    }
}

impl LoginView {
    define_get_nodes![
        (get_name_node, UniqueName::Name, LineEdit),
        (get_pwd_node, UniqueName::Pwd, LineEdit),
        (get_submit_node, UniqueName::Submit, Button),
        (get_error_node, UniqueName::Error, Label),
    ];
}

#[derive(AsRefStr, Debug)]
#[strum(prefix = "%")]
enum UniqueName {
    Name,
    Pwd,
    Submit,
    Error,
}
