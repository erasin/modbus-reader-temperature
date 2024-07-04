use std::time::Duration;

use godot::{
    engine::{Button, IPanelContainer, ItemList, Label, LineEdit, OptionButton, PanelContainer},
    obj::WithBaseField,
    prelude::*,
};
use mb::utils::{hms_from_duration, hms_from_duration_string, time_from_hms};
use strum::{AsRefStr, VariantArray};

use mb_data::{
    config::Config,
    db::{get_db, task::TableTask},
    task::{PowerMode, Task, TaskItem, AB},
};

use crate::{
    scenes::my_global::get_global_config,
    utils::{string_cut, string_number_only},
};

#[derive(GodotClass)]
#[class(init,base=PanelContainer)]
pub struct ProgramsView {
    #[var]
    task_items_str: Array<GString>,
    #[var]
    task_list_str: Array<GString>,

    config: Config,
    /// 当前 A/B面的 list
    list: Vec<Task>,
    /// 当前编辑的 task
    task: Task,
    /// 当前编辑的 item
    item: TaskItem,

    base: Base<PanelContainer>,
}

#[godot_api]
impl IPanelContainer for ProgramsView {
    fn ready(&mut self) {
        godot_print!("programs ready");
        self.config = get_global_config();

        let mut check_a_btn = self
            .base()
            .get_node_as::<Button>(UniqueName::CheckA.as_ref());

        check_a_btn.connect("pressed".into(), self.base().callable("on_check_a"));

        let mut check_b_btn = self
            .base()
            .get_node_as::<Button>(UniqueName::CheckB.as_ref());

        check_b_btn.connect("pressed".into(), self.base().callable("on_check_b"));

        // read config
        if self.config.enable_a_panel {
            self.task.ab = AB::A;
            check_a_btn.set_disabled(true);
        } else {
            check_a_btn.set_visible(false);
            self.task.ab = AB::B;
            check_b_btn.set_disabled(true);
        }

        if !self.config.enable_b_panel {
            check_b_btn.set_visible(false);
        }

        self.task.task_loop = 1;
        self.task.power.voltage = 220;

        let mut temp_node = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::Temp.as_ref());

        temp_node.connect(
            "text_changed".into(),
            self.base().callable("on_temp_number"),
        );

