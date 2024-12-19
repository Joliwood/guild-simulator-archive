#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_return)]

mod audio;
mod content;
mod custom_components;
mod enums;
mod my_assets;
mod structs;
mod systems;
mod ui;
mod utils;

use bevy::{asset::AssetMetaCheck, prelude::*, window::WindowTheme};
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use content::constants::MAX_GAME_SECONDS;
use custom_components::{
    BarrackRoomNotificationContainerTrigger, BarrackRoomNotificationTrigger,
    CommandRoomNotificationContainerTrigger, CommandRoomNotificationTrigger,
    OfficeRoomNotificationContainerTrigger, OfficeRoomNotificationTrigger,
};
use enums::TextureAtlasLayoutEnum;
use pyri_tooltip::prelude::*;
use structs::{
    daily_events_folder::daily_events::{DailyEventTargets, DailyEvents},
    general_structs::{
        DailyEventsModalVisible, DayTime, MissionModalVisible, MissionNotificationsNumber,
        MissionReportsModalVisible, NotificationCount, TutoDoneModalVisible,
        TutoMessagesModalVisible,
    },
    maps::{Maps, SelectedMapId},
    missions::{MissionReports, Missions, SelectedMission},
    player_stats::{PlayerStats, TutoMessages},
    recruits::{SelectedRecruitForEquipment, SelectedRecruitForMission},
};
use systems::updates::barrack::desequip_selected_recruit::{
    desequip_selected_recruit_armor, desequip_selected_recruit_weapon,
};
use systems::updates::barrack::update_selected_recruit::update_selected_recruit;
use systems::updates::close_tuto_message::close_tuto_done_message;
use systems::updates::command_room::update_mission_icons::{
    update_mission_icons, update_unavailable_mission_icons,
};
use systems::updates::office::update_mission_report_documents::update_mission_report_documents;
use systems::updates::switch_room::switch_room;
use systems::updates::{
    close_tuto_message::close_tuto_message, hud::open_tuto_message::open_tuto_message,
    office::update_daily_event_documents::update_daily_event_documents, skip_tuto::skip_tuto,
};
use ui::hud_folder::sleep_button::{PlayerDayTrigger, RealTimeDayProgressBarTrigger};
use ui::rooms::barrack::inventory::build_inventory_grid::build_inventory_grid;
use ui::rooms::barrack::inventory::spawn_inventory::{
    SpawnInventoryParentTrigger, SpawnInventoryTrigger,
};
use ui::rooms::barrack::recruits_list_folder::recruit_card::recruit_card;
use ui::rooms::barrack::recruits_list_folder::recruits_list::{
    UpdateBarrackRecruitListChildrenTrigger, UpdateBarrackRecruitListParentTrigger,
};
use ui::rooms::command_room::map_recruit_card::map_recruit_card;
use ui::rooms::command_room::map_recruit_list::{
    UpdateMapRecruitListChildrenTrigger, UpdateMapRecruitListParentTrigger,
};
use ui::{
    hud_folder::mayor_notification_toast::mayor_notification_toast,
    modals::{
        tuto_done_modal_folder::tuto_done_modal::tuto_done_modal,
        tuto_messages::tuto_message_modal::tuto_message_modal,
    },
};
use utils::{get_layout, sort_recruits_by_total_power};

// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;

// Config fallback missing translations to "en" locale.
// Use `fallback` option to set fallback locale.
//
i18n!("assets/locales", fallback = "en");

