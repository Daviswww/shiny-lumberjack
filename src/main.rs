mod camera;

use bevy::prelude::*;

use camera::CameraPlugin;

fn main() {
    App::new()
        // Bevy built-ins.
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(CameraPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}
