use bevy::{
    asset::{AssetServer, Assets},
    prelude::{info, Commands, Res, ResMut},
    sprite::TextureAtlasLayout,
};

use crate::ui::rooms::room_office::room_office;

/// # All the UI logic and components will be setup here
///
/// It will load the office room at start
///
/// ## Parameters
/// - `commands`: Bevy's commands
/// - `asset_server`: Bevy's asset server
pub fn room_setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    room_office(&asset_server, &mut commands, texture_atlas_layouts);
    info!("On commence avec la room office");
}