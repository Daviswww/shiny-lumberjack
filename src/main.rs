use bevy::prelude::*;

mod animate;
mod asset_loader;
mod assets;
mod camera;
mod consts;
mod metadata;
mod movement;
mod player;

use animate::AnimatePlugin;
use asset_loader::AssetLoaderPlugin;
use assets::CustomAssetPlugin;
use camera::CameraPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
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
        .add_plugins(CustomAssetPlugin)
        .run();
}