        let mut voltage_top_node = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::VoltageTop.as_ref());

        voltage_top_node.connect(
            "text_changed".into(),
            self.base().callable("on_voltage_top_number"),
        );

        let mut voltage_down_node = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::VoltageDown.as_ref());

        voltage_down_node.connect(
            "text_changed".into(),
            self.base().callable("on_voltage_down_number"),
        );

        let mut current_top_node = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::CurrentTop.as_ref());
        current_top_node.connect(
            "text_changed".into(),
            self.base().callable("on_current_top_number"),
        );

        let mut current_down_node = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::CurrentDown.as_ref());
        current_down_node.connect(
            "text_changed".into(),
            self.base().callable("on_current_down_number"),
        );

        let mut power_mode_node = self
            .base()
            .get_node_as::<OptionButton>(UniqueName::PowerType.as_ref());
        power_mode_node.connect(
            "item_selected".into(),
            self.base().callable("on_power_mode_selected"),
        );

        let mut power_voltage_node = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::PowerVoltage.as_ref());
        power_voltage_node.connect(
            "text_changed".into(),
            self.base().callable("on_power_voltage_number"),
        );

        let mut power_current_node = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::PowerCurrent.as_ref());
        power_current_node.connect(
            "text_changed".into(),
            self.base().callable("on_power_current_number"),
        );

        let mut task_loop_node = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::TaskLoop.as_ref());
        task_loop_node.connect(
            "text_changed".into(),
            self.base().callable("on_task_loop_number"),
        );

        let mut task_items_node = self
            .base()
            .get_node_as::<ItemList>(UniqueName::TaskItems.as_ref());
        task_items_node.connect(
            "item_selected".into(),
            self.base().callable("on_task_item_selected"),
        );

        // ItemPowerVoltage
        let mut item_power_voltage_node = self
            .base()
            .get_node_as::<OptionButton>(UniqueName::ItemPowerVoltage.as_ref());
        item_power_voltage_node.connect(
            "item_selected".into(),
            self.base().callable("on_item_power_voltage_selected"),
        );
        self.item_power_voltage_update();

        let mut item_hours_node = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::ItemHours.as_ref());
        item_hours_node.connect(
            "text_changed".into(),
            self.base().callable("on_item_hours_number"),
        );

        let mut item_minutes_node = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::ItemMinutes.as_ref());
        item_minutes_node.connect(
            "text_changed".into(),
            self.base().callable("on_item_minutes_number"),
        );

        let mut item_seconds_node = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::ItemSeconds.as_ref());
        item_seconds_node.connect(
            "text_changed".into(),
            self.base().callable("on_item_seconds_number"),
        );

        let mut item_save_node = self
            .base()
            .get_node_as::<Button>(UniqueName::ItemSave.as_ref());
        item_save_node.connect("pressed".into(), self.base().callable("on_item_save"));

        let mut item_edit_node = self
            .base()
            .get_node_as::<Button>(UniqueName::ItemEdit.as_ref());
        item_edit_node.connect("pressed".into(), self.base().callable("on_item_edit"));

        let mut item_delete_node = self
            .base()
            .get_node_as::<Button>(UniqueName::ItemDelete.as_ref());
        item_delete_node.connect("pressed".into(), self.base().callable("on_item_delete"));

        let mut item_clear_node = self
            .base()
            .get_node_as::<Button>(UniqueName::ItemClear.as_ref());
        item_clear_node.connect("pressed".into(), self.base().callable("on_item_clear"));

        let mut task_list_node = self
            .base()
            .get_node_as::<ItemList>(UniqueName::TaskList.as_ref());
        task_list_node.connect(
            "item_selected".into(),
            self.base().callable("on_task_list_selected"),
        );

        let mut task_save_node = self
            .base()
            .get_node_as::<Button>(UniqueName::TaskSave.as_ref());
        task_save_node.connect("pressed".into(), self.base().callable("on_task_save"));

        let mut task_edit_node = self
            .base()
            .get_node_as::<Button>(UniqueName::TaskEdit.as_ref());
        task_edit_node.connect("pressed".into(), self.base().callable("on_task_edit"));

        let mut task_delete_node = self
            .base()
            .get_node_as::<Button>(UniqueName::TaskDelete.as_ref());
        task_delete_node.connect("pressed".into(), self.base().callable("on_task_delete"));

        let mut task_load_node = self
            .base()
            .get_node_as::<Button>(UniqueName::TaskLoad.as_ref());
        task_load_node.connect("pressed".into(), self.base().callable("on_task_load"));

        self.task_items_update();
        self.task_list_update();
    }
}

#[godot_api]
impl ProgramsView {
    #[signal]
    fn update_task_item_list();

    #[signal]
    fn update_task_list();

    #[func]
    fn on_check_a(&mut self) {
        self.task.ab = AB::A;

        let mut check_a_btn = self
            .base()
            .get_node_as::<Button>(UniqueName::CheckA.as_ref());

        check_a_btn.set_disabled(true);

        let mut check_b_btn = self
            .base()
            .get_node_as::<Button>(UniqueName::CheckB.as_ref());

        check_b_btn.set_disabled(false);
    }

    #[func]
    fn on_check_b(&mut self) {
        self.task.ab = AB::B;

        let mut check_a_btn = self
            .base()
            .get_node_as::<Button>(UniqueName::CheckA.as_ref());

        check_a_btn.set_disabled(false);

        let mut check_b_btn = self
            .base()
            .get_node_as::<Button>(UniqueName::CheckB.as_ref());

        check_b_btn.set_disabled(true);
    }

    #[func]
    fn on_temp_number(&mut self, text: String) {
        let mut number = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::Temp.as_ref());

        let text = string_number_only(text);
        let dur = text
            .parse::<u16>()
            .unwrap_or_default()
            .clamp(u16::MIN + 1, u16::MAX);

        self.task.temperature = dur;

        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_voltage_top_number(&mut self, text: String) {
        let mut number = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::VoltageTop.as_ref());

        let text = string_number_only(text);
        let dur = text
            .parse::<u32>()
            .unwrap_or_default()
            .clamp(u32::MIN + 1, u32::MAX);

