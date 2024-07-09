use godot::{
    engine::{Control, IPanelContainer, Label, PanelContainer},
    obj::WithBaseField,
    prelude::*,
};
use mb::voltage::VoltageChannel;
use strum::AsRefStr;

use crate::{colors::ColorPlate, define_get_nodes};

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
        Self {
            index: 0,
            data: VoltageChannel::default(),
            color: ColorPlate::White.into(),
            base,
        }
    }
    fn ready(&mut self) {
        let mut voltage_label = self.get_voltage_node();
        let mut current_label = self.get_current_node();
        voltage_label.set_visible(false);
        current_label.set_visible(false);
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
        let mut index_label = self.get_index_node();
        let mut voltage_label = self.get_voltage_node();
        let mut current_label = self.get_current_node();
        let mut state = self.get_state_node();

        let index = if self.data.index == 0 {
            self.index
        } else {
            self.data.index
        } + 1;

        index_label.set_text(format!("{:2}", index).into());
        voltage_label.set_text(format!("{:2.2}V", self.data.voltage).into());
        current_label.set_text(format!("{:2.2}A", self.data.current).into());

        state.set_modulate(self.color);
    }

    define_get_nodes![
        (get_index_node, UniqueName::Index, Label),
        (get_voltage_node, UniqueName::Voltage, Label),
        (get_current_node, UniqueName::Current, Label),
        (get_state_node, UniqueName::State, Control),
    ];
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
