use godot::{engine::Control, prelude::*};

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
