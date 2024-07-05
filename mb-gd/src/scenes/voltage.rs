use channel::VoltageChannelView;
use godot::{
    engine::{Button, Control, GridContainer, IPanelContainer, Label, PanelContainer, Timer},
    obj::WithBaseField,
    prelude::*,
};
use mb::{
    utils::hms_from_duration_string,
    voltage::{VoltageData, VoltageState, VOLTAGE_CHANNEL},
};
use mb_data::task::Task;
use state_tag::VoltageStateTagView;
use strum::{AsRefStr, IntoEnumIterator};

use crate::{
    chart::ChartView, colors::IntoColor, data::AB, define_get_nodes, mb_sync::get_voltage_data,
    scenes::my_global::get_global_config,
};

use super::my_global::MyGlobal;

pub mod channel;
pub mod state_tag;

#[derive(GodotClass)]
#[class(init, base=PanelContainer)]
pub struct VoltageView {
    #[export]
    ab: AB,

    channel_scene: Gd<PackedScene>,
    tag_scene: Gd<PackedScene>,

    task: Option<Task>,

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

        let mut my_global = MyGlobal::singleton();
        my_global.connect(
            "task_updated".into(),
            self.base().callable("on_global_task_updated"),
        );

        let mut label_ab_name = self.get_ab_name_node();
        label_ab_name.set_text(self.ab.title().into());

        let mut req_timer = self.base().get_node_as::<Timer>("ReqTimer");
        req_timer.connect(
            "timeout".into(),
            self.base().callable("on_req_timer_timeout"),
        );

        // 标签显示
        let mut tags_container = self.get_tags_node();
        for s in VoltageState::iter() {
            let mut tag_scene = self.tag_scene.instantiate_as::<VoltageStateTagView>();
            tags_container.add_child(tag_scene.clone().upcast());

            let mut tag = tag_scene.bind_mut();
            tag.set_color(s.color());
            tag.set_label(s.to_string().into());
            tag.update_ui();
        }

        // 开启
        self.get_start_toggle_node()
            .connect("pressed".into(), self.base().callable("on_start_toggle"));

        self.get_ageing_toggle_node()
            .connect("pressed".into(), self.base().callable("on_ageing_toggle"));

        self.get_power_toggle_node()
            .connect("pressed".into(), self.base().callable("on_power_toggle"));

        self.task_update();

        let mut chart = self.get_chart_node();
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
    fn on_global_task_updated(&mut self, ab: crate::data::AB) {
        if self.ab != ab {
            return;
        }

        let my_global = MyGlobal::singleton();
        self.task = my_global.bind().get_task(ab);
        self.task_update();
    }

    #[func]
    fn on_req_timer_timeout(&mut self) {
        self.task_run();
    }

    #[func]
    fn on_start_toggle(&mut self) {
        godot_print!("--- start pull");
        let mut start_btn = self.get_start_toggle_node();
        let mut timer = self.get_req_timer_node();
        if timer.is_stopped() {
            timer.start();
            start_btn.set_text("停止".into());
        } else {
            timer.stop();
            start_btn.set_text("开始".into());
        }
    }

    /// 老化
    #[func]
    fn on_ageing_toggle(&mut self) {}

    #[func]
    fn on_power_toggle(&mut self) {}
}

impl VoltageView {
    fn task_update(&mut self) {
        // 渲染 解锁按钮
        // let mut start_btn = self.get_start_toggle_node();
        let mut ageing_btn = self.get_ageing_toggle_node();
        let mut pro_name_node = self.get_pro_name_node();
        let mut ageing_time_node = self.get_ageing_time_node();
        let mut power_state_node = self.get_power_state_node();

        match &self.task {
            Some(task) => {
                // start_btn.set_disabled(false);
                ageing_btn.set_disabled(false);
                pro_name_node.set_text(task.title.clone().into());
                ageing_time_node.set_text(hms_from_duration_string(task.count_time).into());
                power_state_node.set_text(
                    format!("{} {}V", task.power.mode.as_ref(), task.power.voltage).into(),
                );
            }
            None => {
                // start_btn.set_disabled(true);
                ageing_btn.set_disabled(true);
                pro_name_node.set_text("".into());
                ageing_time_node.set_text("".into());
            }
        }
    }

