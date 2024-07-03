use godot::{
    engine::{
        file_dialog::{Access, FileMode},
        window::WindowInitialPosition,
        Button, CheckBox, FileDialog, IPanelContainer, Label, LineEdit, OptionButton,
        PanelContainer,
    },
    prelude::*,
};
use mb::protocol::get_ports;
use strum::AsRefStr;

use crate::scenes::my_global::{get_global_config, set_global_config};
use mb_data::config::{Baudrate, Config, DefectiveRule};

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
    }
}

#[godot_api]
impl SettingView {
    #[signal]
    fn update_data();

    #[func]
    fn on_pro_name(&mut self, text: String) {
        let mut pro_name = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::ProName.to_string());
        let text = text.trim();

        self.config.pro_name = text.to_owned();

        let len = text.len();
        pro_name.set_text(text.into());
        pro_name.set_caret_column(len as i32);
    }

    #[func]
    fn on_enable_a_panel(&mut self, value: bool) {
        self.config.enable_a_panel = value;
        self.ab_init();
    }

    #[func]
    fn on_enable_b_panel(&mut self, value: bool) {
        self.config.enable_b_panel = value;
        self.ab_init();
    }

    #[func]
    fn on_voltage_a_port_item_selected(&mut self, index: i32) {
        let ports = get_ports();

        let sel = match ports.get(index as usize) {
            Some(b) => b.clone(),
            None => return,
        };

        self.config.voltage_a.serial_port.port = sel;

        return;
    }

    #[func]
    fn on_voltage_a_baudrate_item_selected(&mut self, index: i32) {
        let sel = match Baudrate::ALL.get(index as usize) {
            Some(&b) => b,
            None => return,
        };

        self.config.voltage_a.serial_port.baudrate = sel;

        return;
    }

    #[func]
    fn on_voltage_b_port_item_selected(&mut self, index: i32) {
        let ports = get_ports();

        let sel = match ports.get(index as usize) {
            Some(b) => b.clone(),
            None => return,
        };

        self.config.voltage_b.serial_port.port = sel;

        return;
    }

    #[func]
    fn on_voltage_b_baudrate_item_selected(&mut self, index: i32) {
        let sel = match Baudrate::ALL.get(index as usize) {
            Some(&b) => b,
            None => return,
        };

        self.config.voltage_b.serial_port.baudrate = sel;

        return;
    }

    #[func]
    fn on_temp_port_item_selected(&mut self, index: i32) {
        let ports = get_ports();

        let sel = match ports.get(index as usize) {
            Some(b) => b.clone(),
            None => return,
        };

        self.config.temperature.serial_port.port = sel;

        return;
    }

    #[func]
    fn on_temp_baudrate_item_selected(&mut self, index: i32) {
        let sel = match Baudrate::ALL.get(index as usize) {
            Some(&b) => b,
            None => return,
        };

        self.config.temperature.serial_port.baudrate = sel;

        return;
    }

    #[func]
    fn on_relay_port_item_selected(&mut self, index: i32) {
        let ports = get_ports();

        let sel = match ports.get(index as usize) {
            Some(b) => b.clone(),
            None => return,
        };

        self.config.relay.serial_port.port = sel;

        return;
    }

    #[func]
    fn on_relay_baudrate_item_selected(&mut self, index: i32) {
        let sel = match Baudrate::ALL.get(index as usize) {
            Some(&b) => b,
            None => return,
        };

        self.config.relay.serial_port.baudrate = sel;

        return;
    }

    #[func]
    fn on_power_a_port_item_selected(&mut self, index: i32) {
        let ports = get_ports();

        let sel = match ports.get(index as usize) {
            Some(b) => b.clone(),
            None => return,
        };

        self.config.power_a.serial_port.port = sel;

        return;
    }

    #[func]
    fn on_power_a_baudrate_item_selected(&mut self, index: i32) {
        let sel = match Baudrate::ALL.get(index as usize) {
            Some(&b) => b,
            None => return,
        };

        self.config.power_a.serial_port.baudrate = sel;

        return;
    }

    #[func]
    fn on_power_b_port_item_selected(&mut self, index: i32) {
        let ports = get_ports();

        let sel = match ports.get(index as usize) {
            Some(b) => b.clone(),
            None => return,
        };

        self.config.power_b.serial_port.port = sel;

        return;
    }

    #[func]
    fn on_power_b_baudrate_item_selected(&mut self, index: i32) {
        let sel = match Baudrate::ALL.get(index as usize) {
            Some(&b) => b,
            None => return,
        };

        self.config.power_b.serial_port.baudrate = sel;

        return;
    }

    #[func]
    fn on_number_a_start(&mut self, text: String) {
        let mut number = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::VoltageAStartNum.to_string());

        let text = number_only(text);
        let dur = text
            .parse::<u8>()
            .unwrap_or_default()
            .clamp(u8::MIN + 1, u8::MAX);
        self.config.voltage_a.slave_start = dur;
        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_number_a_end(&mut self, text: String) {
        let mut number = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::VoltageAEndNum.to_string());

        let text = number_only(text);
        let dur = text
            .parse::<u8>()
            .unwrap_or_default()
            .clamp(u8::MIN + 1, u8::MAX);
        self.config.voltage_a.slave_end = dur;
        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_number_b_start(&mut self, text: String) {
        let mut number = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::VoltageBStartNum.to_string());

        let text = number_only(text);
        let dur = text
            .parse::<u8>()
            .unwrap_or_default()
            .clamp(u8::MIN + 1, u8::MAX);
        self.config.voltage_b.slave_start = dur;
        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_number_b_end(&mut self, text: String) {
        let mut number = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::VoltageBEndNum.to_string());

        let text = number_only(text);
        let dur = text
            .parse::<u8>()
            .unwrap_or_default()
            .clamp(u8::MIN + 1, u8::MAX);
        self.config.voltage_b.slave_end = dur;
        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_temp_slave(&mut self, text: String) {
        let mut number = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::TempSlave.to_string());

        let text = number_only(text);
        let dur = text
            .parse::<u8>()
            .unwrap_or_default()
            .clamp(u8::MIN + 1, u8::MAX);
        self.config.temperature.slave = dur;
        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_relay_slave(&mut self, text: String) {
        let mut number = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::RelaySlave.to_string());

        let text = number_only(text);
        let dur = text
            .parse::<u8>()
            .unwrap_or_default()
            .clamp(u8::MIN + 1, u8::MAX);
        self.config.relay.slave = dur;
        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_power_a_slave(&mut self, text: String) {
        let mut number = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::PowerASlave.to_string());

        let text = number_only(text);
        let dur = text
            .parse::<u8>()
            .unwrap_or_default()
            .clamp(u8::MIN + 1, u8::MAX);
        self.config.power_a.slave = dur;
        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_power_b_slave(&mut self, text: String) {
        let mut number = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::PowerBSlave.to_string());

        let text = number_only(text);
        let dur = text
            .parse::<u8>()
            .unwrap_or_default()
            .clamp(u8::MIN + 1, u8::MAX);
        self.config.power_b.slave = dur;
        let len = text.len();
        number.set_text(text.into());
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

        return;
    }

    #[func]
    fn on_defective_dur(&mut self, text: String) {
        let mut number = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::DefectiveDur.to_string());

        let text = number_only(text);
        let dur = text.parse::<u32>().unwrap_or_default();
        self.config.defective.dur = dur;
        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_history_log_dur(&mut self, text: String) {
        let mut number = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::HistoryLogDur.to_string());

        let text = number_only(text);
        let dur = text.parse::<u32>().unwrap_or_default();
        self.config.history.log_dur = dur;
        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_history_lazy_dur(&mut self, text: String) {
        let mut number = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::HistoryLazyDur.to_string());

        let text = number_only(text);
        let dur = text.parse::<u32>().unwrap_or_default();
        self.config.history.defective_lazy_dur = dur;
        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_open_history_export_dir(&mut self) {
        let mut root = self.base().get_tree().unwrap().get_root().unwrap();
        let mut dialog = self.file_dialog.instantiate_as::<FileDialog>();

        dialog.set_title("选择文件夹".into());
        dialog.set_text("选择文件夹".into());
        dialog.set_cancel_button_text("取消".into());
        dialog.set_ok_button_text("打开".into());
        dialog.set_file_mode(FileMode::OPEN_DIR);
        dialog.set_access(Access::FILESYSTEM);
        dialog.set_use_native_dialog(true);
        dialog.set_current_dir(self.config.history.export_dir.to_godot());
        dialog.set_size(Vector2i::new(800, 500));
        dialog.set_initial_position(WindowInitialPosition::CENTER_PRIMARY_SCREEN);

        dialog.connect(
            "dir_selected".into(),
            self.base().callable("on_open_history_export_dir_selected"),
        );

        dialog.set_visible(true);
        root.add_child(dialog.upcast());
    }

    #[func]
    fn on_open_history_export_dir_selected(&mut self, file_name: GString) {
        godot_print!("--dir: {}", file_name);
        // label
        let mut label = self
            .base()
            .get_node_as::<Label>(UniqueName::HistoryExportDir.as_ref());
        label.set_text(file_name.clone());

        self.config.history.export_dir = file_name.into();
    }

    #[func]
    fn on_submit(&mut self) {
        set_global_config(self.config.clone());
    }
}

