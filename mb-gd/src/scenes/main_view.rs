use godot::{
    engine::{Button, Control, IControl, SubViewportContainer, Window},
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
        self.sys_setting_scene = load("res://sys_setting_window_view.tscn");

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

        let win_scene = self.sys_setting_scene.instantiate_as::<Window>();
        // win_scene.set_title("test".into());
        // win_scene.set_visible(true);
        // win_scene.set_size(Vector2i::new(300, 300));
        // win_scene.set_position(Vector2i::new(300, 300));

        // win_scene.callable("")

        // win_scene.connect(
        //     "close_requested".into(),
        //     win_scene.callable("on_req_timer_timeout"),
        // );

        root.add_child(win_scene.upcast());
    }
}
