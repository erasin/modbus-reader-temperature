use godot::{
    engine::{Control, IPanelContainer, Label, PanelContainer},
    obj::WithBaseField,
    prelude::*,
};
use mb::voltage::VoltageChannel;
use strum::AsRefStr;

use crate::colors::ColorPlate;

#[derive(GodotClass)]
#[class(base=PanelContainer)]
pub struct VoltageChannelView {
    #[var]
    color: Color,
    // slave index
    index: usize,
    data: VoltageChannel,
    base: Base<PanelContainer>,
}

#[godot_api]
impl IPanelContainer for VoltageChannelView {
    fn init(base: Base<PanelContainer>) -> Self {
        // godot_print!("item init");
        Self {
            index: 0,
            data: VoltageChannel::default(),
            color: ColorPlate::White.into(),
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
        self.data = *data;
    }

    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    pub fn update_ui(&mut self) {
        let mut index_label = self
            .base()
            .get_node_as::<Label>(UniqueName::Index.to_string());
        let mut voltage_label = self
            .base()
            .get_node_as::<Label>(UniqueName::Voltage.to_string());
        let mut current_label = self
            .base()
            .get_node_as::<Label>(UniqueName::Current.to_string());
        let mut state = self
            .base()
            .get_node_as::<Control>(UniqueName::State.to_string());

        index_label.set_text(format!("{:2}", self.index + 1).into());
        voltage_label.set_text(format!("{:2.2}V", self.data.voltage).into());
        current_label.set_text(format!("{:2.2}A", self.data.current).into());

        state.set_modulate(self.color);
    }
}

#[derive(AsRefStr, Debug)]
#[strum(prefix = "%")]
enum UniqueName {
    Index,
    Voltage,
    Current,
    State,
}

impl std::fmt::Display for UniqueName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
