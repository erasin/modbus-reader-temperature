use godot::{
    engine::{
        file_dialog::{Access, FileMode},
        window::WindowInitialPosition,
        Button, FileDialog, IPanelContainer, Label, PanelContainer,
    },
    obj::WithBaseField,
    prelude::*,
};
use mb_data::dirs;
use strum::AsRefStr;

use crate::scenes::my_global::get_global_config;

use super::{
    my_global::MyGlobal,
    sub_window::{SubScenes, SubWindowView},
};

#[derive(GodotClass)]
#[class(init, base=PanelContainer)]
pub struct MainView {
    sub_window: Gd<PackedScene>,
    file_dialog: Gd<PackedScene>,
    base: Base<PanelContainer>,
}

#[godot_api]
impl IPanelContainer for MainView {
    fn ready(&mut self) {
        godot_print!("main view");
        self.sub_window = load("res://sub_window.tscn");
        self.file_dialog = load("res://sys/file_dialog.tscn");

        let mut login_btn = self
            .base()
            .get_node_as::<Button>(UniqueName::LoginBtn.as_ref());
        login_btn.connect("pressed".into(), self.base().callable("on_login"));

        self.base()
            .get_node_as::<Button>(UniqueName::SystemSetBtn.as_ref())
            .connect("pressed".into(), self.base().callable("on_setting"));

        self.base()
            .get_node_as::<Button>(UniqueName::UserManagerBtn.as_ref())
            .connect("pressed".into(), self.base().callable("on_user_manager"));

        self.base()
            .get_node_as::<Button>(UniqueName::ProgramsBtn.as_ref())
            .connect("pressed".into(), self.base().callable("on_programs"));

        let mut my_global = MyGlobal::singleton();
        my_global.connect(
            "config_update".into(),
            self.base().callable("on_global_config_update"),
        );
        my_global.connect(
            "login_update".into(),
            self.base().callable("on_global_login_update"),
        );

        self.ab_init();
        self.user_state_update();

        // let mut voltage_a = self
        //     .base()
        //     .get_node_as::<VoltageView>(UniqueName::VoltageA.as_ref());
        // voltage_a.bind_mut().set_ab(AB::Apanel.to_godot());

        // let mut voltage_b = self
        //     .base()
        //     .get_node_as::<VoltageView>(UniqueName::VoltageA.as_ref());
        // voltage_b.bind_mut().set_ab(AB::Bpanel.to_godot());

        // TODO remove
        let mut open_btn = self.base().get_node_as::<Button>(UniqueName::Open.as_ref());
        open_btn.connect("pressed".into(), self.base().callable("open_xls"));

        let mut save_btn = self.base().get_node_as::<Button>(UniqueName::Save.as_ref());
        save_btn.connect("pressed".into(), self.base().callable("save_xls"));
    }
}

#[godot_api]
impl MainView {
    #[signal]
    fn load_over();

    #[func]
    fn on_global_config_update(&mut self) {
        self.ab_init();
    }

    #[func]
    fn on_global_login_update(&mut self) {
        self.user_state_update()
    }

    #[func]
    fn on_setting(&mut self) {
        let scene = SubScenes::Setting;
        if scene.has_open() {
            return;
        }
        let mut root = self.base().get_tree().unwrap().get_root().unwrap();
        let mut win_scene = self.sub_window.instantiate_as::<SubWindowView>();
        root.add_child(win_scene.clone().upcast());
        win_scene.emit_signal("open".into(), &[scene.to_variant()]);
        win_scene.set_title(scene.to_string().into());
    }

    #[func]
    fn on_login(&mut self) {
        let mut g = MyGlobal::singleton();
        let has_login = g.bind().get_login().is_some();
        if has_login {
            // 退出登录
            g.bind_mut().set_logout();
            self.user_state_update();
            return;
        };

        let scene = SubScenes::Login;
        if scene.has_open() {
            return;
        }
        let mut root = self.base().get_tree().unwrap().get_root().unwrap();
        let mut win_scene = self.sub_window.instantiate_as::<SubWindowView>();
        root.add_child(win_scene.clone().upcast());
        win_scene.emit_signal("open".into(), &[scene.to_variant()]);
        win_scene.set_title(scene.to_string().into());
    }

