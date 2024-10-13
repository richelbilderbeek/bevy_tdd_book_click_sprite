use bevy::prelude::*;
use crate::app::*;
mod app;

fn main() {
    let mut app = create_app();
    let add_camera_fn = |mut commands: Commands| {
        commands.spawn(Camera2dBundle::default());
    };
    app.add_systems(Startup, add_camera_fn);
    app.run();
}
