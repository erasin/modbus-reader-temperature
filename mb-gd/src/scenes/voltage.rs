use std::{ops::Add, path::PathBuf, time::Duration};

use channel::VoltageChannelView;
use godot::{
    classes::{Button, Control, GridContainer, IPanelContainer, Label, PanelContainer, Timer},
    obj::WithBaseField,
    prelude::*,
};
use mb::{Result, voltage::VoltageChannel};
use mb::{
    relay::RelayMode,
    utils::{current_timestamp, hms_from_duration_string},
    voltage::{VOLTAGE_CHANNEL, VoltageData, VoltageState},
};
use mb_data::{
    db::{
        get_db,
        voltage::{
            TableVoltage, VoltageDataGroup, check_defective_in_secs,
            voltage_average_every_n_minutes,
        },
    },
    dirs::doc_dir,
    task::Task,
    user::UserPurview,
    utils::{time_dur_odt, time_human, time_human_filename, time_now},
};
use rust_xlsxwriter::Workbook;
use state_tag::VoltageStateTagView;
use strum::{AsRefStr, IntoEnumIterator};

use crate::{
    chart::ChartView,
    colors::{ColorPlate, IntoColor},
    data::AB,
    define_get_nodes,
    mb_sync::{get_relay, get_temperature, get_voltage_data, set_relay},
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
    task_state: TaskState,

    // 状态
    state: State,

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

    /// 故障产品
    defective_data: Vec<VoltageChannel>,

    // data: Option<VoltageData>,
    base: Base<PanelContainer>,
}

#[derive(Debug, Default, Clone, Copy)]
struct TaskState {
    /// 运行到第几次循环
    loop_index: u32,
    /// 执行总时间
    count_time: Duration,
    /// 执行的 item 索引
    item_index: usize,
    /// 本次执行时间
    item_time: Duration,
    /// 电源状态
    power_on: bool,
}

/// 状态
#[derive(Debug, Default, PartialEq)]
enum State {
    /// 默认不可用
    #[default]
    Disable,
    /// 默认待机
    Wait,
    /// 运行
    Run,
    /// 电源开启
    Power,
    /// 老化
    Ageing,
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
            "task_updated",
            &self.base().callable("on_global_task_updated"),
        );

        let on_task_item_start = &self.base().callable("on_task_item_start");
        self.base_mut()
            .connect("task_item_start", on_task_item_start);

        self.get_ab_name_node().set_text(&self.ab.title());

        self.get_req_timer_node()
            .connect("timeout", &self.base().callable("on_req_timer_timeout"));

        self.get_age_timer_node()
            .connect("timeout", &self.base().callable("on_age_timer_timeout"));

        // 标签显示
        let mut tags_container = self.get_tags_node();
        for s in VoltageState::iter() {
            let mut tag_scene = self.tag_scene.instantiate_as::<VoltageStateTagView>();
            tags_container.add_child(&tag_scene);

            let mut tag = tag_scene.bind_mut();
            tag.set_color(s.color());
            tag.set_label(s.to_string().into());
            tag.update_ui();
        }

        // 开启
        let mut start_btn = self.get_start_toggle_node();
        start_btn.connect("pressed", &self.base().callable("on_start_toggle"));
        start_btn.set_disabled(true);

        let mut age_btn = self.get_ageing_toggle_node();
        age_btn.connect("pressed", &self.base().callable("on_ageing_toggle"));
        age_btn.set_disabled(true);

        let mut power_btn = self.get_power_toggle_node();
        power_btn.connect("pressed", &self.base().callable("on_power_toggle"));
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

        self.task_load();
        self.channel_init();
    }

    fn process(&mut self, delta: f64) {
        self.task_state_update();
        self.content_size_update();
        self.ui_state_update();
        self.btn_state_update();
        self.user_purview_update();
    }
}

#[godot_api]
impl VoltageView {
    #[signal]
    fn mb_read_over();

    /// 变动 item
    #[signal]
    fn task_item_start(index: u32);

