// Exemple of a clippy rule for all this file
// #![allow(clippy::type_complexity)]

mod audio;
mod enums;
mod structs;
mod systems;
mod ui;
mod utils;

// ! Make crash the UI
use bevy::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use structs::PlayerStats;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // WorldInspectorPlugin::new(),
        ))
        .insert_resource(PlayerStats::default())
        .add_systems(
            Startup,
            (
                audio::audio_source::audio_source,
                systems::camera::camera_setup::camera_setup,
                systems::inputs::mouse_systems::mouse_init,
                ui::buttons::room_arrows::room_bottom_arrow_button,
                ui::buttons::room_arrows::room_left_arrow_button,
                ui::buttons::room_arrows::room_right_arrow_button,
                ui::buttons::room_arrows::room_top_arrow_button,
                ui::interface::gold_counter::gold_counter,
                ui::interface::room_interface_text::room_interface_text,
                ui::rooms::room_setup::room_setup,
            ),
        )
        .add_systems(
            Update,
            (
                systems::inputs::mouse_systems::mouse_click_system,
                systems::updates::update_gold_counter::update_gold_counter,
                systems::updates::update_room_interface_text::update_room_interface_text,
                systems::updates::update_room::update_room,
                systems::updates::update_buttons::mouse_interaction_updates,
                systems::updates::update_buttons::buttons_disable_updates,
            ),
        )
        .run()
}
