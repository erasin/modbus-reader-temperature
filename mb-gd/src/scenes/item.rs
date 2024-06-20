use godot::{
    engine::{Control, IControl, Label},
    prelude::*,
};
use mb::voltage::VoltageChannel;

use crate::colors;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct Item {
    #[var]
    color: Color,
    data: VoltageChannel,
    base: Base<Control>,
}

#[godot_api]
impl IControl for Item {
    fn init(base: Base<Control>) -> Self {
        // godot_print!("item init");
        Self {
            data: VoltageChannel::default(),
            color: colors::GREY,
            base,
        }
    }
}

impl Item {
    pub fn set_data(&mut self, data: &VoltageChannel) {
        self.data = data.clone();
    }
}

#[godot_api]
impl Item {
    #[signal]
    fn update_data();

    #[func]
    pub fn update_show(&mut self) {
        let mut index_label = self.base().get_node_as::<Label>("%index");
        let mut voltage_label = self.base().get_node_as::<Label>("%voltage");
        let mut current_label = self.base().get_node_as::<Label>("%current");
        let mut state = self.base().get_node_as::<Control>("%state");

        // let data = self.data.to_godot();
        // index_label.set_text(data.at("index").to());
        // voltage_label.set_text(data.at("voltage").to());
        // current_label.set_text(data.at("current").to());

        index_label.set_text(format!("{:2}", self.data.index + 1).to_godot());
        voltage_label.set_text(format!("{:.2}V", self.data.voltage).to_godot());
        current_label.set_text(format!("{:2} mA", self.data.current).to_godot());

        state.set_modulate(self.color.to_godot());
        // let mut mt = state.get_material().unwrap();
        // mt.set("shader_parameter/color".to_godot(), self.color.to_godot());
    }
}
