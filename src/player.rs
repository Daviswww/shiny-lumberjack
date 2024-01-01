use bevy::prelude::*;

use crate::asset_loader::ImageAssets;

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player);
    }
}

fn spawn_player(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        texture: image_assets.player.clone(),
        ..default()
    });
}
