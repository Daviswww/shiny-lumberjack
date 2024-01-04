use bevy::prelude::*;

use crate::{
    animate::{AnimationIndices, AnimationTimer},
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

fn spawn_player(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = image_assets.player_run.clone();
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 4, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices::new(0, 3);
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            model: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: animation_indices.first,
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    ..default()
                },
                texture_atlas: texture_atlas_handle,
                ..default()
            },
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
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
    let mut transform = query.single_mut();
    let movement: f32 = BASE_MOVEMENT * time.delta_seconds() * PLAYER_SPEED;
    let mut movement_x: f32 = 0.0;
    let mut movement_y: f32 = 0.0;

    if keyboard_input.pressed(KeyCode::D) {
        movement_x = movement;
        transform.rotation.y = 0.0;
    } else if keyboard_input.pressed(KeyCode::A) {
        movement_x = -movement;
        transform.rotation.y = 1.0;
    }

    if keyboard_input.pressed(KeyCode::W) {
        movement_y = movement;
    } else if keyboard_input.pressed(KeyCode::S) {
        movement_y = -movement;
    }

    transform.translation.x += movement_x;
    transform.translation.y += movement_y;
}