        self.task.voltage_verify.voltage_top = dur as f32;

        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_voltage_down_number(&mut self, text: String) {
        let mut number = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::VoltageDown.as_ref());

        let text = string_number_only(text);
        let dur = text
            .parse::<u32>()
            .unwrap_or_default()
            .clamp(u32::MIN + 1, u32::MAX);

        self.task.voltage_verify.voltage_down = dur as f32;

        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_current_top_number(&mut self, text: String) {
        let mut number = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::CurrentTop.as_ref());

        let text = string_number_only(text);
        let dur = text
            .parse::<u32>()
            .unwrap_or_default()
            .clamp(u32::MIN + 1, u32::MAX);

        self.task.voltage_verify.current_top = dur as f32;

        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_current_down_number(&mut self, text: String) {
        let mut number = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::CurrentDown.as_ref());

        let text = string_number_only(text);
        let dur = text
            .parse::<u32>()
            .unwrap_or_default()
            .clamp(u32::MIN + 1, u32::MAX);

        self.task.voltage_verify.current_down = dur as f32;

        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_power_mode_selected(&mut self, index: u32) {
        match PowerMode::VARIANTS.get(index as usize) {
            Some(mode) => self.task.power.mode = mode.clone(),
            None => {}
        };
    }

    #[func]
    fn on_power_voltage_number(&mut self, text: String) {
        let mut number = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::PowerVoltage.as_ref());

        let text = string_number_only(text);
        let dur = text
            .parse::<u32>()
            .unwrap_or_default()
            .clamp(u32::MIN + 1, u32::MAX);

        self.task.power.voltage = dur;

        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);
        self.item_power_voltage_update();
    }

    #[func]
    fn on_power_current_number(&mut self, text: String) {
        let mut number = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::PowerCurrent.as_ref());

        let text = string_number_only(text);
        let dur = text
            .parse::<u32>()
            .unwrap_or_default()
            .clamp(u32::MIN + 1, u32::MAX);

        self.task.power.current = dur;

        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_task_loop_number(&mut self, text: String) {
        let mut number = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::TaskLoop.as_ref());

        let text = string_number_only(text);
        let dur = text
            .parse::<u32>()
            .unwrap_or_default()
            .clamp(u32::MIN + 1, u32::MAX);

        self.task.task_loop = dur;

        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);

        self.task_total_time();
    }

    #[func]
    fn on_item_power_voltage_selected(&mut self, index: u32) {
        godot_print!("power {index}");

        if index == 0 {
            self.item.power_on = false;
            self.item.voltage = 0;
        } else {
            self.item.power_on = true;
            self.item.voltage = self.task.power.voltage;
        }
    }

    #[func]
    fn on_item_hours_number(&mut self, text: String) {
        let mut number = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::ItemHours.as_ref());

        let text = string_number_only(text);
        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);

        // 更新时间
        self.item_total_time();
    }

    #[func]
    fn on_item_minutes_number(&mut self, text: String) {
        let mut number = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::ItemMinutes.as_ref());

        let text = string_number_only(text);
        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);

        // 更新时间
        self.item_total_time();
    }

    #[func]
    fn on_item_seconds_number(&mut self, text: String) {
        let mut number = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::ItemSeconds.as_ref());

        let text = string_number_only(text);
        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);

        // 更新时间
        self.item_total_time();
    }

    #[func]
    fn on_item_save(&mut self) {
        self.task.items.push(self.item.clone());
        self.task_items_update();
        self.task_total_time();
    }

    #[func]
    fn on_item_edit(&mut self) {}

    #[func]
    fn on_item_delete(&mut self) {}

    #[func]
    fn on_item_clear(&mut self) {}

    #[func]
    fn on_task_save(&mut self) {}

    #[func]
    fn on_task_edit(&mut self) {}

    #[func]
    fn on_task_delete(&mut self) {}

    #[func]
    fn on_task_load(&mut self) {}

    #[func]
    fn on_task_item_selected(&mut self, index: u32) {
        godot_print!("item {index}")
    }

    #[func]
    fn on_task_list_selected(&mut self, index: u32) {
        godot_print!("list {index}")
    }
}

