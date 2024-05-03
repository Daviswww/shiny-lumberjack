use bevy::utils::thiserror;
use bevy::{
    asset::{io::Reader, ron, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    reflect::TypePath,
    utils::BoxedFuture,
};
use serde::Deserialize;
use thiserror::Error;

// use crate::{consts::FOOT_PADDING, metadata::*};

pub struct CustomAssetPlugin;

impl Plugin for CustomAssetPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TextureAtlas>()
            .init_asset::<CustomAsset>()
            .init_asset_loader::<CustomAssetLoader>()
            .add_systems(Startup, setup)
            .add_systems(Update, print_on_load);
    }
}

#[derive(Asset, TypePath, Debug, Deserialize)]
struct CustomAsset {
    #[allow(dead_code)]
    value: i32,
}

#[derive(Default)]
struct CustomAssetLoader;

/// Possible errors that can be produced by [`CustomAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum CustomAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) Error
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}

impl AssetLoader for CustomAssetLoader {
    type Asset = CustomAsset;
    type Settings = ();
    type Error = CustomAssetLoaderError;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let custom_asset = ron::de::from_bytes::<CustomAsset>(&bytes)?;
            Ok(custom_asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["lumberjack.player.yaml"]
    }
}

#[derive(Asset, TypePath, Debug)]
struct Blob {
    bytes: Vec<u8>,
}

#[derive(Resource, Default)]
struct State {
    handle: Handle<CustomAsset>,
    other_handle: Handle<CustomAsset>,
    blob: Handle<Blob>,
    printed: bool,
}

fn setup(mut state: ResMut<State>, asset_server: Res<AssetServer>) {
    // Recommended way to load an asset
    state.handle = asset_server.load("data/asset.custom");

    // File extensions are optional, but are recommended for project management and last-resort inference
    state.other_handle = asset_server.load("data/asset_no_extension");

    // Will use BlobAssetLoader instead of CustomAssetLoader thanks to type inference
    state.blob = asset_server.load("data/asset.custom");
}

fn print_on_load(
    mut state: ResMut<State>,
    custom_assets: Res<Assets<CustomAsset>>,
    blob_assets: Res<Assets<Blob>>,
) {
    let custom_asset = custom_assets.get(&state.handle);
    let other_custom_asset = custom_assets.get(&state.other_handle);
    let blob = blob_assets.get(&state.blob);

    // Can't print results if the assets aren't ready
    if state.printed {
        return;
    }

    if custom_asset.is_none() {
        info!("Custom Asset Not Ready");
        return;
    }

    if other_custom_asset.is_none() {
        info!("Other Custom Asset Not Ready");
        return;
    }

    if blob.is_none() {
        info!("Blob Not Ready");
        return;
    }

    info!("Custom asset loaded: {:?}", custom_asset.unwrap());
    info!("Custom asset loaded: {:?}", other_custom_asset.unwrap());
    info!("Blob Size: {:?} Bytes", blob.unwrap().bytes.len());

    // Once printed, we won't print again
    state.printed = true;
}