    #[func]
    fn on_global_task_updated(&mut self, ab: crate::data::AB) {
        if self.ab != ab {
            return;
        }

        let my_global = MyGlobal::singleton();
        self.task = my_global.bind().get_task(ab);
        self.task_load();
    }

    /// 请求处理
    #[func]
    fn on_req_timer_timeout(&mut self) {
        self.task_run();
        self.chart_update();
        self.check_defective();
        self.power_state_update();
    }

    /// 老化结束处理
    #[func]
    fn on_age_timer_timeout(&mut self) {
        {
            self.state = State::Power;
        }

        self.check_defective();
        if let Err(e) = self.save_history() {
            log::error!("{}", e);
        };
    }

    #[func]
    fn on_start_toggle(&mut self) {
        self.state = match self.state {
            State::Wait => State::Run,
            State::Run => State::Wait,
            _ => {
                return;
            }
        };

        self.btn_state_update();
    }

    #[func]
    fn on_power_toggle(&mut self) {
        self.state = match self.state {
            State::Run => State::Power,
            State::Power => State::Run,
            _ => {
                return;
            }
        };

        // TODO 冲击电源kk开关

        self.btn_state_update();
    }

    /// 老化
    #[func]
    fn on_ageing_toggle(&mut self) {
        self.state = match self.state {
            State::Power => State::Ageing,
            State::Ageing => State::Power,
            _ => {
                return;
            }
        };

        self.btn_state_update();

        let mut req_timer = self.get_req_timer_node();
        let mut age_timer = self.get_age_timer_node();
        match self.state {
            State::Ageing => {
                age_timer.set_wait_time(self.count_time.as_secs_f64());
                req_timer.start();
                age_timer.start();
                self.start_at = current_timestamp();
                self.end_at = self.start_at + self.count_time;

                let item_index = self.task_state.item_index as u32;
                self.base_mut()
                    .emit_signal("task_item_start", &[item_index.to_variant()]);
            }
            _ => {
                req_timer.stop();
                age_timer.stop();
            }
        }
    }

    #[func]
    fn on_task_item_start(&mut self, index: u32) {
        if self.task.is_none() {
            return;
        }
        let config = get_global_config();
        let config = config.relay;

        log::debug!("继电器冲击：{index}");

        let task = self.task.clone().unwrap();
        if let Some(item) = task.items.get(index as usize) {
            let pos: u8 = match self.ab {
                AB::Apanel => 0,
                AB::Bpanel => 1,
            };

            self.task_state.power_on = item.power_on;

            // 获取继电器
            let relay = match get_relay(&config, self.ab) {
                Ok(data) => data,
                Err(e) => {
                    log::error!("继电器冲击失败： {e}");
                    return;
                }
            };

            let mode = if item.power_on {
                // 冲击开关
                RelayMode::ON(relay.value, pos)
            } else {
                RelayMode::OFF(relay.value, pos)
            };

            match set_relay(&config, self.ab, &mode) {
                Ok(data) => data,
                Err(e) => {
                    log::error!("继电器冲击失败： {e}");
                    return;
                }
            };
        }
    }
}

