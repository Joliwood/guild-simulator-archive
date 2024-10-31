// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Exemple of a clippy rule for all this file
#![allow(clippy::needless_return)]
#![allow(clippy::type_complexity)]

mod audio;
mod content;
mod custom_components;
mod enums;
mod my_assets;
mod structs;
mod systems;
mod ui;
mod utils;

use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_asset_loader::asset_collection::AssetCollectionApp;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use pyri_tooltip::prelude::*;
use structs::{
    daily_events_folder::daily_events::{DailyEventTargets, DailyEvents},
    general_structs::{
        DailyEventsModalVisible, MissionModalVisible, MissionNotificationsNumber,
        MissionReportsModalVisible,
    },
    maps::{Maps, SelectedMapId},
    missions::{MissionReports, Missions, SelectedMission},
    player_stats::PlayerStats,
    recruits::{SelectedRecruitForEquipment, SelectedRecruitForMission},
};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
pub struct MySystems;

#[derive(Component)]
pub struct AlertButton;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    canvas: Some("#mygame-canvas".into()),
                    ..default()
                }),
                ..default()
            }),
            // Desactivate on testing
            // WorldInspectorPlugin::new(),
            TooltipPlugin::default(),
        ))
        .insert_resource(PlayerStats::default())
        .insert_resource(MissionReports::default())
        .insert_resource(Missions::default())
        .insert_resource(SelectedRecruitForEquipment::default())
        .insert_resource(SelectedRecruitForMission::default())
        .insert_resource(SelectedMission::default())
        .insert_resource(SelectedMapId::default())
        .insert_resource(MissionModalVisible(false))
        .insert_resource(MissionReportsModalVisible(false))
        .insert_resource(DailyEventsModalVisible(false))
        .insert_resource(MissionNotificationsNumber(0))
        .insert_resource(Maps::default())
        .insert_resource(DailyEvents::default())
        .insert_resource(DailyEventTargets::default())
        .init_collection::<my_assets::MyAssets>()
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
                systems::recruits::hiring_setup::hiring_setup,
            ),
        )
        .add_systems(
            Update,
            (
                systems::updates::update_buttons::move_room_from_keyboard,
                systems::inputs::mouse_systems::mouse_click_system,
                systems::updates::interfaces::update_gold_counter::update_gold_counter,
                systems::updates::interfaces::update_room_interface_text::update_room_interface_text,
                systems::updates::update_room::update_room,
                systems::updates::update_buttons::mouse_interaction_updates,
                systems::updates::update_buttons::buttons_disable_updates,
                systems::updates::barrack::select_recruit_for_equipment_button::select_recruit_for_equipment_button,
                systems::updates::command_room::select_recruit_for_mission_button::select_recruit_for_mission_button,
                systems::updates::command_room::select_mission_button::select_mission_button,
            ),
        )
        .add_systems(
            Update,
            (
                systems::updates::command_room::assign_recruit_to_mission::assign_recruit_to_mission,
                systems::updates::command_room::close_mission_modal::close_mission_modal,
                systems::updates::command_room::start_mission_button::start_mission_button,
                systems::updates::barrack::select_item_in_inventory::select_item_in_inventory,
                systems::updates::command_room::update_selected_recruit_for_equipment::update_selected_mission_recruit_id,
                systems::updates::command_room::update_selected_recruit_for_equipment::update_update_selected_mission_percentage_of_victory,
                systems::updates::interfaces::delete_notifications_on_click::delete_notifications_on_click,
                ui::modals::mission_order_modal_folder::mission_order_modal::mission_order_modal,
                ui::modals::daily_events::daily_events_modal::daily_events_modal,
                ui::modals::mission_report_modal_folder::mission_report_modal::mission_report_modal,
            ),
        )
        .add_systems(
            Update,
            (
                systems::updates::interfaces::sleep_button_system::sleep_button_system,
                systems::updates::office::toggle_mission_reports::toggle_mission_reports,
                systems::updates::office::toggle_daily_event_documents::toggle_daily_event_documents,
                systems::updates::office::select_discussion_answer::select_discussion_answer,
                systems::updates::office::sign_mission_report::sign_mission_report,
            ),
        )
        .run()
}
