use godot::{
    engine::{IWindow, Window},
    prelude::*,
};

// use crate::data::config::{Baudrate, Config};

#[derive(GodotClass)]
#[class(init,base=Window)]
pub struct SysSettingWindowView {
    base: Base<Window>,
}

#[godot_api]
impl IWindow for SysSettingWindowView {
    fn ready(&mut self) {
        // let mut win = Window::new_alloc();
        // win.set_title("test----".into());
        // win.add_child(win_scene.upcast());
        // self.on_notification(what)

        // self.base_mut().connect(
        //     "close_requested".into(),
        //     self.base().callable("on_close_requested").clone(),
        // );

        let mut win = self.base_mut();

        win.set_title("test".into());
        win.set_visible(true);
        win.set_size(Vector2i::new(300, 300));
        win.set_position(Vector2i::new(300, 300));
    }
}

#[godot_api]
impl SysSettingWindowView {
    #[func]
    fn on_close_requested(&mut self) {
        self.base_mut().queue_free()
    }
}
