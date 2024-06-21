use godot::{
    engine::{Control, IControl, Label},
    obj::WithBaseField,
    prelude::*,
};
use mb::voltage::VoltageChannel;

use crate::colors::ColorPlate;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct VoltageChannelView {
    #[var]
    color: Color,
    data: VoltageChannel,
    base: Base<Control>,
}

#[godot_api]
impl IControl for VoltageChannelView {
    fn init(base: Base<Control>) -> Self {
        // godot_print!("item init");
        Self {
            data: VoltageChannel::default(),
            color: ColorPlate::White.color(),
            base,
        }
    }
    fn ready(&mut self) {
        // let on_update_data = self.base().callable("on_update_data");
        // self.base_mut()
        //     .connect("update_data".into(), on_update_data);
    }
}

#[godot_api]
impl VoltageChannelView {
    #[signal]
    fn update_data();

    #[func]
    pub fn on_update_data(&mut self) {}
}

impl VoltageChannelView {
    pub fn set_data(&mut self, data: &VoltageChannel) {
        self.data = data.clone();
    }

    pub fn update_ui(&mut self) {
        let mut index_label = self.base().get_node_as::<Label>("%index");
        let mut voltage_label = self.base().get_node_as::<Label>("%voltage");
        let mut current_label = self.base().get_node_as::<Label>("%current");
        let mut state = self.base().get_node_as::<Control>("%state");

        // let data = self.data.to_godot();
        // index_label.set_text(data.at("index").to());
        // voltage_label.set_text(data.at("voltage").to());
        // current_label.set_text(data.at("current").to());

        index_label.set_text(format!("{:2}", self.data.index + 1).into());
        voltage_label.set_text(format!("{:.2}V", self.data.voltage).into());
        current_label.set_text(format!("{:2} mA", self.data.current).into());

        state.set_modulate(self.color);
    }
}
