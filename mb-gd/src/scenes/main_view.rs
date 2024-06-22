use godot::{
    engine::{
        file_dialog::{Access, FileMode},
        window::WindowInitialPosition,
        Button, FileDialog, IPanelContainer, PanelContainer,
    },
    prelude::*,
};
use mb_data::dirs;

use super::sys_setting::SysSettingWindowView;

#[derive(GodotClass)]
#[class(init, base=PanelContainer)]
pub struct MainView {
    sys_setting_scene: Gd<PackedScene>,
    file_dialog: Gd<PackedScene>,
    base: Base<PanelContainer>,
}

#[godot_api]
impl IPanelContainer for MainView {
    fn ready(&mut self) {
        self.sys_setting_scene = load("res://sys/setting_win.tscn");
        self.file_dialog = load("res://sys/file_dialog.tscn");

        let mut sys_btn = self.base().get_node_as::<Button>("%SystemSetBtn");
        sys_btn.connect("pressed".into(), self.base().callable("on_sys_open"));

        let mut open_btn = self.base().get_node_as::<Button>("%Open");
        open_btn.connect("pressed".into(), self.base().callable("open_xls"));

        let mut save_btn = self.base().get_node_as::<Button>("%Save");
        save_btn.connect("pressed".into(), self.base().callable("save_xls"));
    }
}

#[godot_api]
impl MainView {
    #[signal]
    fn load_over();

    #[func]
    fn on_sys_open(&mut self) {
        let mut root = self.base().get_tree().unwrap().get_root().unwrap();

        // 加载新窗口
        let mut win_scene = self
            .sys_setting_scene
            .instantiate_as::<SysSettingWindowView>();

        win_scene.bind_mut().to_center();

        root.add_child(win_scene.upcast());
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
