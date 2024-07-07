use crate::ui::rooms::room_office::room_office;
use bevy::prelude::*;

/// # All the UI logic and components will be setup here
///
/// It will load the office room at start
///
/// ## Parameters
/// - `commands`: Bevy's commands
/// - `asset_server`: Bevy's asset server
pub fn room_setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    room_office(&asset_server, &mut commands);
    print!("On commence avec la room office");
}
