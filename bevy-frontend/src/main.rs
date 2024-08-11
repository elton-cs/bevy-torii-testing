use bevy::prelude::*;
use bevy_frontend::plugins::{torii::ToriiPlugin, visualize::VisualizePlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, default_camera);
    app.add_plugins(ToriiPlugin);
    app.add_plugins(VisualizePlugin);
    app.run();
}

fn default_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 0.3;
    commands.spawn(camera_bundle);
}
