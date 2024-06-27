use godot::{
    engine::{window::WindowInitialPosition, DisplayServer, IWindow, Window},
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
        // win.set_size(Vector2i::new(600, 600));
        // win.set_initial_position(WindowInitialPosition::CENTER_PRIMARY_SCREEN);
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

impl SysSettingWindowView {
    // 移动位置需要实例化
    pub fn to_center(&mut self) {
        let screen = DisplayServer::singleton();
        let screen_size = screen.screen_get_size();
        let win_size = self.base().get_size();
        let position = (screen_size - win_size) / 2 + win_size;
        self.base_mut().set_position(position);

        self.base_mut()
            .set_initial_position(WindowInitialPosition::CENTER_PRIMARY_SCREEN);
    }
}
