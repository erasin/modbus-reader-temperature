use std::time::Duration;

use godot::{
    engine::{
        AcceptDialog, Button, IPanelContainer, ItemList, Label, LineEdit, OptionButton,
        PanelContainer,
    },
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
    define_get_nodes,
    scenes::my_global::get_global_config,
    utils::{string_cut, string_number_only},
};

use super::my_global::MyGlobal;

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
        // godot_print!("programs ready");
        self.config = get_global_config();

        let mut check_a_btn = self.get_check_a_node();
        check_a_btn.connect("pressed".into(), self.base().callable("on_check_a"));

        let mut check_b_btn = self.get_check_b_node();
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

        let mut task_title_node = self.get_task_name_node();
        task_title_node.connect("text_changed".into(), self.base().callable("on_task_name"));
        task_title_node.grab_focus();

        self.get_temp_node().connect(
            "text_changed".into(),
            self.base().callable("on_temp_number"),
        );

        self.get_voltage_top_node().connect(
            "text_changed".into(),
            self.base().callable("on_voltage_top_number"),
        );

        self.get_voltage_down_node().connect(
            "text_changed".into(),
            self.base().callable("on_voltage_down_number"),
        );

        self.get_current_top_node().connect(
            "text_changed".into(),
            self.base().callable("on_current_top_number"),
        );

        self.get_current_down_node().connect(
            "text_changed".into(),
            self.base().callable("on_current_down_number"),
        );

        self.get_power_type_node().connect(
            "item_selected".into(),
            self.base().callable("on_power_mode_selected"),
        );

        self.get_power_voltage_node().connect(
            "text_changed".into(),
            self.base().callable("on_power_voltage_number"),
        );

        self.get_power_current_node().connect(
            "text_changed".into(),
            self.base().callable("on_power_current_number"),
        );

        self.get_task_loop_node().connect(
            "text_changed".into(),
            self.base().callable("on_task_loop_number"),
        );

        self.get_task_items_node().connect(
            "item_selected".into(),
            self.base().callable("on_task_item_selected"),
        );

        // ItemPowerVoltage
        self.get_item_power_voltage_node().connect(
            "item_selected".into(),
            self.base().callable("on_item_power_voltage_selected"),
        );
        self.item_power_voltage_update();

        self.get_item_hours_node().connect(
            "text_changed".into(),
            self.base().callable("on_item_hours_number"),
        );

        self.get_item_minutes_node().connect(
            "text_changed".into(),
            self.base().callable("on_item_minutes_number"),
        );

        self.get_item_seconds_node().connect(
            "text_changed".into(),
            self.base().callable("on_item_seconds_number"),
        );

        self.get_item_save_node()
            .connect("pressed".into(), self.base().callable("on_item_save"));

        self.get_item_edit_node()
            .connect("pressed".into(), self.base().callable("on_item_edit"));

        self.get_item_delete_node()
            .connect("pressed".into(), self.base().callable("on_item_delete"));

        self.get_item_clear_node()
            .connect("pressed".into(), self.base().callable("on_item_clear"));

        self.get_product_title_node().connect(
            "text_changed".into(),
            self.base().callable("on_product_title_changed"),
        );

        self.get_product_index_node().connect(
            "text_changed".into(),
            self.base().callable("on_product_index_changed"),
        );

        self.get_task_list_node().connect(
            "item_selected".into(),
            self.base().callable("on_task_list_selected"),
        );

        self.get_task_save_node()
            .connect("pressed".into(), self.base().callable("on_task_save"));

        self.get_task_delete_node()
            .connect("pressed".into(), self.base().callable("on_task_delete"));

        self.get_task_load_node()
            .connect("pressed".into(), self.base().callable("on_task_load"));

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

        let mut check_a_btn = self.get_check_a_node();
        check_a_btn.set_disabled(true);

        let mut check_b_btn = self.get_check_b_node();
        check_b_btn.set_disabled(false);

        self.task_list_update();
    }

    #[func]
    fn on_check_b(&mut self) {
        self.task.ab = AB::B;

        let mut check_a_btn = self.get_check_a_node();
        check_a_btn.set_disabled(false);

        let mut check_b_btn = self.get_check_b_node();
        check_b_btn.set_disabled(true);

        self.task_list_update();
    }

    #[func]
    fn on_task_name(&mut self, text: String) {
        self.task.title = text;
    }

    #[func]
    fn on_temp_number(&mut self, text: String) {
        let mut number = self.get_temp_node();

        let text = string_number_only(text);
        let dur = text
            .parse::<u16>()
            .unwrap_or_default()
            .clamp(u16::MIN, u16::MAX);

        self.task.temperature = dur;

        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_voltage_top_number(&mut self, text: String) {
        let mut number = self.get_voltage_top_node();

        let text = string_number_only(text);
        let dur = text
            .parse::<u32>()
            .unwrap_or_default()
            .clamp(u32::MIN, u32::MAX);

        self.task.voltage_verify.voltage_top = dur as f32;

        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_voltage_down_number(&mut self, text: String) {
        let mut number = self.get_voltage_down_node();

        let text = string_number_only(text);
        let dur = text
            .parse::<u32>()
            .unwrap_or_default()
            .clamp(u32::MIN, u32::MAX);

        self.task.voltage_verify.voltage_down = dur as f32;

        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_current_top_number(&mut self, text: String) {
        let mut number = self.get_current_top_node();

        let text = string_number_only(text);
        let dur = text
            .parse::<u32>()
            .unwrap_or_default()
            .clamp(u32::MIN, u32::MAX);

        self.task.voltage_verify.current_top = dur as f32;

        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_current_down_number(&mut self, text: String) {
        let mut number = self.get_current_down_node();

        let text = string_number_only(text);
        let dur = text
            .parse::<u32>()
            .unwrap_or_default()
            .clamp(u32::MIN, u32::MAX);

        self.task.voltage_verify.current_down = dur as f32;

        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_power_mode_selected(&mut self, index: u32) {
        if let Some(&mode) = PowerMode::VARIANTS.get(index as usize) {
            self.task.power.mode = mode
        };
    }

    #[func]
    fn on_power_voltage_number(&mut self, text: String) {
        let mut number = self.get_power_voltage_node();

        let text = string_number_only(text);
        let dur = text
            .parse::<u32>()
            .unwrap_or_default()
            .clamp(u32::MIN, u32::MAX);

        self.task.power.voltage = dur;

        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);
        self.item_power_voltage_update();
    }

    #[func]
    fn on_power_current_number(&mut self, text: String) {
        let mut number = self.get_power_current_node();

        let text = string_number_only(text);
        let dur = text
            .parse::<u32>()
            .unwrap_or_default()
            .clamp(u32::MIN, u32::MAX);

        self.task.power.current = dur;

        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_task_loop_number(&mut self, text: String) {
        let mut number = self.get_task_loop_node();

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
    fn on_task_item_selected(&mut self, index: u32) {
        // godot_print!("item {index}");
        if index == 0 {
            return;
        }
        // 去头
        let index = (index - 1) as usize;
        let item = match self.task.items.get(index) {
            Some(&item) => item,
            None => {
                return;
            }
        };

        let mut power_node = self.get_item_power_voltage_node();
        let power_index = match item.power_on {
            true => 1,
            false => 0,
        };
        power_node.select(power_index);

        let (hours, minutes, seconds) = hms_from_duration(item.dur);

        self.get_item_hours_node()
            .set_text(hours.to_string().into());
        self.get_item_minutes_node()
            .set_text(minutes.to_string().into());
        self.get_item_seconds_node()
            .set_text(seconds.to_string().into());
    }

    #[func]
    fn on_item_power_voltage_selected(&mut self, index: u32) {
        // godot_print!("power {index}");

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
        let mut number = self.get_item_hours_node();

        let text = string_number_only(text);
        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);

        // 更新时间
        self.item_total_time();
    }

    #[func]
    fn on_item_minutes_number(&mut self, text: String) {
        let mut number = self.get_item_minutes_node();

        let text = string_number_only(text);
        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);

        // 更新时间
        self.item_total_time();
    }

    #[func]
    fn on_item_seconds_number(&mut self, text: String) {
        let mut number = self.get_item_seconds_node();

        let text = string_number_only(text);
        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);

        // 更新时间
        self.item_total_time();
    }

    #[func]
    fn on_item_save(&mut self) {
        self.task.items.push(self.item);
        self.task_items_update();
    }

    #[func]
    fn on_item_edit(&mut self) {
        let mut item_node = self.get_task_items_node();
        let index = match item_node.get_selected_items().get(0) {
            Some(index) => index - 1, // 去头
            None => {
                return;
            }
        };

        self.task.items[index as usize] = self.item;
        self.task_items_update();
    }

    #[func]
    fn on_item_delete(&mut self) {
        let mut item_node = self.get_task_items_node();
        let index = match item_node.get_selected_items().get(0) {
            Some(index) => index - 1, // 去头
            None => {
                return;
            }
        };

        self.task.items.remove(index as usize);
        self.task_items_update();
        self.task_total_time();
    }

    #[func]
    fn on_item_clear(&mut self) {
        self.task.items = Vec::new();
        self.task_items_update();
    }

    #[func]
    fn on_product_title_changed(&mut self, text: String) {
        self.task.product.title = text.to_string();
    }

    #[func]
    fn on_product_index_changed(&mut self, text: String) {
        self.task.product.index = text.to_string();
    }

    #[func]
    fn on_task_save(&mut self) {
        log::debug!("task save: {}", self.task.title);
        // 保存
        if self.task.title.is_empty() {
            return;
        }
        log::debug!("task: {:?}", self.task);

        {
            let db = get_db().lock().unwrap();
            match TableTask::set(&db, &self.task) {
                Ok(list) => {}
                Err(e) => {
                    log::error!("task 保存失败：{e}");
                }
            };
        }

        self.alert(
            "保存确认".to_owned(),
            "确认".to_owned(),
            format!("已保存为{}", self.task.title),
        );

        self.task_list_update();
    }

    #[func]
    fn on_task_delete(&mut self) {
        let mut task_list_node = self.get_task_list_node();

        let s = task_list_node.get_selected_items();
        // godot_print!("{:?}", s);

        let index = match task_list_node.get_selected_items().get(0) {
            Some(index) => index,
            None => {
                return;
            }
        };

        if let Some(task) = self.list.get(index as usize) {
            let db = get_db().lock().unwrap();
            match TableTask::delete(&db, task.title.clone(), &task.ab) {
                Ok(_) => {}
                Err(e) => {
                    log::error!("task 删除失败：{}", e);
                }
            };
        };

        self.task_list_update();
    }

    #[func]
    fn on_task_load(&mut self) {
        let mut my_global = MyGlobal::singleton();

        // TODO: 检查产品，检查时长

        let mut product_title = self.get_product_title_node();
        let mut product_index = self.get_product_index_node();

        let p_title = product_title.get_text();

        if p_title.is_empty() {
            self.alert(
                "产品名称不可为空！".to_owned(),
                "确认".to_owned(),
                "产品名称不可为空！".to_owned(),
            );
            product_title.grab_focus();
            return;
        }

        let p_index = product_index.get_text();
        if p_index.is_empty() {
            self.alert(
                "产品序列不可为空！".to_owned(),
                "确认".to_owned(),
                "产品序列不可为空！".to_owned(),
            );
            product_index.grab_focus();
            return;
        }

        self.task.product.title = p_title.to_string();
        self.task.product.index = p_index.to_string();

        self.alert(
            format!("加载程序{}", self.task.title),
            "确认".to_owned(),
            format!(
                "已加载程序{}到{}面板",
                self.task.title,
                self.task.ab.as_ref()
            ),
        );

        my_global
            .bind_mut()
            .set_task(self.task.clone(), self.task.ab.into());
    }

    #[func]
    fn on_task_list_selected(&mut self, index: u32) {
        // godot_print!("list {index}");

        self.task = match self.list.get(index as usize) {
            Some(task) => task.clone(),
            None => {
                return;
            }
        };

        self.get_task_name_node()
            .set_text(self.task.title.clone().into());

        self.get_voltage_top_node()
            .set_text(self.task.voltage_verify.voltage_top.to_string().into());
        self.get_voltage_down_node()
            .set_text(self.task.voltage_verify.voltage_down.to_string().into());
        self.get_current_top_node()
            .set_text(self.task.voltage_verify.current_top.to_string().into());
        self.get_current_down_node()
            .set_text(self.task.voltage_verify.current_down.to_string().into());

        self.get_power_type_node()
            .select(self.task.power.mode as i32);

        self.get_power_voltage_node()
            .set_text(self.task.power.voltage.to_string().into());
        self.get_power_current_node()
            .set_text(self.task.power.current.to_string().into());

        self.get_task_loop_node()
            .set_text(self.task.task_loop.to_string().into());

        self.task_items_update();
    }
}

impl ProgramsView {
    fn item_power_voltage_update(&mut self) {
        let mut item_power_voltage_node = self.get_item_power_voltage_node();

        item_power_voltage_node.clear();
        let voltage = self.task.power.voltage;
        item_power_voltage_node.add_item("OFF".into());
        item_power_voltage_node.add_item(format!("{voltage}V").into());
    }

    // 计算时间
    fn item_total_time(&mut self) {
        let item_hours_node = self.get_item_hours_node();
        let text = item_hours_node.get_text();
        let text = string_number_only(text.to_string());
        let hours = text
            .parse::<u32>()
            .unwrap_or_default()
            .clamp(u32::MIN, u32::MAX) as u64;

        let item_minutes_node = self.get_item_minutes_node();
        let text = item_minutes_node.get_text();
        let text = string_number_only(text.to_string());
        let minutes = text
            .parse::<u32>()
            .unwrap_or_default()
            .clamp(u32::MIN, u32::MAX) as u64;

        let item_seconds_node = self.get_item_seconds_node();
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
        self.task.count_time = dur;

        let text = hms_from_duration_string(dur);
        let mut count_time_node = self.get_count_time_node();
        count_time_node.set_text(text.into());
    }

    /// 程序列表更新
    fn task_list_update(&mut self) {
        {
            let db = get_db().lock().unwrap();
            self.list = match TableTask::list(&db, &self.task.ab) {
                Ok(list) => list,
                Err(e) => {
                    log::error!("{}", e);
                    Vec::new()
                }
            };
        }

        // godot_print!("{:?}, {:?}", self.task.ab, self.list.len());
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

        self.task_total_time();

        self.base_mut()
            .emit_signal("update_task_item_list".into(), &[]);
    }

    fn alert(&mut self, title: String, btn: String, info: String) {
        let mut alert = self.get_alert_node();
        let mut alert_info = self.get_alert_info_node();
        alert.set_title(title.into());
        alert.set_ok_button_text(btn.into());
        alert_info.set_text(info.into());
        alert.set_visible(true);
    }

    // get node
    define_get_nodes![
        (get_check_a_node, UniqueName::CheckA, Button),
        (get_check_b_node, UniqueName::CheckB, Button),
        (get_task_name_node, UniqueName::TaskName, LineEdit),
        (get_temp_node, UniqueName::Temp, LineEdit),
        (get_voltage_top_node, UniqueName::VoltageTop, LineEdit),
        (get_voltage_down_node, UniqueName::VoltageDown, LineEdit),
        (get_current_top_node, UniqueName::CurrentTop, LineEdit),
        (get_current_down_node, UniqueName::CurrentDown, LineEdit),
        (get_power_type_node, UniqueName::PowerType, OptionButton),
        (get_power_voltage_node, UniqueName::PowerVoltage, LineEdit),
        (get_power_current_node, UniqueName::PowerCurrent, LineEdit),
        (get_count_time_node, UniqueName::CountTime, Label),
        (get_task_loop_node, UniqueName::TaskLoop, LineEdit),
        (get_task_items_node, UniqueName::TaskItems, ItemList),
        (
            get_item_power_voltage_node,
            UniqueName::ItemPowerVoltage,
            OptionButton
        ),
        (get_item_hours_node, UniqueName::ItemHours, LineEdit),
        (get_item_minutes_node, UniqueName::ItemMinutes, LineEdit),
        (get_item_seconds_node, UniqueName::ItemSeconds, LineEdit),
        (get_item_save_node, UniqueName::ItemSave, Button),
        (get_item_edit_node, UniqueName::ItemEdit, Button),
        (get_item_delete_node, UniqueName::ItemDelete, Button),
        (get_item_clear_node, UniqueName::ItemClear, Button),
        (get_product_title_node, UniqueName::ProductTitle, LineEdit),
        (get_product_index_node, UniqueName::ProductIndex, LineEdit),
        (get_task_list_node, UniqueName::TaskList, ItemList),
        (get_task_save_node, UniqueName::TaskSave, Button),
        (get_task_delete_node, UniqueName::TaskDelete, Button),
        (get_task_load_node, UniqueName::TaskLoad, Button),
        (get_alert_node, UniqueName::Alert, AcceptDialog),
        (get_alert_info_node, UniqueName::AlertInfo, Label),
    ];
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

    ProductTitle,
    ProductIndex,

    TaskList,
    TaskSave,
    TaskDelete,
    TaskLoad,

    Alert,
    AlertInfo,
}
