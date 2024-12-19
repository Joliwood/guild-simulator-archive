use crate::{
    audio::play_sound::play_sound,
    enums::SoundEnum,
    structs::{
        daily_events_folder::daily_events::{DailyEventTargets, DailyEvents},
        general_structs::{DayTime, NotificationCount},
        missions::{MissionReports, Missions},
        player_stats::{PlayerStats, TutoEnum, TutoMessages},
    },
    ui::hud_folder::{
        mayor_notification_toast::NotificationToastTrigger, sleep_button::SleepButtonTrigger,
    },
    utils::finish_mission,
};
use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn sleep_button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &SleepButtonTrigger, &mut BorderColor),
        Changed<Interaction>,
    >,
    mut player_stats: ResMut<PlayerStats>,
    my_assets: Res<AssetServer>,
    mut missions: ResMut<Missions>,
    mut mission_reports: ResMut<MissionReports>,
    query: Query<Entity, With<NotificationToastTrigger>>,
    mut windows: Query<&mut Window>,
    mut daily_events: ResMut<DailyEvents>,
    mut daily_event_targets: ResMut<DailyEventTargets>,
    mut day_time: ResMut<DayTime>,
    mut notification_count: ResMut<NotificationCount>,
    mut tuto_messages: ResMut<TutoMessages>,
) {
    let _window = windows.single_mut();

    for (interaction, _button_trigger, mut border_color) in interaction_query.iter_mut() {
        if !mission_reports.0.is_empty() || !daily_events.0.is_empty() {
            continue;
        }

        match *interaction {
            Interaction::Pressed => {
                // Increment the day in player_stats
                player_stats.day += 1;

                if player_stats.day == 2 && player_stats.tuto.is_first_daily_events_done.is_none() {
                    player_stats.tuto.is_first_daily_events_done = Some(false);
                    tuto_messages.add_tuto_message(TutoEnum::DailyEvents);
                }

                play_sound(&my_assets, &mut commands, SoundEnum::KeysRemovedFromDoor);
                // play_sound(&my_assets, &mut commands, SoundEnum::CockrelMorning);

                // We iterate on every missions to decrement the days left for every mission that days_left.is_some()
                let mission_ids: Vec<u16> = missions
                    .0
                    .iter()
                    .filter(|mission| mission.days_left.is_some())
                    .map(|mission| mission.id)
                    .collect();
                for mission_id in mission_ids {
                    missions.decrement_days_left_by_mission_id(mission_id);
                    let is_mission_over = missions.is_mission_over(mission_id);

                    if is_mission_over {
                        if let Some(percent_of_victory) =
                            missions.get_percent_of_victory_by_mission_id(mission_id)
                        {
                            finish_mission(
                                &mut player_stats,
                                mission_id,
                                &mut missions,
                                percent_of_victory as f32,
                                &mut mission_reports,
                            );

                            for entity in query.iter() {
                                commands.entity(entity).despawn_recursive();
                            }
                        }
                    }
                }

                let new_daily_events = daily_events.get_random_number_of_daily_events(
                    3,
                    player_stats.day,
                    &mut daily_event_targets,
                );
                daily_events.0 = new_daily_events;

                let recruit_length = player_stats.recruits.len();
                player_stats.increment_golds(recruit_length as i32 * -2);

                notification_count.increment_office_count(
                    (daily_events.0.len() + mission_reports.0.len()) as u8,
                    &mut player_stats,
                );
                day_time.reset();

                player_stats.heal_all_injured_recruits();

                border_color.0 = Color::NONE;
            }
            Interaction::Hovered => {
                // window.cursor.icon = CursorIcon::Pointer;
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                // window.cursor.icon = CursorIcon::Default;
                border_color.0 = Color::NONE;
            }
        }
    }
}