fn setup_i18n() {
    rust_i18n::set_locale("fr");
}

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins
            // Plugin option to manager assets for web / itch.io
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Guild simulator".into(),
                    resizable: true,
                    window_theme: Some(WindowTheme::Dark),
                    canvas: Some("#mygame-canvas".into()),
                    ..default()
                }),
                ..default()
            }),
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
        .insert_resource(TutoMessagesModalVisible(true))
        .insert_resource(TutoDoneModalVisible(false))
        .insert_resource(MissionNotificationsNumber(0))
        .insert_resource(Maps::default())
        .insert_resource(DailyEvents::default())
        .insert_resource(DailyEventTargets::default())
        .insert_resource(DayTime::default())
        .insert_resource(NotificationCount::default())
        .insert_resource(TutoMessages::default())
        .add_systems(
            Startup,
            (
                init_tuto_messages,
                audio::audio_source::audio_source,
                systems::camera::camera_setup::camera_setup,
                ui::hud_folder::hud::hud,
                systems::updates::init_rooms::init_rooms,
            ),
        )
        .add_systems(
            Update,
            (
                systems::inputs::escape::close_modal_on_escape,
                systems::inputs::mouse_systems::mouse_click_system,
                systems::updates::barrack::select_recruit_for_equipment_button::select_recruit_for_equipment_button,
                systems::updates::command_room::select_map::select_map,
                systems::updates::command_room::select_mission_button::select_mission_button,
                systems::updates::command_room::select_recruit_for_mission_button::select_recruit_for_mission_button,
                systems::updates::hud::update_day_counter::update_day_counter.run_if(resource_changed::<PlayerStats>),
                systems::updates::hud::update_gold_counter::update_gold_counter.run_if(resource_changed::<PlayerStats>),
                systems::updates::hud::update_guild_level::update_guild_level.run_if(resource_changed::<PlayerStats>),
                systems::updates::hud::update_recruit_counter::update_recruit_counter.run_if(resource_changed::<PlayerStats>),
                systems::updates::hud::update_reputation_counter::update_reputation_counter.run_if(resource_changed::<PlayerStats>),
                systems::updates::hud::update_toxicity_counter::update_toxicity_counter.run_if(resource_changed::<PlayerStats>),
                systems::updates::input::move_room_from_keyboard,
            ),
        )
        .add_systems(
            Update,
            (
                systems::updates::barrack::select_item_in_inventory::select_item_in_inventory,
                systems::updates::command_room::close_mission_modal::close_mission_modal,
                systems::updates::command_room::start_mission_button::start_mission_button,
                systems::updates::hud::sleep_button_system::sleep_button_system,
                systems::updates::office::select_discussion_answer::select_discussion_answer,
                systems::updates::office::sign_mission_report::sign_mission_report,
                systems::updates::office::toggle_daily_event_documents::toggle_daily_event_documents,
                systems::updates::office::toggle_mission_reports::toggle_mission_reports,
                systems::updates::scroll::scroll,
                systems::updates::update_room_on_click::update_room_on_click,
                ui::modals::daily_events::daily_events_modal::daily_events_modal,
                ui::modals::mission_order_modal_folder::mission_order_modal::mission_order_modal,
                ui::modals::mission_report_modal_folder::mission_report_modal::mission_report_modal,
                update_daytime,
                update_progress_bar,
                systems::updates::hud::update_sleep_button_texture::update_sleep_button_texture,
                skip_tuto,
            ),
        )
        .add_systems(
            Update,
            (
                update_notification_indicators_text_for_command_room
                    .run_if(resource_changed::<NotificationCount>),
                update_notification_indicators_text_for_office_room
                    .run_if(resource_changed::<NotificationCount>),
                update_notification_indicators_text_for_barrack_room
                    .run_if(resource_changed::<NotificationCount>),
                tuto_message_modal,
                close_tuto_message,
                mayor_notification_toast
                    .run_if(resource_changed::<TutoMessages>),
                open_tuto_message,
                tuto_done_modal,
                switch_room,
                update_daily_event_documents
                    .run_if(resource_changed::<DailyEvents>),
                update_mission_icons
                    .run_if(resource_changed::<Missions>),
                update_unavailable_mission_icons
                    .run_if(resource_changed::<Missions>),
            ),
        )
        .add_systems(Update,
            (
                update_mission_report_documents
                    .run_if(resource_changed::<MissionReports>),
                update_selected_recruit
                    .run_if(resource_changed::<SelectedRecruitForEquipment>),
                update_barrack_inventory
                    .run_if(resource_changed::<PlayerStats>),
                update_barrack_recruit_list
                    .run_if(resource_changed::<PlayerStats>),
                update_map_recruit_list
                    .run_if(resource_changed::<PlayerStats>),
                close_tuto_done_message,
                desequip_selected_recruit_armor
                    .run_if(resource_changed::<SelectedRecruitForEquipment>),
                desequip_selected_recruit_weapon
                    .run_if(resource_changed::<SelectedRecruitForEquipment>),
            )
        )
        .run()
}

/// Startup system to force i18n to init before the tuto messages spawn
fn init_tuto_messages(mut commands: Commands) {
    setup_i18n();
    let tuto_messages = TutoMessages::default();
    commands.insert_resource(tuto_messages);
}

fn update_daytime(
    mut day_time: ResMut<DayTime>,
    time: Res<Time>,
    _query: Single<Entity, (With<PlayerDayTrigger>, With<Text>)>,
    _writer: TextUiWriter,
) {
    // Si le temps maximal est atteint, on arrête d'incrémenter
    if day_time.second_count >= MAX_GAME_SECONDS {
        return;
    }

    // On ajoute le temps écoulé en secondes au compteur de temps
    day_time.current_time += time.delta_secs();
    day_time.elapsed_time += time.delta_secs();

    // Exécute tous les secondes
    if day_time.elapsed_time >= 1.0 {
        day_time.second_count += 1;
        day_time.elapsed_time -= 1.0;
    }
}

fn update_progress_bar(
    day_time: Res<DayTime>,
    mut query: Query<&mut Node, With<RealTimeDayProgressBarTrigger>>,
) {
    // Calcul du ratio de progression (entre 0.0 et 1.0)
    let progress_ratio = day_time.current_time / MAX_GAME_SECONDS as f32;

    // Mise à jour de la largeur pour chaque entité avec le trigger `RealTimeDayProgressBarTrigger`
    for mut node in query.iter_mut() {
        node.width = Val::Px(progress_ratio * 70.);
    }
}