    #[func]
    fn on_user_manager(&mut self) {
        let scene = SubScenes::UserManager;
        if scene.has_open() {
            return;
        }
        let mut root = self.base().get_tree().unwrap().get_root().unwrap();
        let mut win_scene = self.sub_window.instantiate_as::<SubWindowView>();
        root.add_child(win_scene.clone().upcast());
        win_scene.emit_signal("open".into(), &[scene.to_variant()]);
        win_scene.set_title(scene.to_string().into());
    }

    #[func]
    fn on_programs(&mut self) {
        let scene = SubScenes::Programs;
        if scene.has_open() {
            return;
        }
        let mut root = self.base().get_tree().unwrap().get_root().unwrap();

        let mut win_scene = self.sub_window.instantiate_as::<SubWindowView>();
        root.add_child(win_scene.clone().upcast());
        win_scene.emit_signal("open".into(), &[scene.to_variant()]);
        win_scene.set_title(scene.to_string().into());
    }

    #[func]
    fn on_sync_start(&mut self) {}

    #[func]
    fn open_xls(&mut self) {
        let mut root = self.base().get_tree().unwrap().get_root().unwrap();
        let mut dialog = self.file_dialog.instantiate_as::<FileDialog>();

        dialog.set_title("打开文件".into());
        dialog.set_text("打开文件".into());
        dialog.set_cancel_button_text("取消".into());
        dialog.set_ok_button_text("打开".into());
        dialog.set_file_mode(FileMode::OPEN_FILE);
        dialog.set_access(Access::FILESYSTEM);
        dialog.set_use_native_dialog(true);
        dialog.set_current_dir(
            dirs::data_dir()
                .as_path()
                .to_string_lossy()
                .to_string()
                .into(),
        );
        dialog.set_size(Vector2i::new(800, 500));
        dialog.set_initial_position(WindowInitialPosition::CENTER_PRIMARY_SCREEN);

        dialog.connect("file_selected".into(), self.base().callable("on_open_file"));

        dialog.set_visible(true);
        root.add_child(dialog.upcast());
    }

    #[func]
    fn on_open_file(&mut self, file_name: GString) {
        godot_print!("--file--{}", file_name);
    }

    #[func]
    fn save_xls(&mut self) {
        let mut root = self.base().get_tree().unwrap().get_root().unwrap();
        let mut dialog = self.file_dialog.instantiate_as::<FileDialog>();

        dialog.set_title("保存文件".into());
        dialog.set_text("保存文件".into());
        dialog.set_cancel_button_text("取消".into());
        dialog.set_ok_button_text("保存".into());
        dialog.set_file_mode(FileMode::SAVE_FILE);
        dialog.set_access(Access::FILESYSTEM);
        dialog.set_use_native_dialog(true);
        dialog.set_current_dir(
            dirs::data_dir()
                .as_path()
                .to_string_lossy()
                .to_string()
                .into(),
        );
        dialog.set_size(Vector2i::new(800, 500));
        dialog.set_initial_position(WindowInitialPosition::CENTER_PRIMARY_SCREEN);

        dialog.connect("file_selected".into(), self.base().callable("on_save_file"));

        dialog.set_visible(true);
        root.add_child(dialog.upcast());
    }

    #[func]
    fn on_save_file(&mut self, file_name: GString) {
        godot_print!("--file--{}", file_name);
    }
}

impl MainView {
    fn ab_init(&mut self) {
        let config = get_global_config();

        self.base()
            .get_node_as::<PanelContainer>(UniqueName::Apanel.as_ref())
            .set_visible(config.enable_a_panel);

        self.base()
            .get_node_as::<PanelContainer>(UniqueName::Bpanel.as_ref())
            .set_visible(config.enable_b_panel);
    }

    fn user_state_update(&mut self) {
        let mut login_btn = self
            .base()
            .get_node_as::<Button>(UniqueName::LoginBtn.as_ref());
        let mut label = self
            .base()
            .get_node_as::<Label>(UniqueName::LoginUserName.as_ref());

        let g = MyGlobal::singleton();
        match g.bind().get_login() {
            Some(user) => {
                label.set_text(user.name.into());
                login_btn.set_text("退出".into());
            }
            None => {
                label.set_text("".into());
                login_btn.set_text("登录".into());
            }
        };
    }
}

#[derive(AsRefStr, Debug)]
#[strum(prefix = "%")]
enum UniqueName {
    LoginUserName,
    LoginBtn,

    Apanel,
    Bpanel,

    VoltageA,
    VoltageB,

    SystemSetBtn,
    ProgramsBtn,
    UserManagerBtn,

    Open,
    Save,
}
