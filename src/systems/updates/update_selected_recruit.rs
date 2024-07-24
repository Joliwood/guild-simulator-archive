use crate::structs::{
    SelectedMission, SelectedMissionPercentOfVictoryTrigger, SelectedMissionRecruitIdTrigger,
};
use bevy::{
    log::info,
    prelude::{DetectChanges, Query, Res, With},
    text::Text,
};

/// # Update the gold counter text (top left of the user screen)
///
/// ## Parameters
/// - `player_stats`: Where we take the informations to update the query
/// - `query`: The element that will be updated (has to ba added in an .insert() method in the node)
pub fn update_selected_mission_recruit_id(
    selected_mission: Res<SelectedMission>,
    mut query: Query<&mut Text, With<SelectedMissionRecruitIdTrigger>>,
) -> () {
    if selected_mission.is_changed() {
        info!(
            "the recruit assigned to the selected mission is now : {:?}",
            selected_mission
        );
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("{:?}", selected_mission.recruit_id.unwrap());
        }
    }
}

pub fn update_update_selected_mission_percentage_of_victory(
    selected_mission: Res<SelectedMission>,
    mut query: Query<&mut Text, With<SelectedMissionPercentOfVictoryTrigger>>,
) -> () {
    if selected_mission.is_changed() {
        info!(
            "the recruit assigned to the selected mission is now : {:?}%",
            selected_mission
        );
        for mut text in query.iter_mut() {
            text.sections[0].value =
                format!("{:?} %", selected_mission.percent_of_victory.unwrap());
        }
    }
}
