use channel::VoltageChannelView;
use godot::{
    engine::{Button, Control, IPanelContainer, Label, PanelContainer, Range, Timer},
    obj::WithBaseField,
    prelude::*,
};
use mb::voltage::{get_mb_state, VoltageData, VoltageState};
use state_tag::VoltageStateTagView;
use strum::{AsRefStr, IntoEnumIterator};

use crate::{
    chart::ChartView,
    colors::{self, ColorPlate, Style},
    data::AB,
    mb_sync::get_voltage_data,
    scenes::my_global::get_global_config,
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

        let mut label_ab_name = self
            .base()
            .get_node_as::<Label>(UniqueName::AbName.to_string());

        match self.ab {
            AB::Apanel => label_ab_name.set_text("A面".into()),
            AB::Bpanel => label_ab_name.set_text("B面".into()),
        };

        let mut req_timer = self.base().get_node_as::<Timer>("ReqTimer");

        req_timer.connect(
            "timeout".into(),
            self.base().callable("on_req_timer_timeout"),
        );

        // 标签显示
        let mut tags_container = self
            .base_mut()
            .get_node_as::<Control>(UniqueName::Tags.to_string());
        for s in VoltageState::iter() {
            let mut tag_scene = self.tag_scene.instantiate_as::<VoltageStateTagView>();
            tags_container.add_child(tag_scene.clone().upcast());

            let mut tag = tag_scene.bind_mut();
            tag.set_color(s.style());
            tag.set_label(s.to_string().into());
            tag.update_ui();
        }

        // 开启
        let mut start_btn = self
            .base()
            .get_node_as::<Button>(UniqueName::StartToggle.to_string());
        start_btn.connect(
            "pressed".into(),
            self.base().callable("on_start_toggle_sync"),
        );

        let mut chart = self
            .base()
            .get_node_as::<ChartView>(UniqueName::Chart.to_string());

        {
            let mut chart = chart.bind_mut();

            let points = [
                Vector2::new(0., 10.),
                Vector2::new(10., 110.),
                Vector2::new(20., 110.),
                Vector2::new(30., 110.),
                Vector2::new(40., 10.),
                Vector2::new(50., 10.),
                Vector2::new(60., 10.),
            ];

            let x_labels = [0., 10., 20., 30., 40., 50., 60.0];
            let y_labels = [0., 55., 110., 220., 240.0];

            chart.set_points(points.into());
            chart.set_x_labels(x_labels.into());
            chart.set_y_labels(y_labels.into());
            // chart.set_color(ColorPlate::Red.into());
            // chart.set_background_color(ColorPlate::Black.into());
            // chart.set_grid_color(ColorPlate::Grey.into());
        }
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
        let mut start_btn = self
            .base()
            .get_node_as::<Button>(UniqueName::StartToggle.to_string());

        let mut timer = self
            .base()
            .get_node_as::<Timer>(UniqueName::ReqTimer.to_string());
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
            AB::Apanel => {}
            AB::Bpanel => {}
        };

        let mut timer = self
            .base()
            .get_node_as::<Timer>(UniqueName::ReqTimer.to_string());

        let data = match get_voltage_data(&config.voltage) {
            Ok(i) => i,
            Err(e) => {
                timer.stop();

                let mut start_btn = self
                    .base()
                    .get_node_as::<Button>(UniqueName::StartToggle.to_string());
                start_btn.set_text("开始".into());

                godot_print!(" Write failed {:?}: {:?}", config.voltage, e);
                return;
            }
        };

        self.data = Some(data.clone());

        // godot_print!(" Write failed {:?}", data);

        let mut content = self
            .base_mut()
            .get_node_as::<Control>(UniqueName::Container.to_string());
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

#[derive(AsRefStr, Debug)]
enum UniqueName {
    Tags,
    ReqTimer,
    Container,
    StartToggle,
    AgeingToggle,
    VccToggle,

    ProName,
    StartTime,
    CountDownTime,
    AgeingTime,
    AccState,
    CountNum,
    CountGood,
    CountDefective,

    AbName,
    StateRun,
    StateError,
    StateAgeing,
    StateAcc,

    Chart,
}

// godot 唯一
impl std::fmt::Display for UniqueName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "%{}", self.as_ref())
    }
}
