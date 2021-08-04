mod scripting;
mod units;

use std::env;
use bevy::prelude::*;
use scripting::{
    ScriptableApp as _,
    ScriptingConfig,
    create_custom_cljrs_env,
};

pub fn build_app() -> AppBuilder {
    let mut app = App::build();
    app.add_plugins(DefaultPlugins)
       .add_plugin(units::UnitPlugin)
       .add_scripting(create_custom_cljrs_env(), ScriptingConfig {
           startup_repl: env::args().nth(1) == Some("--with-startup-repl".to_owned()),
       });
    app
}
