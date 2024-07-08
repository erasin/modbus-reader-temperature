use std::{
    ops::{MulAssign, Sub},
    time::Duration,
};

use channel::VoltageChannelView;
use godot::{
    engine::{
        Button, ColorPicker, Control, GridContainer, IPanelContainer, Label, Os, PanelContainer,
        Timer,
    },
    obj::WithBaseField,
    prelude::*,
};
use mb::{
    utils::{current_timestamp, hms_from_duration_string},
    voltage::{VoltageData, VoltageState, VOLTAGE_CHANNEL},
};
use mb_data::{
    db::{
        get_db,
        voltage::{TableVoltage, VoltageDataGroup},
    },
    task::Task,
    utils::{time_dur_odt, time_format_human, time_human},
};
use state_tag::VoltageStateTagView;
use strum::{AsRefStr, IntoEnumIterator};

use crate::{
    chart::ChartView,
    colors::{ColorPlate, IntoColor},
    data::AB,
    define_get_nodes,
    mb_sync::{get_temperature, get_voltage_data},
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

    state_run: bool,
    state_age: bool,
    state_power: bool,

    // TODO 合并到 state_age enum
    age_error: bool,
    power_error: bool,

    start_at: Duration,
    end_at: Duration,
    count_time: Duration,
    count_down: Duration,
    count_num: u64,
    count_good: u64,
    count_defective: u64,

    chart_data: Vec<Vector2>,

    // data: Option<VoltageData>,
    base: Base<PanelContainer>,
}

const CHANNEL_COL: i32 = 12;

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

        self.get_ab_name_node().set_text(self.ab.title().into());

        self.get_req_timer_node().connect(
            "timeout".into(),
            self.base().callable("on_req_timer_timeout"),
        );

        self.get_age_timer_node().connect(
            "timeout".into(),
            self.base().callable("on_age_timer_timeout"),
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
        let mut start_btn = self.get_start_toggle_node();
        start_btn.connect("pressed".into(), self.base().callable("on_start_toggle"));
        start_btn.set_disabled(true);

        let mut age_btn = self.get_ageing_toggle_node();
        age_btn.connect("pressed".into(), self.base().callable("on_ageing_toggle"));
        age_btn.set_disabled(true);

        let mut power_btn = self.get_power_toggle_node();
        power_btn.connect("pressed".into(), self.base().callable("on_power_toggle"));
        power_btn.set_disabled(true);

        let mut chart = self.get_chart_node();
        {
            let mut chart = chart.bind_mut();
            let points = [];
            let x_labels = [0., 10., 20., 30., 40., 50., 60.0];
            let y_labels = [0., 20., 60., 80., 100., 110., 160., 240.0];

            chart.set_points(points.into());
            chart.set_x_coord(x_labels.into());
            chart.set_y_coord(y_labels.into());
            chart.set_x_label("s");
            chart.set_y_label("V");
        }

        self.task_update();
        self.channel_init();
    }

    fn process(&mut self, delta: f64) {
        self.content_size_update();
        self.state_update();
        // if self.state_power {
        //     self.power_state_update();
        // }
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
        self.chart_update();
        self.state_update();
    }

    #[func]
    fn on_start_toggle(&mut self) {
        self.state_run = !self.state_run;
        self.btn_state_update();
    }

    /// 老化
    #[func]
    fn on_ageing_toggle(&mut self) {
        if !self.state_power {
            return;
        }

        self.state_age = !self.state_age;
        self.btn_state_update();
    }

    #[func]
    fn on_power_toggle(&mut self) {
        if !self.state_run {
            return;
        }

        self.state_power = !self.state_power;
        self.btn_state_update();
    }
}

impl VoltageView {
    /// 检查程序加载
    fn task_update(&mut self) {
        // 渲染 解锁按钮
        let mut start_btn = self.get_start_toggle_node();
        let mut age_btn = self.get_ageing_toggle_node();
        let mut power_btn = self.get_power_toggle_node();
        let mut pro_name_node = self.get_pro_name_node();
        let mut ageing_time_node = self.get_ageing_time_node();
        let mut power_state_node = self.get_power_state_node();
        let mut age_timer = self.get_age_timer_node();

        let task = match &self.task {
            Some(task) => task,
            None => {
                start_btn.set_disabled(true);
                age_btn.set_disabled(true);
                power_btn.set_disabled(true);
                pro_name_node.set_text("".into());
                ageing_time_node.set_text("".into());
                return;
            }
        };

        start_btn.set_disabled(false);
        pro_name_node.set_text(task.title.clone().into());
        ageing_time_node.set_text(hms_from_duration_string(task.count_time).into());
        self.count_time = task.count_time;
        self.count_down = task.count_time;
        power_state_node
            .set_text(format!("{} {}V", task.power.mode.as_ref(), task.power.voltage).into());

        age_timer.set_one_shot(true);
        age_timer.set_wait_time(task.count_time.as_secs_f64());
    }

