use bevy::prelude::*;

mod animate;
mod asset_loader;
mod camera;
mod movement;
mod player;

use animate::AnimatePlugin;
use asset_loader::AssetLoaderPlugin;
use camera::CameraPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(AnimatePlugin)
        .run();
}
