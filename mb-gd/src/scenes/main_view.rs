use godot::{
    engine::{Button, Control, DisplayServer, IControl, Window},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct MainView {
    sys_setting_scene: Gd<PackedScene>,
    base: Base<Control>,
}

#[godot_api]
impl IControl for MainView {
    fn ready(&mut self) {
        self.sys_setting_scene = load("res://sys/setting_win.tscn");
        let mut sys_btn = self.base().get_node_as::<Button>("%SystemSetBtn");
        sys_btn.connect("pressed".into(), self.base().callable("on_sys_open"));
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
        let mut win_scene = self.sys_setting_scene.instantiate_as::<Window>();
        // win_scene.set_initial_position(WindowInitialPosition::CENTER_MAIN_WINDOW_SCREEN);

        let screen = DisplayServer::singleton();
        let screen_size = screen.screen_get_size();
        let win_size = win_scene.get_size();
        let position = (screen_size - win_size) / 3 + win_size;
        win_scene.set_position(position);
        // win_scene.set_position(Vector2i::new(300, 300));

        root.add_child(win_scene.upcast());
    }

    #[func]
    fn on_sync_start(&mut self) {}
}