    /// 老化中的状态监控
    fn task_state_update(&mut self) {}

    /// 电源开启后监控
    fn power_state_update(&mut self) {}

    fn btn_state_update(&mut self) {
        let mut start_btn = self.get_start_toggle_node();
        let mut age_btn = self.get_ageing_toggle_node();
        let mut power_btn = self.get_power_toggle_node();
        let mut req_timer = self.get_req_timer_node();
        let mut age_timer = self.get_age_timer_node();

        if self.state_run {
            power_btn.set_disabled(false);
            start_btn.set_text("停止".into());
        } else {
            age_btn.set_disabled(true);
            power_btn.set_disabled(true);
            start_btn.set_text("开始".into());
        }

        if self.state_power {
            age_btn.set_disabled(false);
            start_btn.set_disabled(true);
            power_btn.set_text("关闭电源".into());
        } else {
            start_btn.set_disabled(false);
            age_btn.set_disabled(true);
            power_btn.set_text("启动电源".into());
        }

        if self.state_age {
            req_timer.start();
            age_timer.start();
            self.start_at = current_timestamp();
            self.end_at = self.start_at + self.count_time;
            age_btn.set_text("停止老化".into());
            power_btn.set_disabled(true);
            start_btn.set_disabled(true);
        } else {
            req_timer.stop();
            power_btn.set_disabled(false);
            age_btn.set_text("开始老化".into());
        }
    }

    /// 创建 channel
    fn channel_init(&mut self) {
        let config = get_global_config();
        let mut container = self.get_container_node();

        //根据 AB 区 获取参数
        let voltage = match self.ab {
            AB::Apanel => config.voltage_a.clone(),
            AB::Bpanel => config.voltage_b.clone(),
        };

        let count = (voltage.slave_start..=voltage.slave_end).count() * VOLTAGE_CHANNEL;

        // 计算宽度
        self.count_num = count as u64;

        (0..count).for_each(|index| {
            let name = format!("i{}", index).to_godot();

            let mut channel_scene = self.channel_scene.instantiate_as::<VoltageChannelView>();
            container.add_child(channel_scene.clone().upcast());
            channel_scene.set_name(name);
            channel_scene.bind_mut().set_index(index);
            channel_scene.bind_mut().update_ui();
        });

        self.content_size_update()
    }

    fn content_size_update(&mut self) {
        let config = get_global_config();
        let win_size = self.base().get_window().unwrap().get_size();

        let max_size = if config.enable_a_panel && config.enable_b_panel {
            win_size.x / 2 - 300
        } else {
            win_size.x - 300
        } as f32;

        let mut container = self.get_container_node();
        let size = container.get_size();
        let col = container.get_columns();
        let channel_size = container.get_node_as::<VoltageChannelView>("i0").get_size();
        let min_size = channel_size.x * CHANNEL_COL as f32;

        if min_size >= max_size {
            container.set_columns(CHANNEL_COL);
            return;
        }

        let width = size.x.clamp(min_size, max_size);
        let col = (width / channel_size.x).floor() as i32;
        let col = col.clamp(CHANNEL_COL, 100);
        container.set_columns(col);
    }

