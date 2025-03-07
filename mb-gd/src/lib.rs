#![allow(dead_code)]
#![allow(unused_variables)]

use godot::{classes::Engine, prelude::*};
use scenes::my_global::MyGlobal;

mod chart;
mod colors;
mod data;
mod error;
mod macros;
mod mb_sync;
mod scenes;
mod utils;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            // 注册单例
            Engine::singleton()
                .register_singleton(&MyGlobal::string_name(), &MyGlobal::new_alloc());
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            let mut engine = Engine::singleton();
            let my_global_name = MyGlobal::string_name();

            // 获取指针
            let singleton = engine
                .get_singleton(&my_global_name)
                .expect("cannot retrieve the singleton");

            // 手动注销
            engine.unregister_singleton(&my_global_name);
            singleton.free();
        }
    }
}