impl VoltageView {
    /// 检查程序加载
    fn task_load(&mut self) {
        // 渲染 解锁按钮
        let mut start_btn = self.get_start_toggle_node();
        let mut age_btn = self.get_ageing_toggle_node();
        let mut power_btn = self.get_power_toggle_node();
        let mut pro_name_node = self.get_task_name_node();
        let mut ageing_time_node = self.get_ageing_time_node();
        let mut power_state_node = self.get_power_state_node();
        let mut age_timer = self.get_age_timer_node();
        let mut product_title_node = self.get_product_title_node();
        let mut product_index_node = self.get_product_index_node();
        let mut count_num_node = self.get_count_num_node();

        let task = match &self.task {
            Some(task) => task,
            None => {
                start_btn.set_disabled(true);
                age_btn.set_disabled(true);
                power_btn.set_disabled(true);
                pro_name_node.set_text("");
                ageing_time_node.set_text("");
                return;
            }
        };

        start_btn.set_disabled(false);
        pro_name_node.set_text(&task.title.clone());
        ageing_time_node.set_text(&hms_from_duration_string(task.count_time));
        self.count_time = task.count_time;
        self.count_down = task.count_time;
        power_state_node.set_text(&format!(
            "{} {}V",
            task.power.mode.as_ref(),
            task.power.voltage
        ));

        product_title_node.set_text(&task.product.title.clone());
        product_index_node.set_text(&task.product.index.clone());
        count_num_node.set_text(&self.count_num.to_string());

        age_timer.set_one_shot(true);
        age_timer.set_wait_time(task.count_time.as_secs_f64());

        self.count_good = 0;
        self.count_defective = 0;

        self.state = State::Wait;

        self.task_state = TaskState {
            loop_index: 0,
            count_time: Duration::from_secs(0),
            item_index: 0,
            item_time: Duration::from_secs(0),
            power_on: false,
        }
    }

    /// 老化中的状态监控
    fn task_state_update(&mut self) {
        if self.task.is_none() {
            return;
        }

        let age_timer = self.get_age_timer_node();
        let age_time_left = Duration::from_secs_f64(age_timer.get_time_left());
        let age_time = Duration::from_secs_f64(age_timer.get_wait_time()) - age_time_left;
        // 更新运行时间
        if self.state == State::Ageing {
            self.count_down = age_time_left;
            // self.count_down = match self.end_at.cmp(&current_time) {
            //     std::cmp::Ordering::Less | std::cmp::Ordering::Equal => Duration::from_secs(0),
            //     std::cmp::Ordering::Greater => self.end_at.sub(current_time),
            // }
        } else {
            return;
        }

        let mut task_state = self.task_state;
        let start_at = self.start_at;

        let task = self.task.clone().unwrap();

        // 记录运行状态
        // 执行 Item
        let items_dur = task
            .items
            .iter()
            .enumerate()
            .filter(|(index, _)| index.le(&task_state.item_index))
            .fold(Duration::default(), |mut a, b| {
                a = a.add(b.1.dur);
                a
            });

        let one_loop_dur = task.get_items_time();

        let mut shot_task_item = false;

        // log::debug!(
        //     "TaskState：{:?}, {}, {}, {}",
        //     task_state,
        //     age_time.as_secs(),
        //     items_dur.as_secs(),
        //     one_loop_dur.as_secs(),
        // );

        // 如果 item 执行的时限超出一次循环
        // 获取已运行的计算的部分
        let dur = items_dur + task_state.loop_index * one_loop_dur;

        // 进入下一个循环
        if age_time.ge(&dur) {
            if task_state.item_index.lt(&(task.items.len() - 1)) {
                task_state.item_index += 1;
            } else {
                task_state.loop_index += 1;
                task_state.item_index = 0;
            }
            shot_task_item = true;
        }

        self.task_state = task_state;

        // 冲击电源
        if shot_task_item {
            let item_index = self.task_state.item_index as u32;
            self.base_mut()
                .emit_signal("task_item_start", &[item_index.to_variant()]);
        }
    }

    /// 电源开启后监控
    fn power_state_update(&mut self) {
        if self.task.is_none() {
            return;
        }

        // TODO 检查电源
    }