pub fn update_notification_indicators_text_for_command_room(
    notification_count: Res<NotificationCount>,
    query_text: Single<Entity, (With<CommandRoomNotificationTrigger>, With<Text>)>,
    mut query_container: Query<(&mut Node, &CommandRoomNotificationContainerTrigger)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*query_text, 0) = notification_count.command_room_count.to_string();

    for (mut node, _) in query_container.iter_mut() {
        node.display = if notification_count.command_room_count > 0 {
            Display::Flex
        } else {
            Display::None
        };
    }
}

pub fn update_notification_indicators_text_for_office_room(
    notification_count: Res<NotificationCount>,
    query_text: Single<Entity, (With<OfficeRoomNotificationTrigger>, With<Text>)>,
    mut query_container: Query<(&mut Node, &OfficeRoomNotificationContainerTrigger)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*query_text, 0) = notification_count.office_count.to_string();

    for (mut node, _) in query_container.iter_mut() {
        node.display = if notification_count.office_count > 0 {
            Display::Flex
        } else {
            Display::None
        };
    }
}

pub fn update_notification_indicators_text_for_barrack_room(
    notification_count: Res<NotificationCount>,
    query_text: Single<Entity, (With<BarrackRoomNotificationTrigger>, With<Text>)>,
    mut query_container: Query<(&mut Node, &BarrackRoomNotificationContainerTrigger)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*query_text, 0) = notification_count.barrack_count.to_string();

    for (mut node, _) in query_container.iter_mut() {
        node.display = if notification_count.barrack_count > 0 {
            Display::Flex
        } else {
            Display::None
        };
    }
}

// Fonction pour mettre à jour l'inventaire
pub fn update_barrack_inventory(
    mut commands: Commands,
    player_stats: Res<PlayerStats>,
    my_assets: Res<AssetServer>,
    parent_query: Query<Entity, With<SpawnInventoryParentTrigger>>,
    childs_query: Query<Entity, With<SpawnInventoryTrigger>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Step 1: Find the parent entity
    if let Some(parent_entity) = parent_query.iter().next() {
        // Step 2: Despawn all children of the parent
        for child in childs_query.iter() {
            commands.entity(child).despawn_recursive();
        }

        // Step 3: Recreate the inventory grid as children of the parent
        commands.entity(parent_entity).with_children(|grid| {
            build_inventory_grid(grid, &player_stats, &my_assets, &mut texture_atlas_layouts);
        });
    }
}

// Fonction pour mettre à jour l'inventaire
pub fn update_barrack_recruit_list(
    mut commands: Commands,
    player_stats: Res<PlayerStats>,
    my_assets: Res<AssetServer>,
    parent_query: Query<Entity, With<UpdateBarrackRecruitListParentTrigger>>,
    childs_query: Query<Entity, With<UpdateBarrackRecruitListChildrenTrigger>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Step 1: Find the parent entity
    if let Some(parent_entity) = parent_query.iter().next() {
        // Step 2: Despawn all children of the parent
        for child in childs_query.iter() {
            commands.entity(child).despawn_recursive();
        }

        let recruit_layout = get_layout(TextureAtlasLayoutEnum::Recruit);
        let recruit_texture_atlas_layout: Handle<TextureAtlasLayout> =
            texture_atlas_layouts.add(recruit_layout);

        // Step 3: Recreate the inventory grid as children of the parent
        commands.entity(parent_entity).with_children(|parent| {
            for recruit in player_stats.recruits.iter() {
                recruit_card(
                    parent,
                    &my_assets,
                    &player_stats,
                    recruit,
                    recruit_texture_atlas_layout.clone(),
                    &mut texture_atlas_layouts,
                );
            }
        });
    }
}

// Fonction pour mettre à jour l'inventaire
pub fn update_map_recruit_list(
    mut commands: Commands,
    player_stats: Res<PlayerStats>,
    my_assets: Res<AssetServer>,
    parent_query: Query<Entity, With<UpdateMapRecruitListParentTrigger>>,
    childs_query: Query<Entity, With<UpdateMapRecruitListChildrenTrigger>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Step 1: Find the parent entity
    if let Some(parent_entity) = parent_query.iter().next() {
        // Step 2: Despawn all children of the parent
        for child in childs_query.iter() {
            commands.entity(child).despawn_recursive();
        }

        let recruit_layout = get_layout(TextureAtlasLayoutEnum::Recruit);
        let recruit_texture_atlas_layout: Handle<TextureAtlasLayout> =
            texture_atlas_layouts.add(recruit_layout);

        // Step 3: Recreate the inventory grid as children of the parent
        commands.entity(parent_entity).with_children(|parent| {
            let sorted_recruits = sort_recruits_by_total_power(player_stats.recruits.clone());

            // Barrack room > left container > recruit buttons
            for recruit in sorted_recruits.iter() {
                map_recruit_card(
                    parent,
                    &my_assets,
                    recruit,
                    recruit_texture_atlas_layout.clone(),
                );
            }
        });
    }
}
