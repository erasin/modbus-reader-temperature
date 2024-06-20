use godot::{
    engine::{Control, IControl, Label, Sprite2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct Main {
    base: Base<Control>,
}

#[godot_api]
impl Main {
    #[signal]
    fn load_over();
}
