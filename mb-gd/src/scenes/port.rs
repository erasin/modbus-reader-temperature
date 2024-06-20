use godot::{
    engine::{Control, IControl, Timer},
    prelude::*,
};
use mb::voltage::{get_mb_state, VoltageData, VoltageState};

use crate::{
    colors::Style,
    mb_sync::get_voltage_data,
    scenes::{item, my_global::get_global_config},
};

#[derive(GodotClass)]
#[class(base=Control)]
struct SerialPort {
    item_scene: Gd<PackedScene>,
    data: Option<VoltageData>,
    base: Base<Control>,
}

#[godot_api]
impl IControl for SerialPort {
    fn init(base: Base<Control>) -> Self {
        godot_print!("port init");
        Self {
            item_scene: PackedScene::new_gd(),
            data: None,
            base,
        }
    }

    fn ready(&mut self) {
        self.item_scene = load("res://item.tscn");

        let mut req_timer = self.base().get_node_as::<Timer>("ReqTimer");

        req_timer.connect(
            "timeout".into(),
            self.base().callable("on_req_timer_timeout"),
        );
    }
}

#[godot_api]
impl SerialPort {
    #[signal]
    fn mb_read_over();

    #[func]
    fn on_req_timer_timeout(&mut self) {
        self.mb_read();
    }

    #[func]
    pub fn mb_read(&mut self) {
        let config = get_global_config();

        let data = match get_voltage_data(&config) {
            Ok(i) => i,
            Err(e) => {
                godot_print!(" Write failed {}: {:?}", config.port(), e);
                return;
            }
        };

        self.data = Some(data);

        godot_print!(" Write failed {:?}", data);

        let mut content = self.base_mut().get_node_as::<Control>("%Container");
        let has = content.get_child_count() == 15;

        for (i, data) in data.data.iter().enumerate() {
            let name = format!("i{i}").to_godot();
            let item_scene = if !has {
                let mut item_scene = self.item_scene.instantiate_as::<Control>();
                content.add_child(item_scene.clone().upcast());

                item_scene.set_name(name);
                // item_scene.set_offset(Side::LEFT, i as f32 * 200.);

                item_scene
            } else {
                let item_scene = content.get_node_as::<Control>(name);

                item_scene
            };

            let mut item = item_scene.cast::<item::Item>();

            {
                let (_, color) = get_mb_state(data, &config.verify).style();
                let mut item = item.bind_mut();

                item.set_data(data);
                item.set_color(color);
                item.update_show();
            }

            item.emit_signal("update_data".into(), &[]);
        }

        self.base_mut().emit_signal("mb_read_over".into(), &[]);
    }
}