impl SettingView {
    pub fn init_data(&mut self) {
        let mut pro_name = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::ProName.as_ref());
        pro_name.connect("text_changed".into(), self.base().callable("on_pro_name"));
        pro_name.set_text(self.config.pro_name.clone().into());

        // -- a b --
        let mut enable_a_panel = self
            .base()
            .get_node_as::<CheckBox>(UniqueName::EnableApanel.to_string());
        enable_a_panel.set_pressed(self.config.enable_a_panel);
        enable_a_panel.connect("toggled".into(), self.base().callable("on_enable_a_panel"));

        let mut enable_b_panel = self
            .base()
            .get_node_as::<CheckBox>(UniqueName::EnableBpanel.to_string());
        enable_b_panel.set_pressed(self.config.enable_b_panel);
        enable_b_panel.connect("toggled".into(), self.base().callable("on_enable_b_panel"));

        // -- port ---

        let mut voltage_a_port_btn = self
            .base()
            .get_node_as::<OptionButton>(UniqueName::VoltageAPort.to_string());

        let mut voltage_b_port_btn = self
            .base()
            .get_node_as::<OptionButton>(UniqueName::VoltageBPort.to_string());

        let mut temp_port_btn = self
            .base()
            .get_node_as::<OptionButton>(UniqueName::TempPort.as_ref());

