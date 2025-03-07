use godot::{
    classes::{
        AcceptDialog, Button, CheckBox, FileDialog, IPanelContainer, Label, LineEdit, OptionButton,
        PanelContainer, RichTextLabel,
        file_dialog::{Access, FileMode},
        window::WindowInitialPosition,
    },
    prelude::*,
};
use mb::protocol::get_ports;
use strum::AsRefStr;

use crate::{
    define_get_nodes,
    scenes::my_global::{get_global_config, set_global_config},
    utils::string_number_only,
};
use mb_data::{
    config::{Baudrate, Config, DefectiveRule},
    dirs::{data_dir, log_file},
};

use super::my_global::MyGlobal;

#[derive(GodotClass)]
#[class(init,base=PanelContainer)]
pub struct SettingView {
    config: Config,
    file_dialog: Gd<PackedScene>,
    base: Base<PanelContainer>,
}

#[godot_api]
impl IPanelContainer for SettingView {
    // fn init(base: Base<PanelContainer>) -> Self {
    //     godot_print!("setting init");
    //     Self {
    //         config: Config::default(),
    //         base,
    //     }
    // }

    fn ready(&mut self) {
        self.file_dialog = load("res://sys/file_dialog.tscn");
        self.config = get_global_config();
        self.init_data();
        self.defective_init();
        self.history_init();
        self.ab_init();
        self.debug();
    }
}

#[godot_api]
impl SettingView {
    #[signal]
    fn update_data();

    #[func]
    fn on_pro_name(&mut self, text: String) {
        let mut pro_name = self.get_pro_name_node();
        let text = text.trim();
        text.clone_into(&mut self.config.pro_name);

        let len = text.len();
        pro_name.set_text(text);
        pro_name.set_caret_column(len as i32);
    }

    #[func]
    fn on_enable_a_panel(&mut self, toggle_on: bool) {
        self.config.enable_a_panel = toggle_on;
        self.ab_init();
    }

    #[func]
    fn on_enable_b_panel(&mut self, toggle_on: bool) {
        self.config.enable_b_panel = toggle_on;
        self.ab_init();
    }

    #[func]
    fn on_voltage_a_port_item_selected(&mut self, index: u32) {
        let ports = get_ports();

        let sel = match ports.get(index as usize) {
            Some(b) => b.clone(),
            None => return,
        };

        self.config.voltage_a.serial_port.port = sel;
    }

    #[func]
    fn on_voltage_a_baudrate_item_selected(&mut self, index: u32) {
        let sel = match Baudrate::ALL.get(index as usize) {
            Some(&b) => b,
            None => return,
        };

        self.config.voltage_a.serial_port.baudrate = sel;
    }

    #[func]
    fn on_voltage_b_port_item_selected(&mut self, index: u32) {
        let ports = get_ports();

        let sel = match ports.get(index as usize) {
            Some(b) => b.clone(),
            None => return,
        };

        self.config.voltage_b.serial_port.port = sel;
    }

    #[func]
    fn on_voltage_b_baudrate_item_selected(&mut self, index: u32) {
        let sel = match Baudrate::ALL.get(index as usize) {
            Some(&b) => b,
            None => return,
        };

        self.config.voltage_b.serial_port.baudrate = sel;
    }

    #[func]
    fn on_temp_port_item_selected(&mut self, index: u32) {
        let ports = get_ports();

        let sel = match ports.get(index as usize) {
            Some(b) => b.clone(),
            None => return,
        };

        self.config.temperature.serial_port.port = sel;
    }

    #[func]
    fn on_temp_baudrate_item_selected(&mut self, index: u32) {
        let sel = match Baudrate::ALL.get(index as usize) {
            Some(&b) => b,
            None => return,
        };

        self.config.temperature.serial_port.baudrate = sel;
    }

    #[func]
    fn on_relay_port_item_selected(&mut self, index: u32) {
        let ports = get_ports();

        let sel = match ports.get(index as usize) {
            Some(b) => b.clone(),
            None => return,
        };

        self.config.relay.serial_port.port = sel;
    }

    #[func]
    fn on_relay_baudrate_item_selected(&mut self, index: u32) {
        let sel = match Baudrate::ALL.get(index as usize) {
            Some(&b) => b,
            None => return,
        };

        self.config.relay.serial_port.baudrate = sel;
    }