    fn btn_state_update(&mut self) {
        let mut start_btn = self.get_start_toggle_node();
        let mut age_btn = self.get_ageing_toggle_node();
        let mut power_btn = self.get_power_toggle_node();

        match self.state {
            State::Disable => {
                start_btn.set_disabled(true);
                age_btn.set_disabled(true);
                power_btn.set_disabled(true);
            }
            State::Wait => {
                start_btn.set_disabled(false);
                age_btn.set_disabled(true);
                power_btn.set_disabled(true);

                start_btn.set_text("开始");
                power_btn.set_text("启动电源");
                age_btn.set_text("开始老化");
            }
            State::Run => {
                start_btn.set_disabled(false);
                power_btn.set_disabled(false);
                age_btn.set_disabled(true);

                start_btn.set_text("停止");
                power_btn.set_text("启动电源");
                age_btn.set_text("开始老化");
            }
            State::Power => {
                start_btn.set_disabled(true);
                power_btn.set_disabled(false);
                age_btn.set_disabled(false);

                power_btn.set_text("关闭电源");
                age_btn.set_text("开始老化");
            }
            State::Ageing => {
                age_btn.set_disabled(false);
                power_btn.set_disabled(true);
                start_btn.set_disabled(true);

                age_btn.set_text("停止老化");
            }
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
            container.add_child(&channel_scene);
            channel_scene.set_name(&name);
            channel_scene.bind_mut().set_index(index);
            channel_scene.bind_mut().update_ui();
        });