    /// 读取老化数据
    fn task_run(&mut self) {
        let config = get_global_config();
        let task = self.task.clone().unwrap();

        //根据 AB 区 获取参数
        let voltage = match self.ab {
            AB::Apanel => config.voltage_a.clone(),
            AB::Bpanel => config.voltage_b.clone(),
        };

        let mut label_ab_name = self.get_ab_name_node();
        label_ab_name.set_text(self.ab.title().into());

        let temperature = match get_temperature(&config.temperature, self.ab) {
            Ok(t) => t.value,
            Err(e) => {
                self.on_ageing_toggle();
                log::error!("温度获取失败: {}", e.to_string());
                0.
            }
        };

        let data: Vec<VoltageData> = (voltage.slave_start..=voltage.slave_end)
            .map(|slave| match get_voltage_data(&voltage, slave) {
                Ok(d) => {
                    let mut data = d;
                    data.update_channel_state(&voltage.verify);
                    data
                }
                Err(e) => {
                    self.on_ageing_toggle();
                    log::error!("电压电流获取失败: {}", e.to_string());
                    VoltageData::new(Duration::from_secs(0), 0, Vec::new())
                }
            })
            .collect();

        if !data.is_empty() {
            let data_group = VoltageDataGroup {
                time: current_timestamp(),
                ab: self.ab.into(),
                // TODO 产品名称
                good_name: "".to_string(),
                task_name: task.title.clone(),
                start_at: self.start_at,
                task_age_time: task.count_time,
                temperature,
                data: data.clone(),
            };
            {
                let db = get_db().lock().unwrap();
                if let Err(e) = TableVoltage::set(&db, &data_group) {
                    log::error!("老化数据存储错误: {}", e.to_string());
                };
            }
        }

        let content = self.get_container_node();

        for (j, data) in data.iter().enumerate() {
            for (i, data) in data.data.iter().enumerate() {
                let index = i + j * VOLTAGE_CHANNEL;
                let name = format!("i{}", index).to_godot();
                let mut channel = content.get_node_as::<VoltageChannelView>(name);

                {
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

    /// 图表数据更新
    fn chart_update(&mut self) {
        let list = {
            let db = get_db().lock().unwrap();
            match TableVoltage::range_last(&db, self.ab.into(), 100) {
                Ok(data) => data,
                Err(e) => {
                    log::error!("老化数据读取失败: {}", e.to_string());
                    return;
                }
            }
        };

        let points = list
            .iter()
            .rev()
            .take(60)
            .enumerate()
            .map(|(index, data)| Vector2::new(index as f32, data.voltage()))
            .collect();

        let mut chart = self.get_chart_node();
        chart.bind_mut().set_points(points);
        chart.queue_redraw();
    }

    /// 状态更新
    fn state_update(&mut self) {
        if self.task.is_none() {
            return;
        }

        if self.state_age {
            self.count_down = self.end_at - current_timestamp();
        }

        let mut start_time = self.get_start_time_node();
        let mut count_down_time = self.get_count_down_time_node();
        let mut count_good = self.get_count_good_node();
        let mut count_defective = self.get_count_defective_node();

        start_time.set_text(time_human(time_dur_odt(self.start_at)).into());
        count_down_time.set_text(hms_from_duration_string(self.count_down).into());

        count_good.set_text(time_human(time_dur_odt(self.end_at)).into());
        count_defective.set_text(self.count_defective.to_string().into());

        let mut state_run_node = self.get_state_run_node();
        let mut state_error_node = self.get_state_error_node();
        let mut state_ageing_node = self.get_state_ageing_node();
        let mut state_power_node = self.get_state_power_node();

        let state_run = if self.state_run {
            ColorPlate::Green
        } else {
            ColorPlate::Grey
        };
        state_run_node.set_modulate(state_run.into());

        let state_age = if self.state_age {
            if self.age_error {
                ColorPlate::Red
            } else {
                ColorPlate::Green
            }
        } else {
            ColorPlate::Grey
        };
        state_ageing_node.set_modulate(state_age.into());

        let state_power = if self.state_power {
            if self.power_error {
                ColorPlate::Red
            } else {
                ColorPlate::Green
            }
        } else {
            ColorPlate::Grey
        };
        state_power_node.set_modulate(state_power.into());

        let state_error = if self.age_error || self.power_error {
            ColorPlate::Red
        } else {
            ColorPlate::Grey
        };

        state_error_node.set_modulate(state_error.into());
    }

    /// 保存文件
    fn save_history(&mut self) {}

    define_get_nodes![
        (get_voltage_container, UniqueName::VoltageContainer, Control),
        (get_req_timer_node, UniqueName::ReqTimer, Timer),
        (get_age_timer_node, UniqueName::AgeTimer, Timer),
        (get_tags_node, UniqueName::Tags, Control),
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
        (get_state_run_node, UniqueName::StateRun, Control),
        (get_state_error_node, UniqueName::StateError, Control),
        (get_state_ageing_node, UniqueName::StateAgeing, Control),
        (get_state_power_node, UniqueName::StatePower, Control),
        (get_chart_node, UniqueName::Chart, ChartView),
    ];
}

#[derive(AsRefStr, Debug)]
#[strum(prefix = "%")]
enum UniqueName {
    VoltageContainer,

    ReqTimer,
    AgeTimer,

    Tags,
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
