use godot::{
    builtin::StringName,
    engine::{Engine, IObject, Object},
    log::godot_print,
    obj::{Base, Gd},
    register::{godot_api, GodotClass},
};

use mb_data::{
    config::Config,
    db::{config::TableGlobal, get_db},
    user::UserConfig,
    utils::{get_time_offset, time_now},
};

use crate::utils::init_logging;

/// 单例：用于全局数据存储
#[derive(GodotClass)]
#[class(base=Object)]
pub struct MyGlobal {
    config: Option<Config>,
    pub user_state: Option<UserConfig>,
    sub_window: u8,

    base: Base<Object>,
}

#[godot_api]
impl IObject for MyGlobal {
    fn init(base: Base<Object>) -> Self {
        godot_print!("my singleton init");

        let _ = get_time_offset();
        let _ = init_logging(3);

        Self {
            config: None,
            user_state: None,
            sub_window: 0b0000,
            base,
        }
    }
}

#[godot_api]
impl MyGlobal {
    #[func]
    pub fn foo(&mut self) {
        godot_print!("my singleton foo ..., {}", time_now().to_string());
    }
}

impl MyGlobal {
    /// 获得单例
    ///
    /// ```
    /// let mut my_global = MyGlobal::singleton();
    /// let mut my_global = my_global.bind_mut();
    /// let config = my_global.get_config();
    /// ```
    pub fn singleton() -> Gd<Self> {
        Engine::singleton()
            .get_singleton(MyGlobal::string_name())
            .unwrap()
            .cast::<MyGlobal>()
    }

    /// 单例路径
    pub fn string_name() -> StringName {
        StringName::from("MyGlobal")
    }

    pub fn get_config(&mut self) -> Config {
        // from db
        if self.config.is_none() {
            let db = get_db().lock().unwrap();
            let conf = match TableGlobal::get_config(&db) {
                Ok(c) => c,
                Err(_) => Config::default(),
            };
            self.config = Some(conf);
        }

        let conf = self.config.as_ref().unwrap();
        conf.clone()
    }

    pub fn set_config(&mut self, config: Config) {
        self.config = Some(config);
    }

    pub fn get_sub_window(&self) -> u8 {
        // godot_print!("state: {:b}", state);
        self.sub_window
    }

    pub fn set_sub_window(&mut self, state: u8) {
        self.sub_window = state;
    }
}

pub fn get_global_config() -> Config {
    let mut my_global = MyGlobal::singleton();
    let mut my_global = my_global.bind_mut();
    let config = my_global.get_config();
    config
}

pub fn set_global_config(config: Config) {
    let mut my_global = MyGlobal::singleton();
    let mut my_global = my_global.bind_mut();
    my_global.set_config(config);
}
