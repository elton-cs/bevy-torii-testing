use bevy::prelude::*;
use bevy_frontend::plugins::torii::ToriiPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(ToriiPlugin);
    app.run();
}
