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

const SPACESHIP_SPEED: f32 = 5.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;

fn player_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = query.single_mut();
    let mut movement = 0.0;
    let mut rotation = 0.0;

    if keyboard_input.pressed(KeyCode::D) {
        rotation = -SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::A) {
        rotation = SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::S) {
        movement = -SPACESHIP_SPEED;
        transform.translation.y -= 100.0 * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::W) {
        movement = SPACESHIP_SPEED;
        transform.translation.y += 100.0 * time.delta_seconds();
    }

    transform.rotate_y(rotation);

    // Update the spaceship's velocity based on new direction.
    velocity.value = transform.translation * movement;
}
