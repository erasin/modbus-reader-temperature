use godot::{
    engine::{
        control::LayoutPreset, Button, IPanelContainer, LineEdit, OptionButton, PanelContainer,
    },
    prelude::*,
};
use mb::protocol::get_ports;
use strum::AsRefStr;

use crate::scenes::my_global::{get_global_config, set_global_config};
use mb_data::{
    config::{Baudrate, Config},
    db::{config::TableGlobal, get_db},
};

#[derive(GodotClass)]
#[class(base=PanelContainer)]
pub struct SettingView {
    config: Config,
    base: Base<PanelContainer>,
}

#[godot_api]
impl IPanelContainer for SettingView {
    fn init(base: Base<PanelContainer>) -> Self {
        godot_print!("setting init");
        Self {
            config: Config::default(),
            base,
        }
    }

    fn ready(&mut self) {
        self.init_data();
    }
}

#[godot_api]
impl SettingView {
    #[signal]
    fn update_data();

    #[func]
    fn on_port_item_selected(&mut self, index: i32) {
        let ports = get_ports();

        let sel = match ports.get(index as usize) {
            Some(b) => b.clone(),
            None => return,
        };

        self.config.voltage.serial_port.port = sel;

        return;
    }

    #[func]
    fn on_baudrate_item_selected(&mut self, index: i32) {
        let sel = match Baudrate::ALL.get(index as usize) {
            Some(&b) => b,
            None => return,
        };

        self.config.voltage.serial_port.baudrate = sel;

        return;
    }

    #[func]
    fn on_number_a_start(&mut self, text: String) {
        let mut number = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::ApanelStartNum.to_string());

        let text = number_only(text);
        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_number_a_end(&mut self, text: String) {
        let mut number = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::ApanelEndNum.to_string());

        let text = number_only(text);
        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_number_b_start(&mut self, text: String) {
        let mut number = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::BpanelStartNum.to_string());

        let text = number_only(text);
        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_number_b_end(&mut self, text: String) {
        let mut number = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::BpanelEndNum.to_string());

        let text = number_only(text);
        let len = text.len();
        number.set_text(text.into());
        number.set_caret_column(len as i32);
    }

    #[func]
    fn on_submit(&mut self) {
        // TODO 整合数据

        // data
        {
            let db = get_db().lock().unwrap();

            let _ = TableGlobal::set_config(&db, &self.config);

            match TableGlobal::get_config(&db) {
                Ok(conf) => {
                    godot_print!("--- config: {:?}", conf);
                    set_global_config(conf);
                }
                Err(e) => godot_print!("{e}"),
            };
        }
    }
}

impl SettingView {
    pub fn init_data(&mut self) {
        self.config = get_global_config();

        let mut number_a_start = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::ApanelStartNum.to_string());

        number_a_start.connect(
            "text_changed".into(),
            self.base().callable("on_number_a_start"),
        );

        let mut number_a_end = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::ApanelEndNum.to_string());

        number_a_end.connect(
            "text_changed".into(),
            self.base().callable("on_number_a_end"),
        );

        let mut number_b_start = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::BpanelStartNum.to_string());

        number_b_start.connect(
            "text_changed".into(),
            self.base().callable("on_number_b_start"),
        );

        let mut number_b_end = self
            .base()
            .get_node_as::<LineEdit>(UniqueName::BpanelEndNum.to_string());

        number_b_end.connect(
            "text_changed".into(),
            self.base().callable("on_number_b_end"),
        );

        let mut voltage_port_btn = self
            .base()
            .get_node_as::<OptionButton>(UniqueName::VoltagePort.to_string());

        // let mut port_btn = self
        //     .base()
        //     .get_node_as::<OptionButton>(UniqueName::VoltagePort.to_string());

        let ports = get_ports();

        for (index, port) in ports.iter().enumerate() {
            voltage_port_btn.add_item(port.to_godot());
            if port
                .bytes()
                .eq(self.config.voltage.serial_port.port.bytes())
            {
                voltage_port_btn.select(index as i32);
            }
        }

        voltage_port_btn.connect(
            "item_selected".into(),
            self.base().callable("on_port_item_selected"),
        );

        let mut baudrate_btn = self
            .base()
            .get_node_as::<OptionButton>(UniqueName::VoltageBaudrate.to_string());

        for (index, &item) in Baudrate::ALL.iter().enumerate() {
            baudrate_btn.add_item(item.to_string().to_godot());

            if item == self.config.voltage.serial_port.baudrate {
                baudrate_btn.select(index as i32);
            }
        }

        baudrate_btn.connect(
            "item_selected".into(),
            self.base().callable("on_baudrate_item_selected"),
        );

        let mut submit_btn = self
            .base()
            .get_node_as::<Button>(UniqueName::Submit.to_string());

        submit_btn.connect("pressed".into(), self.base().callable("on_submit"));
    }
}

fn number_only(text: String) -> String {
    text.chars()
        .filter_map(|c| if c.is_digit(10) { Some(c) } else { None })
        .collect()
}

#[derive(AsRefStr, Debug)]
enum UniqueName {
    VoltagePort,
    VoltageBaudrate,
    ApanelStartNum,
    ApanelEndNum,
    BpanelStartNum,
    BpanelEndNum,
    TempPort,
    TempBaudrate,
    RelayPort,
    RelayBudrate,
    Submit,
}

// godot 唯一
impl std::fmt::Display for UniqueName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "%{}", self.as_ref())
    }
}
