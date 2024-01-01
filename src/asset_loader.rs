use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct ImageAssets {
    pub player: Handle<Image>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ImageAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<ImageAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = ImageAssets {
        player: asset_server.load("player.png"),
    }
}
