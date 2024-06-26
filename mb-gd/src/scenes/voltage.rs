use channel::VoltageChannelView;
use godot::{
    engine::{Button, Control, IPanelContainer, Label, PanelContainer, Timer},
    obj::WithBaseField,
    prelude::*,
};
use mb::voltage::{get_mb_state, VoltageData, VoltageState};
use state_tag::VoltageStateTagView;

use crate::{
    colors::Style, data::AB, mb_sync::get_voltage_data, scenes::my_global::get_global_config,
};

pub mod channel;
pub mod state_tag;

#[derive(GodotClass)]
#[class(init, base=PanelContainer)]
pub struct VoltageView {
    #[export]
    ab: AB,

    channel_scene: Gd<PackedScene>,
    tag_scene: Gd<PackedScene>,
    data: Option<VoltageData>,
    base: Base<PanelContainer>,
}

#[godot_api]
impl IPanelContainer for VoltageView {
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

        let mut label_ab_name = self.base().get_node_as::<Label>("%AbName");

        match self.ab {
            AB::A => label_ab_name.set_text("A面".into()),
            AB::B => label_ab_name.set_text("B面".into()),
        };

        let mut req_timer = self.base().get_node_as::<Timer>("ReqTimer");

        req_timer.connect(
            "timeout".into(),
            self.base().callable("on_req_timer_timeout"),
        );

        // 标签显示
        let mut tags_container = self.base_mut().get_node_as::<Control>("%Tags");
        for s in VoltageState::vec() {
            let mut tag_scene = self.tag_scene.instantiate_as::<VoltageStateTagView>();
            tags_container.add_child(tag_scene.clone().upcast());

            let mut tag = tag_scene.bind_mut();
            tag.set_color(s.style());
            tag.set_label(s.to_string().into());
            tag.update_ui();
        }

        // 开启
        let mut start_btn = self.base().get_node_as::<Button>("%StartToggle");
        start_btn.connect(
            "pressed".into(),
            self.base().callable("on_start_toggle_sync"),
        );
    }
}

#[godot_api]
impl VoltageView {
    #[signal]
    fn mb_read_over();

    #[func]
    fn on_req_timer_timeout(&mut self) {
        self.mb_read();
    }

    #[func]
    fn on_start_toggle_sync(&mut self) {
        godot_print!("--- start pull");
        let mut start_btn = self.base().get_node_as::<Button>("%StartToggle");

        let mut timer = self.base().get_node_as::<Timer>("%ReqTimer");
        if timer.is_stopped() {
            timer.start();
            start_btn.set_text("停止".into());
        } else {
            timer.stop();
            start_btn.set_text("开始".into());
        }
    }
}

impl VoltageView {
    pub fn mb_read(&mut self) {
        let config = get_global_config();

        //TODO 根据 AB 区 获取参数
        let _config = match self.ab {
            AB::A => {}
            AB::B => {}
        };

        let mut timer = self.base().get_node_as::<Timer>("%ReqTimer");

        let data = match get_voltage_data(&config.voltage) {
            Ok(i) => i,
            Err(e) => {
                timer.stop();

                let mut start_btn = self.base().get_node_as::<Button>("%StartToggle");
                start_btn.set_text("开始".into());

                godot_print!(" Write failed {:?}: {:?}", config.voltage, e);
                return;
            }
        };

        self.data = Some(data.clone());

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
                let color = get_mb_state(data, &config.voltage.verify).style();
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
