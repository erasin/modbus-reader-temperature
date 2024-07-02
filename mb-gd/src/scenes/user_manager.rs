use godot::{
    engine::{BoxContainer, CheckBox, IPanelContainer, PanelContainer},
    prelude::*,
};
use strum::{AsRefStr, IntoEnumIterator, VariantArray};

use mb_data::user::{UserConfig, UserPurview};

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
        let mut purview = self
            .base()
            .get_node_as::<BoxContainer>(UniqueName::Purview.as_ref());

        UserPurview::iter().into_iter().for_each(|up| {
            let mut cb = CheckBox::new_alloc();
            cb.set_text(up.to_string().into());
            purview.add_child(cb.upcast());
        });

        // 设置用户list
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
