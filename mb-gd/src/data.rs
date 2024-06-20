use godot::prelude::*;
use mb::voltage::VoltageChannel;

pub mod config;
pub mod user;

/// 使用新的 特性实现 to_godot
pub trait ToGodotVia {
    type Via;
    fn to_godot(&self) -> Self::Via;
}

impl ToGodotVia for VoltageChannel {
    type Via = Dictionary;

    fn to_godot(&self) -> Self::Via {
        let mut dict = Dictionary::new();
        dict.set("index", format!("{:2}", self.index + 1));
        dict.set("voltage", format!("{:.2}V", self.voltage));
        dict.set("current", format!("{:2} mA", self.current));
        dict
    }
}
