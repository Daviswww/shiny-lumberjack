use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct ImageAssets {
    pub player_run: Handle<Image>,
    pub player: Handle<Image>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ImageAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut image_assets: ResMut<ImageAssets>, asset_server: Res<AssetServer>) {
    *image_assets = ImageAssets {
        player_run: asset_server.load("run.png"),
        player: asset_server.load("player.png"),
    }
}
