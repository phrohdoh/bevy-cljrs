use std::env;
use bevy::prelude::*;

pub mod scripting;
pub mod unit_manager;

use scripting::{
    ScriptableApp as _,
    ScriptingConfig,
    create_custom_cljrs_env,
};

pub fn build_app() -> AppBuilder {
    let mut app = App::build();
    app.add_plugins(DefaultPlugins)
       .add_plugin(unit_manager::UnitManagerPlugin)
       .add_scripting(create_custom_cljrs_env(), ScriptingConfig {
           startup_repl: env::args().nth(1) == Some("--with-startup-repl".to_owned()),
       });
    app
}
