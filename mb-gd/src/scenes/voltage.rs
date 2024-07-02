use channel::VoltageChannelView;
use godot::{
    engine::{Button, Control, IPanelContainer, Label, PanelContainer, Timer},
    obj::WithBaseField,
    prelude::*,
};
use mb::voltage::{get_mb_state, VoltageData, VoltageState, VOLTAGE_CHANNEL};
use state_tag::VoltageStateTagView;
use strum::{AsRefStr, IntoEnumIterator};

use crate::{
    chart::ChartView, colors::IntoColor, data::AB, mb_sync::get_voltage_data,
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
    // data: Option<VoltageData>,
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
            .get_node_as::<Label>(UniqueName::AbName.as_ref());
        label_ab_name.set_text(self.ab.title().into());

        let mut req_timer = self.base().get_node_as::<Timer>("ReqTimer");
        req_timer.connect(
            "timeout".into(),
            self.base().callable("on_req_timer_timeout"),
        );

        // 标签显示
        let mut tags_container = self
            .base_mut()
            .get_node_as::<Control>(UniqueName::Tags.as_ref());
        for s in VoltageState::iter() {
            let mut tag_scene = self.tag_scene.instantiate_as::<VoltageStateTagView>();
            tags_container.add_child(tag_scene.clone().upcast());

            let mut tag = tag_scene.bind_mut();
            tag.set_color(s.color());
            tag.set_label(s.to_string().into());
            tag.update_ui();
        }

        // 开启
        let mut start_btn = self
            .base()
            .get_node_as::<Button>(UniqueName::StartToggle.as_ref());
        start_btn.connect(
            "pressed".into(),
            self.base().callable("on_start_toggle_sync"),
        );

        let mut chart = self
            .base()
            .get_node_as::<ChartView>(UniqueName::Chart.as_ref());

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
            chart.set_x_coord(x_labels.into());
            chart.set_y_coord(y_labels.into());
            chart.set_x_label("s");
            chart.set_y_label("V");
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
            .get_node_as::<Button>(UniqueName::StartToggle.as_ref());

        let mut timer = self
            .base()
            .get_node_as::<Timer>(UniqueName::ReqTimer.as_ref());
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
        let voltage = match self.ab {
            AB::Apanel => config.voltage_a.clone(),
            AB::Bpanel => config.voltage_b.clone(),
        };

        let mut label_ab_name = self
            .base()
            .get_node_as::<Label>(UniqueName::AbName.as_ref());
        label_ab_name.set_text(self.ab.title().into());

        let mut timer = self
            .base()
            .get_node_as::<Timer>(UniqueName::ReqTimer.as_ref());

        godot_print!("voltage init -- {:?}", voltage);

        let data: Vec<VoltageData> = (voltage.slave_start..=voltage.slave_end)
            .into_iter()
            .map(|slave| {
                let data = match get_voltage_data(&voltage, slave) {
                    Ok(d) => d,
                    Err(e) => {
                        timer.stop();

                        let mut start_btn = self
                            .base()
                            .get_node_as::<Button>(UniqueName::StartToggle.as_ref());
                        start_btn.set_text("开始".into());

                        godot_print!(" Write failed {:?}: {:?}", voltage, e);
                        VoltageData::new(0, Vec::new())
                    }
                };

                data.clone()
            })
            .collect();

        // self.data = Some(data.clone());
        // godot_print!(" Write failed {:?}", data);

        let mut content = self
            .base_mut()
            .get_node_as::<Control>(UniqueName::Container.as_ref());
        let has = content.get_child_count() == (VOLTAGE_CHANNEL * data.len()) as i32;

        for (j, data) in data.iter().enumerate() {
            for (i, data) in data.data.iter().enumerate() {
                let name = format!("i{}", i + j).to_godot();
                let mut channel = if !has {
                    let mut channel_scene =
                        self.channel_scene.instantiate_as::<VoltageChannelView>();
                    content.add_child(channel_scene.clone().upcast());
                    channel_scene.set_name(name);
                    channel_scene
                } else {
                    let channel_scene = content.get_node_as::<VoltageChannelView>(name);
                    channel_scene
                };

                {
                    let color = get_mb_state(data, &voltage.verify).color();
                    let mut channel = channel.bind_mut();

                    channel.set_index(j);
                    channel.set_color(color);
                    channel.set_data(data);
                    channel.update_ui();
                }

                channel.emit_signal("update_data".into(), &[]);
            }
        }

        self.base_mut().emit_signal("mb_read_over".into(), &[]);
    }
}

#[derive(AsRefStr, Debug)]
#[strum(prefix = "%")]
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

impl std::fmt::Display for UniqueName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
