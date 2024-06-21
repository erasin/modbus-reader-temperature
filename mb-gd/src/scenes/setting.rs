use godot::{
    engine::{Button, Control, IControl, OptionButton},
    prelude::*,
};
use mb::protocol::get_ports;

use crate::scenes::my_global::{get_global_config, set_global_config};
use mb_data::{
    config::{Baudrate, Config},
    db::{
        config::{get_config, set_config},
        get_db,
    },
};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct SettingView {
    config: Config,
    base: Base<Control>,
}

#[godot_api]
impl IControl for SettingView {
    fn init(base: Base<Control>) -> Self {
        godot_print!("setting init");
        Self {
            config: get_global_config(),
            base,
        }
    }

    fn ready(&mut self) {
        let mut port_btn = self.base().get_node_as::<OptionButton>("%Port");

        let ports = get_ports();

        for (index, port) in ports.iter().enumerate() {
            port_btn.add_item(port.to_godot());
            if port.bytes().eq(self.config.port.bytes()) {
                port_btn.select(index as i32);
            }
        }

        port_btn.connect(
            "item_selected".into(),
            self.base().callable("on_port_item_selected"),
        );

        let mut baudrate_btn = self.base().get_node_as::<OptionButton>("%Baudrate");

        for (index, &item) in Baudrate::ALL.iter().enumerate() {
            baudrate_btn.add_item(item.to_string().to_godot());

            if item == self.config.baudrate {
                baudrate_btn.select(index as i32);
            }
        }

        baudrate_btn.connect(
            "item_selected".into(),
            self.base().callable("on_baudrate_item_selected"),
        );

        let mut submit_btn = self.base().get_node_as::<Button>("%Submit");

        submit_btn.connect("pressed".into(), self.base().callable("on_submit"));
    }
}

#[godot_api]
impl SettingView {
    #[signal]
    fn update_data();

    #[func]
    fn on_port_item_selected(&mut self, index: i32) {
        let ports = get_ports();

        let sel = match ports.get(index as usize) {
            Some(b) => b.clone(),
            None => return,
        };

        self.config.port = sel;

        return;
    }

    #[func]
    fn on_baudrate_item_selected(&mut self, index: i32) {
        let sel = match Baudrate::ALL.get(index as usize) {
            Some(&b) => b,
            None => return,
        };

        self.config.baudrate = sel;

        return;
    }

    #[func]
    fn on_submit(&mut self) {
        {
            let db = get_db().lock().unwrap();
            let _ = set_config(&db, self.config.clone());

            match get_config(&db) {
                Ok(conf) => {
                    godot_print!("--- config: {:?}", conf);
                    set_global_config(conf);
                }
                Err(e) => godot_print!("{e}"),
            };
        }
    }
}

impl SettingView {}
