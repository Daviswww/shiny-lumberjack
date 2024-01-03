use bevy::prelude::*;

use crate::{
    asset_loader::ImageAssets,
    movement::{Acceleration, MovingObjectBundle, Velocity},
};

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player)
            .add_systems(Update, player_movement_controls);
    }
}

fn spawn_player(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            model: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    ..default()
                },
                texture: image_assets.player.clone(),
                ..default()
            },
        },
        Player,
    ));
}

const PLAYER_SPEED: f32 = 5.0;
const BASE_MOVEMENT: f32 = 100.0;

fn player_movement_controls(
    mut query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let movement: f32 = BASE_MOVEMENT * time.delta_seconds() * PLAYER_SPEED;
    let mut transform = query.single_mut();
    let mut movement_x: f32 = 0.0;
    let mut movement_y: f32 = 0.0;

    if keyboard_input.pressed(KeyCode::D) {
        movement_x = movement;
    } else if keyboard_input.pressed(KeyCode::A) {
        movement_x = -movement;
    }

    if keyboard_input.pressed(KeyCode::W) {
        movement_y = movement;
    } else if keyboard_input.pressed(KeyCode::S) {
        movement_y = -movement;
    }
    transform.translation.x += movement_x;
    transform.translation.y += movement_y;
}
