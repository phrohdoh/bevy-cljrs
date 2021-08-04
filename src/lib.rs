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
    app
       .insert_resource(WindowDescriptor {
           title: "bevy + cljrs".into(),
           width: 200.0,
           height: 200.0,
           ..Default::default()
       })
       .add_plugins(DefaultPlugins)
       .add_plugin(units::UnitPlugin)
       .add_scripting(create_custom_cljrs_env(), ScriptingConfig {
           startup_repl: env::args().nth(1) == Some("--with-startup-repl".to_owned()),
       });
    app
}