        let mut relay_port_btn = self
            .base()
            .get_node_as::<OptionButton>(UniqueName::RelayPort.as_ref());

        let mut power_a_port_btn = self
            .base()
            .get_node_as::<OptionButton>(UniqueName::PowerAPort.as_ref());

        let mut power_b_port_btn = self
            .base()
            .get_node_as::<OptionButton>(UniqueName::PowerBPort.as_ref());

        let ports = get_ports();

        for (index, port) in ports.iter().enumerate() {
            voltage_a_port_btn.add_item(port.to_godot());
            voltage_b_port_btn.add_item(port.to_godot());
            temp_port_btn.add_item(port.to_godot());
            relay_port_btn.add_item(port.to_godot());
            power_a_port_btn.add_item(port.to_godot());
            power_b_port_btn.add_item(port.to_godot());

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
            "item_selected".into(),
            self.base().callable("on_voltage_a_port_item_selected"),
        );
        voltage_b_port_btn.connect(
            "item_selected".into(),
            self.base().callable("on_voltage_b_port_item_selected"),
        );
        temp_port_btn.connect(
            "item_selected".into(),
            self.base().callable("on_temp_port_item_selected"),
        );
        relay_port_btn.connect(
            "item_selected".into(),
            self.base().callable("on_relay_port_item_selected"),
        );
        power_a_port_btn.connect(
            "item_selected".into(),
            self.base().callable("on_power_a_port_item_selected"),
        );
        power_b_port_btn.connect(
            "item_selected".into(),
            self.base().callable("on_power_b_port_item_selected"),
        );

