use crate::{
    enums::RoomEnum,
    structs::{
        daily_events_folder::daily_events::DailyEvents,
        general_structs::MissionReportsModalVisible,
        maps::{Maps, SelectedMapId},
        missions::{MissionReports, Missions},
        player_stats::PlayerStats,
        recruits::SelectedRecruitForEquipment,
    },
    ui::rooms::{
        barrack::barrack_room::spawn_room_barrack,
        command_room::room_command_room::room_command_room, office::room_office::room_office,
    },
};
use bevy::prelude::*;

#[derive(Component)]
pub struct ResetRoomTrigger;

#[allow(clippy::too_many_arguments)]
pub fn init_rooms(
    my_assets: Res<AssetServer>,
    player_stats: Res<PlayerStats>,
    mut commands: Commands,
    _query: Query<Entity, With<ResetRoomTrigger>>,
    _previous_room: Local<Option<RoomEnum>>,
    selected_recruit_for_equipment: Res<SelectedRecruitForEquipment>,
    missions: Res<Missions>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mission_reports: Res<MissionReports>,
    mission_reports_modal_visibility: ResMut<MissionReportsModalVisible>,
    maps: Res<Maps>,
    selected_map_id: Res<SelectedMapId>,
    daily_events: Res<DailyEvents>,
) {
    room_office(
        &my_assets,
        &mut commands,
        &mission_reports,
        mission_reports_modal_visibility,
        &daily_events,
    );

    spawn_room_barrack(
        &my_assets,
        &mut commands,
        &player_stats,
        &selected_recruit_for_equipment,
        &mut texture_atlas_layouts,
    );

    room_command_room(
        &my_assets,
        &mut commands,
        missions,
        selected_map_id,
        maps,
        &player_stats,
        &mut texture_atlas_layouts,
    );
}