        self.content_size_update()
    }

    /// 按照窗口大小变动 channel 列
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
        if self.state != State::Ageing {
            self.get_req_timer_node().stop();
            return;
        }

        // 电源关闭情况下不予检查
        if !self.task_state.power_on {
            // TODO 是否重置 channel
            return;
        }

        let config = get_global_config();
        let task = self.task.clone().unwrap();

        //根据 AB 区 获取参数
        let voltage = match self.ab {
            AB::Apanel => config.voltage_a.clone(),
            AB::Bpanel => config.voltage_b.clone(),
        };

        let mut label_ab_name = self.get_ab_name_node();
        label_ab_name.set_text(&self.ab.title());

        let temperature = match get_temperature(&config.temperature, self.ab) {
            Ok(t) => t.value,
            Err(e) => {
                self.on_ageing_toggle();
                log::error!("温度获取失败: {}", e);
                0.
            }
        };

        let data: Vec<VoltageData> = (voltage.slave_start..=voltage.slave_end)
            .enumerate()
            .map(|(index, slave)| match get_voltage_data(&voltage, slave) {
                Ok(d) => {
                    let mut data = d;
                    data.update_channel_state(&voltage.verify);
                    data.update_channel_index(index);
                    data
                }
                Err(e) => {
                    self.on_ageing_toggle();
                    log::error!("电压电流获取失败: {}", e);
                    VoltageData::new(Duration::from_secs(0), 0, Vec::new())
                }
            })
            .collect();

        if !data.is_empty() {
            let data_group = VoltageDataGroup {
                time: current_timestamp(),
                ab: self.ab.into(),
                good_name: format!("{}_{}", task.product.title, task.product.index),
                task_name: task.title.clone(),
                start_at: self.start_at,
                task_age_time: task.count_time,
                temperature,
                data: data.clone(),
            };
            {
                let db = get_db().lock().unwrap();
                if let Err(e) = TableVoltage::set(&db, &data_group) {
                    log::error!("老化数据存储错误: {}", e);
                };
            }
        }

        let content = self.get_container_node();

        for (j, data) in data.iter().enumerate() {
            for (i, data) in data.data.iter().enumerate() {
                let index = i + j * VOLTAGE_CHANNEL;
                let name = format!("i{}", index);
                let mut channel = content.get_node_as::<VoltageChannelView>(&name);

                {
                    let color = data.state.color();
                    let mut channel = channel.bind_mut();

                    channel.set_index(index);
                    channel.set_color(color);
                    channel.set_data(data);
                    channel.update_ui();
                }

                channel.emit_signal("update_data", &[]);
            }
        }

        self.base_mut().emit_signal("mb_read_over", &[]);
    }

    /// 图表数据更新
    fn chart_update(&mut self) {
        // TODO  数据取材？
        let list = {
            let db = get_db().lock().unwrap();
            match TableVoltage::range_last(&db, self.ab.into(), 100) {
                Ok(data) => data,
                Err(e) => {
                    log::error!("老化数据读取失败: {}", e);
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
    fn ui_state_update(&mut self) {
        if self.task.is_none() {
            return;
        }

        let mut start_time = self.get_start_time_node();
        let mut count_down_time = self.get_count_down_time_node();

        start_time.set_text(&time_human(time_dur_odt(self.start_at)));
        count_down_time.set_text(&hms_from_duration_string(self.count_down));

        let mut count_good = self.get_count_good_node();
        let mut count_defective = self.get_count_defective_node();
        count_good.set_text(&self.count_good.to_string());
        count_defective.set_text(&self.count_defective.to_string());

        let mut state_run_node = self.get_state_run_node();
        let mut state_error_node = self.get_state_error_node();
        let mut state_ageing_node = self.get_state_ageing_node();
        let mut state_power_node = self.get_state_power_node();

        match self.state {
            State::Disable | State::Wait => {
                state_run_node.set_modulate(ColorPlate::Grey.into());
                state_power_node.set_modulate(ColorPlate::Grey.into());
                state_ageing_node.set_modulate(ColorPlate::Grey.into());
            }
            State::Run => {
                state_run_node.set_modulate(ColorPlate::Green.into());
                state_power_node.set_modulate(ColorPlate::Grey.into());
                state_ageing_node.set_modulate(ColorPlate::Grey.into());
            }
            State::Power => {
                state_run_node.set_modulate(ColorPlate::Green.into());

                let power_color = if self.power_error {
                    ColorPlate::Red
                } else {
                    ColorPlate::Green
                };

                state_power_node.set_modulate(power_color.into());

                state_ageing_node.set_modulate(ColorPlate::Grey.into());
            }
            State::Ageing => {
                state_run_node.set_modulate(ColorPlate::Green.into());
                state_power_node.set_modulate(ColorPlate::Green.into());

                let age_color = if self.age_error {
                    ColorPlate::Red
                } else {
                    ColorPlate::Green
                };
                state_ageing_node.set_modulate(age_color.into());
            }
        }

        let state_error = if self.age_error || self.power_error {
            ColorPlate::Red
        } else {
            ColorPlate::Grey
        };

        state_error_node.set_modulate(state_error.into());
    }

    /// 计算良品率
    fn check_defective(&mut self) {
        if self.task.is_none() {
            return;
        }

        let config = get_global_config();
        let defective_config = config.defective;
        let verify = match self.ab {
            AB::Apanel => config.voltage_a.verify,
            AB::Bpanel => config.voltage_b.verify,
        };

        let list = {
            let db = get_db().lock().unwrap();
            match defective_config.rule {
                mb_data::config::DefectiveRule::RealTime => {
                    match TableVoltage::get_last(&db, self.ab.into()) {
                        Ok(d) => vec![d],
                        Err(_) => {
                            return;
                        }
                    }
                }
                mb_data::config::DefectiveRule::InTime => {
                    match TableVoltage::range_last(&db, self.ab.into(), defective_config.dur) {
                        Ok(data) => data,
                        Err(e) => {
                            log::error!("老化数据读取失败: {}", e);
                            return;
                        }
                    }
                }
            }
        };
        // godot_print!("list -->{:?}", list);
        // TODO 实时检查
        // TODO 延时检查

        // godot_print!("gef-->{:?}", list);
        let dur = match defective_config.rule {
            mb_data::config::DefectiveRule::RealTime => 0,
            mb_data::config::DefectiveRule::InTime => defective_config.dur,
        };

        let list = check_defective_in_secs(list, dur, &verify);

        // godot_print!("def-->{:?}", list);

        // 与当前的对比
        if self.defective_data.is_empty() {
            self.defective_data = list.clone();
        } else {
            list.iter().for_each(|&ch| {
                if !self.defective_data.iter().any(|x| x.index != ch.index) {
                    self.defective_data.push(ch);
                }
            });
        }

        self.count_defective = self.defective_data.len() as u64;
        // 计算良品
        self.count_good = self.count_num - self.count_defective;

        let mut count_good = self.get_count_good_node();
        let mut count_defective = self.get_count_defective_node();
        count_good.set_text(&self.count_good.to_string());
        count_defective.set_text(&self.count_defective.to_string());
    }

    /// 保存文件
    fn save_history(&mut self) -> Result<()> {
        let config = get_global_config();
        // 保存

        // 文件路径
        let mut doc_path = if config.history.export_dir.is_empty() {
            doc_dir()
        } else {
            PathBuf::from(config.history.export_dir)
        };

        let time_name = time_human_filename(time_now());
        //产品名称
        let task = self.task.clone().unwrap();
        let pro_name = task.product.title;
        let pro_id = task.product.index;
        let file_name = format!("{pro_name}_{pro_id}_{time_name}.xlsx");
        doc_path.push(file_name);

        // 数据
        // 计算每分钟的品均值
        // 保存为
        let list = {
            let db = get_db().lock().unwrap();
            TableVoltage::list(&db, self.ab.into())?
        };
        log::debug!("list - {:?} ", list.len());

        let list = voltage_average_every_n_minutes(list, config.history.log_dur as u64);

        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();

        worksheet.write(0, 0, "Channel")?;
        worksheet.write(0, 1, "电压")?;
        worksheet.write(0, 2, "电流")?;
        worksheet.write(0, 3, "时间")?;

        for item in list.iter() {
            let channel = item.ch;
            let row = item.index as u32 + 1;
            worksheet.write(row, 0, channel.index as u64)?;
            worksheet.write(row, 1, channel.voltage)?;
            worksheet.write(row, 2, channel.current)?;
            worksheet.write(row, 3, time_human(time_dur_odt(item.time)))?;
        }

        workbook.save(doc_path)?;

        self.clear_old_data();
        Ok(())
    }

    /// 清理旧数据
    fn clear_old_data(&mut self) {
        let config = get_global_config();
        {
            let db = get_db().lock().unwrap();
            if let Err(e) = TableVoltage::clean(&db, self.ab.into()) {
                log::error!("清理数据错误：{e}")
            }
        };
    }

    /// 权限
    fn user_purview_update(&mut self) {
        let mut purview_run = self.get_purview_run_node();
        let g = MyGlobal::singleton();
        match g.bind().get_login() {
            Some(user) => {
                user.purview.iter().for_each(|p| {
                    if let UserPurview::Run = p {
                        purview_run.set_visible(true)
                    }
                });
            }
            None => {
                purview_run.set_visible(false);
            }
        };
    }

    define_get_nodes![
        (get_voltage_container, UniqueName::VoltageContainer, Control),
        (get_req_timer_node, UniqueName::ReqTimer, Timer),
        (get_age_timer_node, UniqueName::AgeTimer, Timer),
        (get_tags_node, UniqueName::Tags, Control),
        (get_container_node, UniqueName::Container, GridContainer),
        (get_purview_run_node, UniqueName::PurviewRun, PanelContainer),
        (get_start_toggle_node, UniqueName::StartToggle, Button),
        (get_ageing_toggle_node, UniqueName::AgeingToggle, Button),
        (get_power_toggle_node, UniqueName::PowerToggle, Button),
        (get_task_name_node, UniqueName::TaskName, Label),
        (get_start_time_node, UniqueName::StartTime, Label),
        (get_count_down_time_node, UniqueName::CountDownTime, Label),
        (get_ageing_time_node, UniqueName::AgeingTime, Label),
        (get_power_state_node, UniqueName::PowerState, Label),
        (get_count_num_node, UniqueName::CountNum, Label),
        (get_count_good_node, UniqueName::CountGood, Label),
        (get_count_defective_node, UniqueName::CountDefective, Label),
        (get_product_title_node, UniqueName::ProductTitle, Label),
        (get_product_index_node, UniqueName::ProductIndex, Label),
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
    PurviewRun,
    StartToggle,
    AgeingToggle,
    PowerToggle,

    TaskName,
    StartTime,
    CountDownTime,
    AgeingTime,
    PowerState,
    CountNum,
    CountGood,
    CountDefective,
    ProductTitle,
    ProductIndex,

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
