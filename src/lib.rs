mod scripting;
mod units;
mod console;

use std::env;
use bevy::{input::system::exit_on_esc_system, prelude::*};
use scripting::{
    Configuration,
    create_custom_cljrs_env,
};

pub fn build_app() -> App {
    let mut app = App::new();
    app // normal bevy stuff
       .add_system(exit_on_esc_system)
       .insert_resource(WindowDescriptor {
           title: "bevy + cljrs".into(),
           width: 1024.0,
           height: 768.0,
           ..Default::default()
       })
       .add_plugins(DefaultPlugins)
       ;

    app // something to visualize
       .add_plugin(units::UnitPlugin)
       ;

    scripting::add_scripting(
        &mut app,
        create_custom_cljrs_env(),
        Configuration {
            pre_window_repl: env::args().nth(1) == Some("--with-startup-repl".to_owned()),
        },
    );

    // todo: bind clj *out* to console
    app // in-app console a la Quake/Doom/etc
        .insert_non_send_resource(console::state_ref(console::State {
           scrollback: vec![
               "; welcome to the Clojure console of\tbevy + cljrs".into(),
               "; try the following:".into(),
               "    (bevy-cljrs/hi :foo 'bar {:k,\"v\"})".into(),
               "    (console/clear-scrollback)".into(),
            ],
           ..Default::default()
       }))
       .insert_resource(console::Configuration {
           top_pos: 0.0,
           left_pos: 0.0,
           width: 1024.0,
           height: (768.0 * 0.25),
           title: None,
           is_collapsable: false,
           submit_input_key: console::egui::Key::Enter,
           toggle_open_key: console::Key::KeyCode(KeyCode::Grave),
       })
       .add_plugin(console::ConsolePlugin)
       ;

    app
}
