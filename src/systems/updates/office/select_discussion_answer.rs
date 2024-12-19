use crate::structs::{
    daily_events_folder::{
        daily_events::{Answer, DailyEvents},
        discussions::DailyDiscussion,
        spontaneous_applications::SpontaneousApplication,
    },
    general_structs::{DailyEventsModalVisible, NotificationCount},
    player_stats::PlayerStats,
};
use bevy::prelude::*;

pub fn select_discussion_answer(
    mut query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &Answer,
            Option<&DailyDiscussion>,
            Option<&SpontaneousApplication>,
        ),
        (Changed<Interaction>, With<Answer>),
    >,
    mut windows: Query<&mut Window>,
    mut daily_events: ResMut<DailyEvents>,
    mut daily_events_modal_visibility: ResMut<DailyEventsModalVisible>,
    mut player_stats: ResMut<PlayerStats>,
    mut notification_count: ResMut<NotificationCount>,
) {
    let _window = windows.single_mut();
    for (interaction, mut background_color, answer, discussion, spontaneous_application) in
        query.iter_mut()
    {
        let is_answer_disabled = answer.gold_impact.is_some()
            && answer.gold_impact.unwrap().is_negative()
            && player_stats.golds < -answer.gold_impact.unwrap();

        if is_answer_disabled {
            continue;
        }

        match *interaction {
            Interaction::Pressed => {
                if let Some(discussion) = discussion {
                    daily_events.remove_daily_discussion_by_id(discussion.id);
                } else if let Some(application) = spontaneous_application {
                    daily_events.remove_spontaneous_application_by_id(application.id);
                }

                player_stats.apply_equipment_impact(answer, &mut notification_count);

                daily_events_modal_visibility.0 = false;

                if !daily_events.0.is_empty() {
                    daily_events_modal_visibility.0 = true;
                }
            }
            Interaction::Hovered => {
                background_color.0 = Color::srgba(0., 0., 0., 0.4);
            }
            Interaction::None => {
                background_color.0 = Color::srgba(0., 0., 0., 0.5);
            }
        };
    }
}
