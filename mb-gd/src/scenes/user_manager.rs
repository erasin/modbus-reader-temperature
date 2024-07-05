use godot::{
    engine::{
        BoxContainer, Button, CheckBox, IPanelContainer, ItemList, Label, LineEdit, PanelContainer,
    },
    prelude::*,
};
use strum::{AsRefStr, IntoEnumIterator, VariantArray};

use mb_data::{
    db::{get_db, user::TableUser},
    user::{UserConfig, UserPurview},
};

use crate::define_get_nodes;

#[derive(GodotClass)]
#[class(init,base=PanelContainer)]
pub struct UserManagerView {
    /// 用户列表
    users: Vec<UserConfig>,

    #[var]
    list: Array<GString>,

    base: Base<PanelContainer>,
}

#[godot_api]
impl IPanelContainer for UserManagerView {
    fn ready(&mut self) {
        godot_print!("user manager ready");

        // 添加权限list and connect
        let mut purview_node = self.get_purview_node();

        UserPurview::iter().into_iter().for_each(|up| {
            let mut cb = CheckBox::new_alloc();
            cb.set_text(up.to_string().into());
            purview_node.add_child(cb.upcast());
        });

        let mut user_list_node = self.get_user_list_node();
        user_list_node.connect(
            "item_selected".into(),
            self.base().callable("on_user_item_selected"),
        );

        let mut submit_btn = self.get_submit_node();
        submit_btn.connect("pressed".into(), self.base().callable("on_submit"));

        let mut delete_btn = self.get_delete_node();
        delete_btn.connect("pressed".into(), self.base().callable("on_delete"));

        self.user_item_update();

        // let user_iu = self.base().callable("on_user_item_update");
        // self.base_mut().connect("update_users".into(), user_iu);
    }

    // 手动释放
    // fn exit_tree(&mut self) {
    //     let purview = self
    //         .base()
    //         .get_node_as::<BoxContainer>(UniqueName::Purview.as_ref());

    //     purview
    //         .get_children()
    //         .iter_shared()
    //         .for_each(|node| node.free());
    // }
}

#[godot_api]
impl UserManagerView {
    #[signal]
    fn update_users();

    #[func]
    fn on_submit(&mut self) {
        let mut err_node = self.get_error_node();
        err_node.set_text("".into());

        let name_node = self.get_name_node();
        let user_name = name_node.get_text().to_string().trim().to_string();

        if user_name.is_empty() {
            err_node.set_text("用户名不可为空！".into());
            return;
        }

        let user_pwd_node = self.get_pwd_node();
        let user_pwd = user_pwd_node.get_text().to_string().trim().to_string();

        if user_pwd.is_empty() {
            err_node.set_text("密码不可为空！".into());
            return;
        }

        let mut user_purview: Vec<UserPurview> = Vec::new();

        let purview_node = self.get_purview_node();

        purview_node
            .get_children()
            .iter_shared()
            .enumerate()
            .for_each(|(index, node)| {
                let cb = node.cast::<CheckBox>();
                if cb.is_pressed() {
                    if let Some(p) = UserPurview::VARIANTS.get(index) {
                        user_purview.push(p.clone());
                    };
                }
            });

        let user = UserConfig::new(user_name, user_pwd, user_purview);

        {
            let db = get_db().lock().unwrap();
            match TableUser::set(&db, &user) {
                Ok(_) => {}
                Err(e) => {
                    log::error!("保存用户失败 {e}");
                }
            };
        }

        self.user_item_update();
    }

    #[func]
    fn on_delete(&mut self) {
        let mut name = self.get_name_node();
        let user_name = name.get_text().to_string().trim().to_string();

        {
            let db = get_db().lock().unwrap();
            match TableUser::delete(&db, user_name) {
                Ok(_) => {}
                Err(e) => {
                    log::error!("删除用户失败：{e}");
                    return;
                }
            };
        }

        self.user_item_update();

        let mut pwd = self.get_pwd_node();
        name.set_text("".into());
        pwd.set_text("".into());

        let purview = self.get_purview_node();
        purview
            .get_children()
            .iter_shared()
            .enumerate()
            .for_each(|(index, node)| {
                let mut cb = node.cast::<CheckBox>();
                cb.set_pressed(false);
            });
    }

    #[func]
    fn on_user_item_selected(&mut self, index: u32) {
        let index = index as usize;
        let user = match self.users.get(index) {
            Some(user) => user.clone(),
            None => {
                return;
            }
        };

        let mut name = self.get_name_node();
        let mut pwd = self.get_pwd_node();

        name.set_text(user.name.clone().into());
        pwd.set_text(user.pwd.clone().into());

        user.purview.iter().for_each(|p| {});

        let purview = self.get_purview_node();

        purview
            .get_children()
            .iter_shared()
            .enumerate()
            .for_each(|(index, node)| {
                let mut cb = node.cast::<CheckBox>();
                cb.set_pressed(false);

                if let Some(p) = UserPurview::VARIANTS.get(index) {
                    if user.purview.contains(&p) {
                        cb.set_pressed(true);
                    }
                };
            });
    }
}

impl UserManagerView {
    fn user_item_update(&mut self) {
        // 设置用户list
        {
            let db = get_db().lock().unwrap();
            self.users = match TableUser::list(&db) {
                Ok(list) => list,
                Err(_) => Vec::new(),
            };
        }

        // let mut user_list = self
        //     .base()
        //     .get_node_as::<ItemList>(UniqueName::UserList.as_ref());

        self.list = Array::new();
        self.users.iter().for_each(|user| {
            self.list.push(user.name.clone().into());
            // FIX gdext , sub_window 冲突
            // user_list.add_item(user.name.clone().into());
        });

        // gdscript 中处理 add_item
        self.base_mut().emit_signal("update_users".into(), &[]);
    }

    define_get_nodes![
        (get_name_node, UniqueName::Name, LineEdit),
        (get_pwd_node, UniqueName::Pwd, LineEdit),
        (get_purview_node, UniqueName::Purview, BoxContainer),
        (get_submit_node, UniqueName::Submit, Button),
        (get_delete_node, UniqueName::Delete, Button),
        (get_user_list_node, UniqueName::UserList, ItemList),
        (get_error_node, UniqueName::Error, Label),
    ];
}

#[derive(AsRefStr, Debug)]
#[strum(prefix = "%")]
enum UniqueName {
    Name,
    Pwd,
    Purview,
    Submit,
    Delete,

    UserList,
    Error,
}