    #[func]
    fn on_power_a_port_item_selected(&mut self, index: u32) {
        let ports = get_ports();

        let sel = match ports.get(index as usize) {
            Some(b) => b.clone(),
            None => return,
        };

        self.config.power_a.serial_port.port = sel;
    }

    #[func]
    fn on_power_a_baudrate_item_selected(&mut self, index: u32) {
        let sel = match Baudrate::ALL.get(index as usize) {
            Some(&b) => b,
            None => return,
        };

        self.config.power_a.serial_port.baudrate = sel;
    }

    #[func]
    fn on_power_b_port_item_selected(&mut self, index: u32) {
        let ports = get_ports();

        let sel = match ports.get(index as usize) {
            Some(b) => b.clone(),
            None => return,
        };

        self.config.power_b.serial_port.port = sel;
    }

    #[func]
    fn on_power_b_baudrate_item_selected(&mut self, index: u32) {
        let sel = match Baudrate::ALL.get(index as usize) {
            Some(&b) => b,
            None => return,
        };

        self.config.power_b.serial_port.baudrate = sel;
    }

    #[func]
    fn on_number_a_start(&mut self, text: String) {
        let mut number = self.get_voltage_a_start_num_node();

        let text = string_number_only(text);
        let dur = text
            .parse::<u8>()
            .unwrap_or_default()
            .clamp(u8::MIN, u8::MAX);
        self.config.voltage_a.slave_start = dur;
        let len = text.len();
        number.set_text(&text);
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_number_a_end(&mut self, text: String) {
        let mut number = self.get_voltage_a_end_num_node();

        let text = string_number_only(text);
        let dur = text
            .parse::<u8>()
            .unwrap_or_default()
            .clamp(u8::MIN, u8::MAX);
        self.config.voltage_a.slave_end = dur;
        let len = text.len();
        number.set_text(&text);
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_number_b_start(&mut self, text: String) {
        let mut number = self.get_voltage_b_start_num_node();

        let text = string_number_only(text);
        let dur = text
            .parse::<u8>()
            .unwrap_or_default()
            .clamp(u8::MIN, u8::MAX);
        self.config.voltage_b.slave_start = dur;
        let len = text.len();
        number.set_text(&text);
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_number_b_end(&mut self, text: String) {
        let mut number = self.get_voltage_b_end_num_node();

        let text = string_number_only(text);
        let dur = text
            .parse::<u8>()
            .unwrap_or_default()
            .clamp(u8::MIN, u8::MAX);
        self.config.voltage_b.slave_end = dur;
        let len = text.len();
        number.set_text(&text);
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_temp_slave(&mut self, text: String) {
        let mut number = self.get_temp_slave_node();

        let text = string_number_only(text);
        let dur = text
            .parse::<u8>()
            .unwrap_or_default()
            .clamp(u8::MIN, u8::MAX);
        self.config.temperature.slave = dur;
        let len = text.len();
        number.set_text(&text);
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_relay_slave(&mut self, text: String) {
        let mut number = self.get_relay_slave_node();

        let text = string_number_only(text);
        let dur = text
            .parse::<u8>()
            .unwrap_or_default()
            .clamp(u8::MIN, u8::MAX);
        self.config.relay.slave = dur;
        let len = text.len();
        number.set_text(&text);
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_power_a_slave(&mut self, text: String) {
        let mut number = self.get_power_a_slave_node();

        let text = string_number_only(text);
        let dur = text
            .parse::<u8>()
            .unwrap_or_default()
            .clamp(u8::MIN, u8::MAX);
        self.config.power_a.slave = dur;
        let len = text.len();
        number.set_text(&text);
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_power_b_slave(&mut self, text: String) {
        let mut number = self.get_power_b_slave_node();

        let text = string_number_only(text);
        let dur = text
            .parse::<u8>()
            .unwrap_or_default()
            .clamp(u8::MIN, u8::MAX);
        self.config.power_b.slave = dur;
        let len = text.len();
        number.set_text(&text);
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_defective_rule_item_selected(&mut self, index: i32) {
        let sel = match DefectiveRule::ALL.get(index as usize) {
            Some(&b) => b,
            None => return,
        };

        self.config.defective.rule = sel;

        let mut dur_input = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::DefectiveDur.as_ref());

        dur_input.set_editable(self.config.defective.rule == DefectiveRule::InTime);
    }

    #[func]
    fn on_defective_dur(&mut self, text: String) {
        let mut number = self.get_defective_dur_node();

        let text = string_number_only(text);
        let dur = text.parse::<u64>().unwrap_or_default();
        self.config.defective.dur = dur;
        let len = text.len();
        number.set_text(&text);
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_history_log_dur(&mut self, text: String) {
        let mut number = self.get_history_log_dur_node();

        let text = string_number_only(text);
        let dur = text.parse::<u32>().unwrap_or_default();
        self.config.history.log_dur = dur;
        let len = text.len();
        number.set_text(&text);
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_history_lazy_dur(&mut self, text: String) {
        let mut number = self.get_history_lazy_dur_node();

        let text = string_number_only(text);
        let dur = text.parse::<u32>().unwrap_or_default();
        self.config.history.defective_lazy_dur = dur;
        let len = text.len();
        number.set_text(&text);
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_open_history_export_dir(&mut self) {
        let mut root = self.base().get_tree().unwrap().get_root().unwrap();
        let mut dialog = self.file_dialog.instantiate_as::<FileDialog>();

        dialog.set_title("选择文件夹");
        dialog.set_text("选择文件夹");
        dialog.set_cancel_button_text("取消");
        dialog.set_ok_button_text("打开");
        dialog.set_file_mode(FileMode::OPEN_DIR);
        dialog.set_access(Access::FILESYSTEM);
        dialog.set_use_native_dialog(true);
        dialog.set_current_dir(&self.config.history.export_dir);
        dialog.set_size(Vector2i::new(800, 500));
        dialog.set_initial_position(WindowInitialPosition::CENTER_PRIMARY_SCREEN);

        dialog.connect(
            "dir_selected",
            &self.base().callable("on_open_history_export_dir_selected"),
        );

        dialog.set_visible(true);
        root.add_child(&dialog);
    }

    #[func]
    fn on_open_history_export_dir_selected(&mut self, file_name: GString) {
        // godot_print!("--dir: {}", file_name);
        // label
        let mut label = self.get_history_export_dir_node();
        label.set_text(&file_name);

        self.config.history.export_dir = file_name.into();
    }

    #[func]
    fn on_submit(&mut self) {
        self.alert(
            "保存确认".to_owned(),
            "确认".to_owned(),
            "已保存配置".to_owned(),
        );
        set_global_config(self.config.clone());
    }
}

impl SettingView {
    pub fn init_data(&mut self) {
        let mut pro_name = self.get_pro_name_node();
        pro_name.connect("text_changed", &self.base().callable("on_pro_name"));
        pro_name.set_text(&self.config.pro_name.clone());
        pro_name.grab_focus();

        // -- a b --
        let mut enable_a_panel = self.get_enable_apanel_node();
        enable_a_panel.set_pressed(self.config.enable_a_panel);
        enable_a_panel.connect("toggled", &self.base().callable("on_enable_a_panel"));

        let mut enable_b_panel = self.get_enable_bpanel_node();
        enable_b_panel.set_pressed(self.config.enable_b_panel);
        enable_b_panel.connect("toggled", &self.base().callable("on_enable_b_panel"));

        // -- port ---

        let mut voltage_a_port_btn = self.get_voltage_a_port_node();
        let mut voltage_b_port_btn = self.get_voltage_b_port_node();
        let mut temp_port_btn = self.get_temp_port_node();
        let mut relay_port_btn = self.get_relay_port_node();
        let mut power_a_port_btn = self.get_power_a_port_node();
        let mut power_b_port_btn = self.get_power_b_port_node();

        let ports = get_ports();

        for (index, port) in ports.iter().enumerate() {
            voltage_a_port_btn.add_item(port);
            voltage_b_port_btn.add_item(port);
            temp_port_btn.add_item(port);
            relay_port_btn.add_item(port);
            power_a_port_btn.add_item(port);
            power_b_port_btn.add_item(port);

            let index = index as i32;

            if port
                .bytes()
                .eq(self.config.voltage_a.serial_port.port.bytes())
            {
                voltage_a_port_btn.select(index);
            }

            if port
                .bytes()
                .eq(self.config.voltage_b.serial_port.port.bytes())
            {
                voltage_b_port_btn.select(index);
            }

            if port
                .bytes()
                .eq(self.config.temperature.serial_port.port.bytes())
            {
                temp_port_btn.select(index);
            }

            if port.bytes().eq(self.config.relay.serial_port.port.bytes()) {
                relay_port_btn.select(index);
            }

            if port
                .bytes()
                .eq(self.config.power_a.serial_port.port.bytes())
            {
                power_a_port_btn.select(index);
            }

            if port
                .bytes()
                .eq(self.config.power_b.serial_port.port.bytes())
            {
                power_b_port_btn.select(index);
            }
        }

        voltage_a_port_btn.connect(
            "item_selected",
            &self.base().callable("on_voltage_a_port_item_selected"),
        );
        voltage_b_port_btn.connect(
            "item_selected",
            &self.base().callable("on_voltage_b_port_item_selected"),
        );
        temp_port_btn.connect(
            "item_selected",
            &self.base().callable("on_temp_port_item_selected"),
        );
        relay_port_btn.connect(
            "item_selected",
            &self.base().callable("on_relay_port_item_selected"),
        );
        power_a_port_btn.connect(
            "item_selected",
            &self.base().callable("on_power_a_port_item_selected"),
        );
        power_b_port_btn.connect(
            "item_selected",
            &self.base().callable("on_power_b_port_item_selected"),
        );

        // --- baudrate ---

        let mut voltage_a_baudrate_btn = self.get_voltage_a_baudrate_node();
        let mut voltage_b_baudrate_btn = self.get_voltage_b_baudrate_node();
        let mut temp_baudrate_btn = self.get_temp_baudrate_node();
        let mut relay_baudrate_btn = self.get_relay_baudrate_node();
        let mut power_a_baudrate_btn = self.get_power_a_baudrate_node();
        let mut power_b_baudrate_btn = self.get_power_b_baudrate_node();

        for (index, &item) in Baudrate::ALL.iter().enumerate() {
            voltage_a_baudrate_btn.add_item(&item.to_string());
            voltage_b_baudrate_btn.add_item(&item.to_string());
            temp_baudrate_btn.add_item(&item.to_string());
            relay_baudrate_btn.add_item(&item.to_string());
            power_a_baudrate_btn.add_item(&item.to_string());
            power_b_baudrate_btn.add_item(&item.to_string());

            let index = index as i32;

            if item == self.config.voltage_a.serial_port.baudrate {
                voltage_a_baudrate_btn.select(index);
            }
            if item == self.config.voltage_b.serial_port.baudrate {
                voltage_b_baudrate_btn.select(index);
            }
            if item == self.config.temperature.serial_port.baudrate {
                temp_baudrate_btn.select(index);
            }
            if item == self.config.relay.serial_port.baudrate {
                relay_baudrate_btn.select(index);
            }
            if item == self.config.power_a.serial_port.baudrate {
                power_a_baudrate_btn.select(index);
            }
            if item == self.config.power_b.serial_port.baudrate {
                power_b_baudrate_btn.select(index);
            }
        }

        voltage_a_baudrate_btn.connect(
            "item_selected",
            &self.base().callable("on_voltage_a_baudrate_item_selected"),
        );
        voltage_b_baudrate_btn.connect(
            "item_selected",
            &self.base().callable("on_voltage_b_baudrate_item_selected"),
        );
        temp_baudrate_btn.connect(
            "item_selected",
            &self.base().callable("on_temp_baudrate_item_selected"),
        );
        relay_baudrate_btn.connect(
            "item_selected",
            &self.base().callable("on_relay_baudrate_item_selected"),
        );
        power_a_baudrate_btn.connect(
            "item_selected",
            &self.base().callable("on_power_a_baudrate_item_selected"),
        );
        power_b_baudrate_btn.connect(
            "item_selected",
            &self.base().callable("on_power_b_baudrate_item_selected"),
        );

        // --- slave ---

        let mut number_a_start = self.get_voltage_a_start_num_node();
        number_a_start.set_text(&self.config.voltage_a.slave_start.to_string());
        number_a_start.connect("text_changed", &self.base().callable("on_number_a_start"));

        let mut number_a_end = self.get_voltage_a_end_num_node();
        number_a_end.set_text(&self.config.voltage_a.slave_end.to_string());
        number_a_end.connect("text_changed", &self.base().callable("on_number_a_end"));

        let mut number_b_start = self.get_voltage_b_start_num_node();
        number_b_start.set_text(&self.config.voltage_b.slave_start.to_string());
        number_b_start.connect("text_changed", &self.base().callable("on_number_b_start"));

        let mut number_b_end = self.get_voltage_b_end_num_node();
        number_b_end.set_text(&self.config.voltage_b.slave_end.to_string());
        number_b_end.connect("text_changed", &self.base().callable("on_number_b_end"));

        let mut temp_slave = self.get_temp_slave_node();
        temp_slave.set_text(&self.config.temperature.slave.to_string());
        temp_slave.connect("text_changed", &self.base().callable("on_temp_slave"));

        let mut relay_slave = self.get_relay_slave_node();
        relay_slave.set_text(&self.config.relay.slave.to_string());
        relay_slave.connect("text_changed", &self.base().callable("on_relay_slave"));

        let mut power_a_slave = self.get_power_a_slave_node();
        power_a_slave.set_text(&self.config.power_a.slave.to_string());
        power_a_slave.connect("text_changed", &self.base().callable("on_power_a_slave"));

        let mut power_b_slave = self.get_power_b_slave_node();
        power_b_slave.set_text(&self.config.power_b.slave.to_string());
        power_b_slave.connect("text_changed", &self.base().callable("on_power_b_slave"));

        // --- submit ---

        let mut submit_btn = self.get_submit_node();
        submit_btn.connect("pressed", &self.base().callable("on_submit"));
    }

    fn defective_init(&mut self) {
        let mut select_box = self
            .base()
            .get_node_as::<OptionButton>(UniqueName::DefectiveRule.as_ref());

        for (index, &item) in DefectiveRule::ALL.iter().enumerate() {
            select_box.add_item(&item.title());

            if item == self.config.defective.rule {
                select_box.select(index as i32);
            }
        }

        select_box.connect(
            "item_selected",
            &self.base().callable("on_defective_rule_item_selected"),
        );

        let mut dur_input = self.get_defective_dur_node();

        dur_input.connect("text_changed", &self.base().callable("on_defective_dur"));

        dur_input.set_editable(self.config.defective.rule == DefectiveRule::InTime);
        dur_input.set_text(&self.config.defective.dur.to_string())
    }

    fn history_init(&mut self) {
        let mut history_log_dur = self.get_history_log_dur_node();
        history_log_dur.set_text(&self.config.history.log_dur.to_string());

        history_log_dur.connect("text_changed", &self.base().callable("on_history_log_dur"));

        let mut history_lazy_dur = self.get_history_lazy_dur_node();
        history_lazy_dur.set_text(&self.config.history.defective_lazy_dur.to_string());

        history_lazy_dur.connect("text_changed", &self.base().callable("on_history_lazy_dur"));

        let mut export_btn = self.get_history_export_dir_btn_node();
        export_btn.connect(
            "pressed",
            &self.base().callable("on_open_history_export_dir"),
        );

        let mut history_export_dir = self.get_history_export_dir_node();
        history_export_dir.set_text(&self.config.history.export_dir.clone());
    }

    fn ab_init(&mut self) {
        let mut panel_a = self.get_panel_a_node();
        let mut panel_b = self.get_panel_b_node();
        panel_a.set_visible(self.config.enable_a_panel);
        panel_b.set_visible(self.config.enable_b_panel);
    }

    fn alert(&mut self, title: String, btn: String, info: String) {
        let mut alert = self.get_alert_node();
        let mut alert_info = self.get_alert_info_node();
        alert.set_title(&title);
        alert.set_ok_button_text(&btn);
        alert_info.set_text(&info);
        alert.set_visible(true);
    }

    /// 测试工具
    fn debug(&mut self) {
        let g = MyGlobal::singleton();
        if let Some(user) = g.bind().get_login() {
            if !user.name.eq("root") {
                return;
            }
            self.get_debug_panel_node().set_visible(true);

            self.get_path_data_node()
                .set_text(&data_dir().to_string_lossy().to_string());
            self.get_path_log_node()
                .set_text(&log_file().to_string_lossy().to_string());
        };
    }

    define_get_nodes![
        (get_pro_name_node, UniqueName::ProName, LineEdit),
        (get_enable_apanel_node, UniqueName::EnableApanel, CheckBox),
        (get_enable_bpanel_node, UniqueName::EnableBpanel, CheckBox),
        (get_panel_a_node, UniqueName::PanelA, PanelContainer),
        (get_panel_b_node, UniqueName::PanelB, PanelContainer),
        (
            get_voltage_a_port_node,
            UniqueName::VoltageAPort,
            OptionButton
        ),
        (
            get_voltage_a_baudrate_node,
            UniqueName::VoltageABaudrate,
            OptionButton
        ),
        (
            get_voltage_a_start_num_node,
            UniqueName::VoltageAStartNum,
            LineEdit
        ),
        (
            get_voltage_a_end_num_node,
            UniqueName::VoltageAEndNum,
            LineEdit
        ),
        (
            get_voltage_b_port_node,
            UniqueName::VoltageBPort,
            OptionButton
        ),
        (
            get_voltage_b_baudrate_node,
            UniqueName::VoltageBBaudrate,
            OptionButton
        ),
        (
            get_voltage_b_start_num_node,
            UniqueName::VoltageBStartNum,
            LineEdit
        ),
        (
            get_voltage_b_end_num_node,
            UniqueName::VoltageBEndNum,
            LineEdit
        ),
        (get_temp_port_node, UniqueName::TempPort, OptionButton),
        (
            get_temp_baudrate_node,
            UniqueName::TempBaudrate,
            OptionButton
        ),
        (get_temp_slave_node, UniqueName::TempSlave, LineEdit),
        (get_relay_port_node, UniqueName::RelayPort, OptionButton),
        (
            get_relay_baudrate_node,
            UniqueName::RelayBaudrate,
            OptionButton
        ),
        (get_relay_slave_node, UniqueName::RelaySlave, LineEdit),
        (get_power_a_port_node, UniqueName::PowerAPort, OptionButton),
        (
            get_power_a_baudrate_node,
            UniqueName::PowerABaudrate,
            OptionButton
        ),
        (get_power_a_slave_node, UniqueName::PowerASlave, LineEdit),
        (get_power_b_port_node, UniqueName::PowerBPort, OptionButton),
        (
            get_power_b_baudrate_node,
            UniqueName::PowerBBaudrate,
            OptionButton
        ),
        (get_power_b_slave_node, UniqueName::PowerBSlave, LineEdit),
        (
            get_defective_rule_node,
            UniqueName::DefectiveRule,
            OptionButton
        ),
        (get_defective_dur_node, UniqueName::DefectiveDur, LineEdit),
        (
            get_history_log_dur_node,
            UniqueName::HistoryLogDur,
            LineEdit
        ),
        (
            get_history_lazy_dur_node,
            UniqueName::HistoryLazyDur,
            LineEdit
        ),
        (
            get_history_export_dir_btn_node,
            UniqueName::HistoryExportDirBtn,
            Button
        ),
        (
            get_history_export_dir_node,
            UniqueName::HistoryExportDir,
            RichTextLabel
        ),
        (get_submit_node, UniqueName::Submit, Button),
        (get_alert_node, UniqueName::Alert, AcceptDialog),
        (get_alert_info_node, UniqueName::AlertInfo, Label),
        (get_debug_panel_node, UniqueName::DebugPanel, PanelContainer),
        (get_path_data_node, UniqueName::PathData, RichTextLabel),
        (get_path_log_node, UniqueName::PathLog, RichTextLabel),
    ];
}

#[derive(AsRefStr, Debug)]
#[strum(prefix = "%")]
enum UniqueName {
    ProName,
    EnableApanel,
    EnableBpanel,

    PanelA,
    PanelB,

    VoltageAPort,
    VoltageABaudrate,
    VoltageAStartNum,
    VoltageAEndNum,

    VoltageBPort,
    VoltageBBaudrate,
    VoltageBStartNum,
    VoltageBEndNum,

    TempPort,
    TempBaudrate,
    TempSlave,

    RelayPort,
    RelayBaudrate,
    RelaySlave,

    PowerAPort,
    PowerABaudrate,
    PowerASlave,

    PowerBPort,
    PowerBBaudrate,
    PowerBSlave,

    DefectiveRule,
    DefectiveDur,

    HistoryLogDur,
    HistoryLazyDur,
    HistoryExportDirBtn,
    HistoryExportDir,

    Submit,

    Alert,
    AlertInfo,

    DebugPanel,
    PathData,
    PathLog,
}

impl std::fmt::Display for UniqueName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
