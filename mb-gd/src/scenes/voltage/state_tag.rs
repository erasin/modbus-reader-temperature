use godot::{
    classes::{Control, IPanelContainer, Label, PanelContainer},
    obj::WithBaseField,
    prelude::*,
};

use crate::colors::ColorPlate;

#[derive(GodotClass)]
#[class(base=PanelContainer)]
pub struct VoltageStateTagView {
    #[var]
    color: Color,
    #[var]
    label: GString,

    base: Base<PanelContainer>,
}

#[godot_api]
impl IPanelContainer for VoltageStateTagView {
    fn init(base: Base<PanelContainer>) -> Self {
        // godot_print!("item init");
        Self {
            label: "空位".into(),
            color: ColorPlate::White.into(),
            base,
        }
    }

    fn ready(&mut self) {
        self.update_ui();
    }
}

impl VoltageStateTagView {
    pub fn update_ui(&mut self) {
        let mut label = self.base().get_node_as::<Label>("%Label");
        let mut state = self.base().get_node_as::<Control>("%Color");

        label.set_text(&self.label);
        state.set_modulate(self.color);
    }
}