impl ProgramsView {
    fn item_power_voltage_update(&mut self) {
        let mut item_power_voltage_node = self
            .base()
            .get_node_as::<OptionButton>(UniqueName::ItemPowerVoltage.as_ref());

        item_power_voltage_node.clear();
        let voltage = self.task.power.voltage;
        item_power_voltage_node.add_item("OFF".into());
        item_power_voltage_node.add_item(format!("{voltage}V").into());
    }

    // 计算时间
    fn item_total_time(&mut self) {
        let item_hours_node = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::ItemHours.as_ref());
        let text = item_hours_node.get_text();
        let text = string_number_only(text.to_string());
        let hours = text
            .parse::<u32>()
            .unwrap_or_default()
            .clamp(u32::MIN, u32::MAX) as u64;

        let item_minutes_node = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::ItemMinutes.as_ref());

        let text = item_minutes_node.get_text();
        let text = string_number_only(text.to_string());
        let minutes = text
            .parse::<u32>()
            .unwrap_or_default()
            .clamp(u32::MIN, u32::MAX) as u64;

        let item_seconds_node = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::ItemSeconds.as_ref());

        let text = item_seconds_node.get_text();
        let text = string_number_only(text.to_string());
        let seconds = text
            .parse::<u32>()
            .unwrap_or_default()
            .clamp(u32::MIN, u32::MAX) as u64;

        let dur = time_from_hms(hours, minutes, seconds);

        self.item.dur = dur;
    }

    fn task_total_time(&mut self) {
        let dur = self
            .task
            .items
            .iter()
            .map(|item| item.dur)
            .fold(Duration::from_secs(0), |sum, x| sum + x);

        let dur = dur * self.task.task_loop;

        let text = hms_from_duration_string(dur);

        let mut count_time_node = self
            .base()
            .get_node_as::<Label>(UniqueName::CountTime.as_ref());
        count_time_node.set_text(text.into());
    }

    fn task_list_update(&mut self) {
        {
            let db = get_db().lock().unwrap();
            self.list = match TableTask::list(&db, &self.task.ab) {
                Ok(list) => list,
                Err(_) => Vec::new(),
            };
        }

        self.task_list_str = Array::new();
        self.list.iter().for_each(|task| {
            self.task_list_str.push(task.title.clone().into());
        });

        // gdscript 中处理 add_item
        self.base_mut().emit_signal("update_task_list".into(), &[]);
    }

    fn task_items_update(&mut self) {
        let mut data = vec![[
            "序号".to_owned(),
            "电压".to_owned(),
            "项目".to_owned(),
            "老化时间".to_owned(),
        ]];

        self.task
            .items
            .iter()
            .enumerate()
            .map(|(index, task)| {
                [
                    index.to_string(),
                    task.voltage.to_string(),
                    match task.power_on {
                        true => "老化中",
                        false => "断电",
                    }
                    .to_owned(),
                    hms_from_duration_string(task.dur),
                ]
            })
            .for_each(|item| data.push(item));

        let s: Vec<String> = data
            .iter()
            .map(|a| {
                let s: String = a
                    .iter()
                    .enumerate()
                    .map(|(index, s)| match index {
                        0 => string_cut(s, 10),
                        1 => string_cut(s, 20),
                        2 => string_cut(s, 30),
                        _ => string_cut(s, 20),
                    })
                    .collect();
                s
            })
            .collect();

        self.task_items_str.clear();
        s.iter().for_each(|s| {
            self.task_items_str.push(s.into());
        });

        self.base_mut()
            .emit_signal("update_task_item_list".into(), &[]);
    }

    fn task_load(&mut self) {}
}

#[derive(AsRefStr, Debug)]
#[strum(prefix = "%")]
enum UniqueName {
    CheckA,
    CheckB,

    TaskName,
    Temp,

    VoltageTop,
    VoltageDown,
    CurrentTop,
    CurrentDown,

    PowerType,
    PowerVoltage,
    PowerCurrent,

    CountTime,
    TaskLoop,

    TaskItems,
    ItemPowerVoltage,
    ItemHours,
    ItemMinutes,
    ItemSeconds,

    ItemSave,
    ItemEdit,
    ItemDelete,
    ItemClear,

    TaskList,
    TaskSave,
    TaskEdit,
    TaskDelete,
    TaskLoad,
}
