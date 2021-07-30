use bevy::prelude::*;
use bevy_cljrs as game;

fn main() {
    let mut app = App::build();
    app.add_plugins(DefaultPlugins)
       .add_plugin(game::UnitManagerPlugin)
       .add_plugin(game::ClojureScriptingPlugin);
    app.run();
}