        // --- baudrate ---

        let mut voltage_a_baudrate_btn = self
            .base()
            .get_node_as::<OptionButton>(UniqueName::VoltageABaudrate.as_ref());

        let mut voltage_b_baudrate_btn = self
            .base()
            .get_node_as::<OptionButton>(UniqueName::VoltageBBaudrate.as_ref());

        let mut temp_baudrate_btn = self
            .base()
            .get_node_as::<OptionButton>(UniqueName::TempBaudrate.as_ref());

        let mut relay_baudrate_btn = self
            .base()
            .get_node_as::<OptionButton>(UniqueName::RelayBaudrate.as_ref());

        let mut power_a_baudrate_btn = self
            .base()
            .get_node_as::<OptionButton>(UniqueName::PowerABaudrate.as_ref());

        let mut power_b_baudrate_btn = self
            .base()
            .get_node_as::<OptionButton>(UniqueName::PowerBBaudrate.as_ref());

        for (index, &item) in Baudrate::ALL.iter().enumerate() {
            voltage_a_baudrate_btn.add_item(item.to_string().to_godot());
            voltage_b_baudrate_btn.add_item(item.to_string().to_godot());
            temp_baudrate_btn.add_item(item.to_string().to_godot());
            relay_baudrate_btn.add_item(item.to_string().to_godot());
            power_a_baudrate_btn.add_item(item.to_string().to_godot());
            power_b_baudrate_btn.add_item(item.to_string().to_godot());

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
            "item_selected".into(),
            self.base().callable("on_voltage_a_baudrate_item_selected"),
        );
        voltage_b_baudrate_btn.connect(
            "item_selected".into(),
            self.base().callable("on_voltage_b_baudrate_item_selected"),
        );
        temp_baudrate_btn.connect(
            "item_selected".into(),
            self.base().callable("on_temp_baudrate_item_selected"),
        );
        relay_baudrate_btn.connect(
            "item_selected".into(),
            self.base().callable("on_relay_baudrate_item_selected"),
        );
        power_a_baudrate_btn.connect(
            "item_selected".into(),
            self.base().callable("on_power_a_baudrate_item_selected"),
        );
        power_b_baudrate_btn.connect(
            "item_selected".into(),
            self.base().callable("on_power_b_baudrate_item_selected"),
        );

        // --- slave ---

