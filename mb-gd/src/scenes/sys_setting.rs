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
        let on_close = self.base().callable("on_close_requested").clone();

        let mut win = self.base_mut();
        win.connect("close_requested".into(), on_close);

        win.set_title("系统设置".into());
        win.set_size(Vector2i::new(600, 600));

        // 移动位置需要实例化
        // win.set_position(Vector2i::new(300, 300));
        // win.set_initial_position(WindowInitialPosition::CENTER_MAIN_WINDOW_SCREEN);

        win.set_visible(true);
    }
}

#[godot_api]
impl SysSettingWindowView {
    #[func]
    fn on_close_requested(&mut self) {
        self.base_mut().queue_free()
    }
}