    /// 老化中的状态监控
    fn task_state_update(&mut self) {}

    /// 电源开启后监控
    fn power_state_update(&mut self) {}

    /// 读取老化数据
    fn task_run(&mut self) {
        let config = get_global_config();

        //根据 AB 区 获取参数
        let voltage = match self.ab {
            AB::Apanel => config.voltage_a.clone(),
            AB::Bpanel => config.voltage_b.clone(),
        };

        let mut label_ab_name = self.get_ab_name_node();
        label_ab_name.set_text(self.ab.title().into());

        let mut timer = self.get_req_timer_node();

        // godot_print!("voltage init -- {:?}", voltage);

        let data: Vec<VoltageData> = (voltage.slave_start..=voltage.slave_end)
            .into_iter()
            .map(|slave| {
                let data = match get_voltage_data(&voltage, slave) {
                    Ok(d) => {
                        let mut data = d;
                        data.update_channel_state(&voltage.verify);
                        data
                    }
                    Err(e) => {
                        timer.stop();

                        let mut start_btn = self.get_start_toggle_node();
                        start_btn.set_text("开始".into());

                        godot_print!(" Write failed {:?}: {:?}", voltage, e);
                        VoltageData::new(0, 0, Vec::new())
                    }
                };

                data.clone()
            })
            .collect();

        // self.data = Some(data.clone());
        // godot_print!(" Write failed {:?}", data);

        let mut content = self.get_container_node();
        let has = content.get_child_count() == (VOLTAGE_CHANNEL * data.len()) as i32;

        for (j, data) in data.iter().enumerate() {
            for (i, data) in data.data.iter().enumerate() {
                let index = i + j * VOLTAGE_CHANNEL;
                let name = format!("i{}", index).to_godot();
                // godot_print!(" Write failed {:?} {i} {j}", name);
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
                    // let color = get_mb_state(data, &voltage.verify).color();
                    let color = data.state.color();
                    let mut channel = channel.bind_mut();

                    channel.set_index(index);
                    channel.set_color(color);
                    channel.set_data(data);
                    channel.update_ui();
                }

                channel.emit_signal("update_data".into(), &[]);
            }
        }

        self.base_mut().emit_signal("mb_read_over".into(), &[]);
    }

    define_get_nodes![
        (get_tags_node, UniqueName::Tags, Control),
        (get_req_timer_node, UniqueName::ReqTimer, Timer),
        (get_container_node, UniqueName::Container, GridContainer),
        (get_start_toggle_node, UniqueName::StartToggle, Button),
        (get_ageing_toggle_node, UniqueName::AgeingToggle, Button),
        (get_power_toggle_node, UniqueName::PowerToggle, Button),
        (get_pro_name_node, UniqueName::ProName, Label),
        (get_start_time_node, UniqueName::StartTime, Label),
        (get_count_down_time_node, UniqueName::CountDownTime, Label),
        (get_ageing_time_node, UniqueName::AgeingTime, Label),
        (get_power_state_node, UniqueName::PowerState, Label),
        (get_count_num_node, UniqueName::CountNum, Label),
        (get_count_good_node, UniqueName::CountGood, Label),
        (get_count_defective_node, UniqueName::CountDefective, Label),
        (get_ab_name_node, UniqueName::AbName, Label),
        (get_state_run_node, UniqueName::StateRun, Label),
        (get_state_error_node, UniqueName::StateError, Label),
        (get_state_ageing_node, UniqueName::StateAgeing, Label),
        (get_state_power_node, UniqueName::StatePower, Label),
        (get_chart_node, UniqueName::Chart, ChartView),
    ];
}

#[derive(AsRefStr, Debug)]
#[strum(prefix = "%")]
enum UniqueName {
    Tags,
    ReqTimer,
    Container,
    StartToggle,
    AgeingToggle,
    PowerToggle,

    ProName,
    StartTime,
    CountDownTime,
    AgeingTime,
    PowerState,
    CountNum,
    CountGood,
    CountDefective,

    AbName,
    StateRun,
    StateError,
    StateAgeing,
    StatePower,

    Chart,
}

impl std::fmt::Display for UniqueName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
