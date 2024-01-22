use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy, States, Default)]
pub enum AppState {
    #[default]
    Setup,
    // AssetLoading,
    // Loading,
    // InGame,
    // LevelDone,
    // Finished,
}
