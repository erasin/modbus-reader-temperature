use crate::data::MbState;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct Item {
    #[var]
    color: Color,

    data: MbState,

    base: Base<Control>,
}

#[godot_api]
impl IControl for Item {
    fn init(base: Base<Control>) -> Self {
        // godot_print!("item init");
        Self {
            data: MbCh::default(),

            color: colors::GREY,

            base,
        }
    }
}

impl Item {
    pub fn set_data(&mut self, data: &MbCh) {
        self.data = data.clone();
    }
}