        let mut number_a_start = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::VoltageAStartNum.to_string());
        number_a_start.set_text(self.config.voltage_a.slave_start.to_string().into());
        number_a_start.connect(
            "text_changed".into(),
            self.base().callable("on_number_a_start"),
        );

        let mut number_a_end = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::VoltageAEndNum.to_string());
        number_a_end.set_text(self.config.voltage_a.slave_end.to_string().into());
        number_a_end.connect(
            "text_changed".into(),
            self.base().callable("on_number_a_end"),
        );

        let mut number_b_start = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::VoltageBStartNum.to_string());
        number_b_start.set_text(self.config.voltage_b.slave_start.to_string().into());
        number_b_start.connect(
            "text_changed".into(),
            self.base().callable("on_number_b_start"),
        );

        let mut number_b_end = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::VoltageBEndNum.to_string());
        number_b_end.set_text(self.config.voltage_b.slave_end.to_string().into());
        number_b_end.connect(
            "text_changed".into(),
            self.base().callable("on_number_b_end"),
        );

        let mut temp_slave = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::TempSlave.to_string());
        temp_slave.set_text(self.config.temperature.slave.to_string().into());
        temp_slave.connect("text_changed".into(), self.base().callable("on_temp_slave"));

        let mut relay_slave = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::RelaySlave.to_string());
        relay_slave.set_text(self.config.relay.slave.to_string().into());
        relay_slave.connect(
            "text_changed".into(),
            self.base().callable("on_relay_slave"),
        );

        let mut power_a_slave = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::PowerASlave.to_string());
        power_a_slave.set_text(self.config.power_a.slave.to_string().into());
        power_a_slave.connect(
            "text_changed".into(),
            self.base().callable("on_power_a_slave"),
        );

        let mut power_b_slave = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::PowerBSlave.to_string());
        power_b_slave.set_text(self.config.power_b.slave.to_string().into());

        power_b_slave.connect(
            "text_changed".into(),
            self.base().callable("on_power_b_slave"),
        );

        // --- submit ---

        let mut submit_btn = self
            .base()
            .get_node_as::<Button>(UniqueName::Submit.to_string());

        submit_btn.connect("pressed".into(), self.base().callable("on_submit"));
    }

    fn defective_init(&mut self) {
        let mut select_box = self
            .base()
            .get_node_as::<OptionButton>(UniqueName::DefectiveRule.as_ref());

        for (index, &item) in DefectiveRule::ALL.iter().enumerate() {
            select_box.add_item(item.title().to_godot());

            if item == self.config.defective.rule {
                select_box.select(index as i32);
            }
        }

        select_box.connect(
            "item_selected".into(),
            self.base().callable("on_defective_rule_item_selected"),
        );

        let mut dur_input = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::DefectiveDur.as_ref());

        dur_input.connect(
            "text_changed".into(),
            self.base().callable("on_defective_dur"),
        );

        dur_input.set_editable(self.config.defective.rule == DefectiveRule::InTime);
        dur_input.set_text(self.config.defective.dur.to_string().into())
    }

    fn history_init(&mut self) {
        let mut history_log_dur = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::HistoryLogDur.to_string());
        history_log_dur.set_text(self.config.history.log_dur.to_string().into());

        history_log_dur.connect(
            "text_changed".into(),
            self.base().callable("on_history_log_dur"),
        );

        let mut history_lazy_dur = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::HistoryLazyDur.to_string());
        history_lazy_dur.set_text(self.config.history.defective_lazy_dur.to_string().into());

        history_lazy_dur.connect(
            "text_changed".into(),
            self.base().callable("on_history_lazy_dur"),
        );

        let mut export_btn = self
            .base()
            .get_node_as::<Button>(UniqueName::HistoryExportDirBtn.as_ref());
        export_btn.connect(
            "pressed".into(),
            self.base().callable("on_open_history_export_dir"),
        );

        let mut history_export_dir = self
            .base()
            .get_node_as::<Label>(UniqueName::HistoryExportDir.to_string());
        history_export_dir.set_text(self.config.history.export_dir.clone().into());
    }

    fn ab_init(&mut self) {
        let mut panel_a = self
            .base()
            .get_node_as::<PanelContainer>(UniqueName::PanelA.as_ref());
        let mut panel_b = self
            .base()
            .get_node_as::<PanelContainer>(UniqueName::PanelB.as_ref());

        panel_a.set_visible(self.config.enable_a_panel);
        panel_b.set_visible(self.config.enable_b_panel);
    }
}

fn number_only(text: String) -> String {
    text.chars()
        .filter_map(|c| if c.is_digit(10) { Some(c) } else { None })
        .collect()
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
}

impl std::fmt::Display for UniqueName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
