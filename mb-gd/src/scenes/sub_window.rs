use godot::{
    classes::{IWindow, PanelContainer, Window},
    prelude::*,
};
use strum::AsRefStr;

use crate::scenes::programs::ProgramsView;

use super::{
    login::LoginView, my_global::MyGlobal, setting::SettingView, user_manager::UserManagerView,
};

// use crate::data::config::{Baudrate, Config};

#[derive(GodotClass)]
#[class(init,base=Window)]
pub struct SubWindowView {
    setting: Gd<PackedScene>,
    programs: Gd<PackedScene>,
    login: Gd<PackedScene>,
    user_manager: Gd<PackedScene>,

    /// 记录当前类型
    scene: SubScenes,

    base: Base<Window>,
}

#[godot_api]
impl IWindow for SubWindowView {
    fn ready(&mut self) {
        self.setting = load("res://sys/setting.tscn");
        self.programs = load("res://sys/programs.tscn");
        self.user_manager = load("res://user/user_manager.tscn");
        self.login = load("res://user/login.tscn");

        let on_close = &self.base().callable("on_close_requested");
        let on_open = &self.base().callable("on_open");

        let mut win = self.base_mut();
        win.connect("close_requested", on_close);
        win.connect("open", on_open);
        win.set_title("系统设置");
        // win.set_size(Vector2i::new(600, 600));
        // win.set_initial_position(WindowInitialPosition::CENTER_PRIMARY_SCREEN);
        win.set_visible(true);
    }
}

#[godot_api]
impl SubWindowView {
    #[signal]
    fn open(scene: SubScenes);

    #[func]
    fn on_close_requested(&mut self) {
        // 关闭时释放并设定全局设置
        self.scene.set_open(false);
        self.base_mut().queue_free();
    }

    #[func]
    fn on_open(&mut self, scene: SubScenes) {
        // godot_print!("open - {:?}", scene);
        self.scene = scene;
        let scene: Gd<PanelContainer> = match scene {
            SubScenes::Setting => self.setting.instantiate_as::<SettingView>().upcast(),
            SubScenes::Programs => self.programs.instantiate_as::<ProgramsView>().upcast(),
            SubScenes::Login => self.login.instantiate_as::<LoginView>().upcast(),
            SubScenes::UserManager => self
                .user_manager
                .instantiate_as::<UserManagerView>()
                .upcast(),
        };

        let mut container = self
            .base()
            .get_node_as::<PanelContainer>(UniqueName::Container.as_ref());

        container.add_child(&scene);
        self.scene.set_open(true)
    }
}

#[derive(Debug, GodotConvert, Copy, Clone, Default, strum::Display)]
#[godot(via = GString)]
pub enum SubScenes {
    #[default]
    #[strum(to_string = "用户登录")]
    Login,
    #[strum(to_string = "用户管理")]
    UserManager,
    #[strum(to_string = "系统设定")]
    Setting,
    #[strum(to_string = "程序编辑")]
    Programs,
}

impl SubScenes {
    pub fn has_open(&self) -> bool {
        let state = MyGlobal::singleton().bind().get_sub_window();
        let pos: u8 = *self as u8;

        (state & (1 << pos)) != 0
    }

    pub fn set_open(&self, open: bool) {
        let mut my_global = MyGlobal::singleton();
        let pos: u8 = *self as u8;
        let state = my_global.bind().get_sub_window();

        let state = if open {
            state | (1 << pos)
        } else {
            state & !(1 << pos)
        };

        my_global.bind_mut().set_sub_window(state);
    }
}

#[derive(AsRefStr, Debug)]
#[strum(prefix = "%")]
enum UniqueName {
    Container,
}
