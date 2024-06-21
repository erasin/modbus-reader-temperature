use channel::VoltageChannelView;
use godot::{
    engine::{Control, IControl, Timer},
    obj::WithBaseField,
    prelude::*,
};
use mb::voltage::{get_mb_state, VoltageData, VoltageState};
use state_tag::VoltageStateTagView;

use crate::{colors::Style, mb_sync::get_voltage_data, scenes::my_global::get_global_config};

pub mod channel;
pub mod state_tag;

#[derive(GodotClass)]
#[class(init, base=Control)]
struct VoltageView {
    #[var]
    sync: bool,
    channel_scene: Gd<PackedScene>,
    tag_scene: Gd<PackedScene>,
    data: Option<VoltageData>,
    base: Base<Control>,
}

#[godot_api]
impl IControl for VoltageView {
    // fn init(base: Base<Control>) -> Self {
    //     godot_print!("voltage init");
    //     Self {
    //         channel_scene: PackedScene::new_gd(),
    //         tag_scene: PackedScene::new_gd(),
    //         data: None,
    //         base,
    //     }
    // }

    fn ready(&mut self) {
        self.channel_scene = load("res://voltage/channel.tscn");
        self.tag_scene = load("res://voltage/state_tag.tscn");

        let mut req_timer = self.base().get_node_as::<Timer>("ReqTimer");

        req_timer.connect(
            "timeout".into(),
            self.base().callable("on_req_timer_timeout"),
        );

        let mut tags_container = self.base_mut().get_node_as::<Control>("%Tags");
        for s in VoltageState::vec() {
            let mut tag_scene = self.tag_scene.instantiate_as::<VoltageStateTagView>();
            tags_container.add_child(tag_scene.clone().upcast());

            let mut tag = tag_scene.bind_mut();
            tag.set_color(s.style());
            tag.set_label(s.to_string().into());
            tag.update_ui();
        }
    }
}

#[godot_api]
impl VoltageView {
    #[signal]
    fn mb_read_over();

    #[func]
    fn on_req_timer_timeout(&mut self) {
        if !self.sync {
            self.mb_read();
        }
    }
}

impl VoltageView {
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

        // godot_print!(" Write failed {:?}", data);

        let mut content = self.base_mut().get_node_as::<Control>("%Container");
        let has = content.get_child_count() == 15;

        for (i, data) in data.data.iter().enumerate() {
            let name = format!("i{i}").to_godot();
            let mut channel = if !has {
                let mut channel_scene = self.channel_scene.instantiate_as::<VoltageChannelView>();
                content.add_child(channel_scene.clone().upcast());
                channel_scene.set_name(name);
                channel_scene
            } else {
                let channel_scene = content.get_node_as::<VoltageChannelView>(name);
                channel_scene
            };

            {
                let color = get_mb_state(data, &config.verify).style();
                let mut channel = channel.bind_mut();

                channel.set_color(color);
                channel.set_data(data);
                channel.update_ui();
            }

            channel.emit_signal("update_data".into(), &[]);
        }

        self.base_mut().emit_signal("mb_read_over".into(), &[]);
    }
}
