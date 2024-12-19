use crate::{
    audio::play_sound::play_sound,
    enums::{ColorPaletteEnum, RecruitStateEnum, SoundEnum},
    structs::{
        general_structs::{MissionReportsModalVisible, NotificationCount, TutoDoneModalVisible},
        maps::Maps,
        missions::{MissionReport, MissionReports, Missions},
        player_stats::PlayerStats,
    },
    ui::modals::mission_report_modal_folder::mission_report_modal::MissionReportModalSignButtonTrigger,
};
use bevy::prelude::*;
use rand::Rng;

#[allow(clippy::too_many_arguments)]
pub fn sign_mission_report(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut Node,
            &MissionReportModalSignButtonTrigger,
            &mut BackgroundColor,
            &MissionReport,
        ),
        Changed<Interaction>,
    >,
    mut windows: Query<&mut Window>,
    mut mission_reports_modal_visibility: ResMut<MissionReportsModalVisible>,
    mut player_stats: ResMut<PlayerStats>,
    mut missions: ResMut<Missions>,
    mut mission_reports: ResMut<MissionReports>,
    my_assets: Res<AssetServer>,
    mut commands: Commands,
    mut maps: ResMut<Maps>,
    mut notification_count: ResMut<NotificationCount>,
    mut tuto_done_modal_visibility: ResMut<TutoDoneModalVisible>,
) {
    let _window = windows.single_mut();

    for (interaction, mut node, _button_trigger, mut color, mission_report) in
        interaction_query.iter_mut()
    {
        match *interaction {
            Interaction::Pressed => {
                let (recruit_id, mission_id) =
                    (mission_report.recruit_id, mission_report.mission_id);

                player_stats.update_state_of_recruit(
                    mission_report.recruit_id,
                    RecruitStateEnum::Available,
                );

                missions.desassign_recruit_to_mission(mission_id);

                if mission_report.success {
                    if let Some(xp_earned_from_mission) = mission_report.experience_gained {
                        player_stats.gain_xp(xp_earned_from_mission);
                        player_stats.gain_xp_to_recruit(recruit_id, xp_earned_from_mission);
                    } else {
                        panic!("A mission should always have xp to earn, check out the mission contents")
                    }

                    player_stats.increment_golds(mission_report.golds_gained.unwrap());
                    play_sound(&my_assets, &mut commands, SoundEnum::PickingGolds);

                    missions.unlock_missions_by_mission_id(mission_id);

                    for loot in mission_report.loots.iter() {
                        player_stats.add_item(loot.clone());
                    }
                    player_stats.stats.golds_earned += mission_report.golds_gained.unwrap();
                    player_stats.stats.mission_completed += 1;

                    notification_count.increment_barrack_count(
                        mission_report.loots.len() as u8,
                        &mut player_stats,
                    );

                    let map_id = maps.get_map_by_mission_id(mission_id).unwrap().id;

                    let map = maps.get_map_by_id(map_id);

                    if map.is_some() {
                        maps.finish_mission_by_id(mission_id);
                    } else {
                        error!("The mission isn't present in any map, check out the mission & map contents");
                    }

                    if map_id == 1 && mission_id == 6 {
                        tuto_done_modal_visibility.0 = true;
                    }
                } else {
                    let random_number_from_0_to_100 = rand::thread_rng().gen_range(1..=100);
                    if random_number_from_0_to_100 < 25 {
                        // The recruit has 25% chance to be injured if he fails the mission
                        player_stats.update_state_of_recruit(
                            mission_report.recruit_id,
                            RecruitStateEnum::Injured,
                        );
                        play_sound(&my_assets, &mut commands, SoundEnum::DeadMale);
                    }
                }

                mission_reports.remove_mission_report_by_id(mission_id);

                mission_reports_modal_visibility.0 = false;

                play_sound(&my_assets, &mut commands, SoundEnum::PencilSign);

                if !mission_reports.0.is_empty() {
                    mission_reports_modal_visibility.0 = true;
                }
            }
            Interaction::Hovered => {
                // window.cursor.icon = CursorIcon::Pointer;
                *color = BackgroundColor(ColorPaletteEnum::DarkGray.as_color());
                // Add a border when hovered
                node.border = UiRect {
                    left: Val::Px(3.0),
                    right: Val::Px(3.0),
                    top: Val::Px(3.0),
                    bottom: Val::Px(3.0),
                };
            }
            Interaction::None => {
                // Remove the border when not interacted with
                // window.cursor.icon = CursorIcon::Default;
                *color = BackgroundColor(ColorPaletteEnum::DarkBrown.as_color());
                node.border = UiRect::default();
            }
        }
    }
}
