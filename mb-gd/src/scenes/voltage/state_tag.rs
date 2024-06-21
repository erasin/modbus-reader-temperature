use godot::{
    engine::{Control, IControl, Label},
    obj::WithBaseField,
    prelude::*,
};

use crate::colors::ColorPlate;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct VoltageStateTagView {
    #[var]
    color: Color,
    #[var]
    label: GString,

    base: Base<Control>,
}

#[godot_api]
impl IControl for VoltageStateTagView {
    fn init(base: Base<Control>) -> Self {
        // godot_print!("item init");
        Self {
            label: "空位".into(),
            color: ColorPlate::White.color(),
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

        label.set_text(self.label.clone());
        state.set_modulate(self.color);
    }
}
