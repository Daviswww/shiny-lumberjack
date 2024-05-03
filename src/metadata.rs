use crate::animate::Clip;
use bevy::{
    math::{UVec2, Vec2},
    prelude::{Component, Handle, Image},
    sprite::TextureAtlas,
    utils::HashMap,
};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Component)]
pub struct PlayerMeta {
    pub name: String,
    #[serde(skip)]
    pub center_y: f32,
    #[serde(skip)]
    pub collision_offset: f32,
    pub hud: PlayerHudMeta,
    pub spritesheet: PlayerSpritesheetMeta,
    pub attachment: Option<PlayerSpritesheetMeta>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct PlayerSpritesheetMeta {
    pub image: Vec<String>,
    #[serde(skip)]
    pub atlas_handle: Vec<TextureAtlas>,
    pub tile_size: UVec2,
    pub columns: usize,
    pub rows: usize,
    pub animation_fps: f32,
    pub animations: HashMap<String, Clip>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct PlayerHudMeta {
    pub portrait: ImageMeta,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ImageMeta {
    pub image: String,
    pub image_size: Vec2,
    #[serde(skip)]
    pub image_handle: Handle<Image>,
}
