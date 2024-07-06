//! data extra for godot

use godot::prelude::*;
use mb::voltage::VoltageChannel;
use mb_data::task;

#[derive(GodotConvert, Var, Export, Debug, Default, Clone, Copy, PartialEq, Eq)]
#[godot(via = GString)]
pub enum AB {
    #[default]
    Apanel,
    Bpanel,
}

impl AB {
    pub fn title(self) -> String {
        match self {
            AB::Apanel => "A面".to_owned(),
            AB::Bpanel => "B面".to_owned(),
        }
    }

    pub fn is_a(&self) -> bool {
        *self == AB::Apanel
    }

    pub fn is_b(&self) -> bool {
        *self == AB::Bpanel
    }
}

impl From<task::AB> for AB {
    fn from(value: task::AB) -> Self {
        match value {
            task::AB::A => Self::Apanel,
            task::AB::B => Self::Bpanel,
        }
    }
}

impl From<AB> for task::AB {
    fn from(val: AB) -> Self {
        match val {
            AB::Apanel => task::AB::A,
            AB::Bpanel => task::AB::B,
        }
    }
}

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
