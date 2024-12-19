use crate::{
    audio::play_sound::play_sound,
    enums::SoundEnum,
    structs::{
        general_structs::{
            DailyEventsModalVisible, MissionModalVisible, MissionReportsModalVisible,
            TutoMessagesModalVisible,
        },
        recruits::SelectedRecruitForMission,
    },
    ui::rooms::office::mission_report_documents::MissionReportTrigger,
    utils::reset_modals_visibility,
};
use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn toggle_mission_reports(
    my_assets: Res<AssetServer>,
    mut commands: Commands,
    mut query: Query<&Interaction, (Changed<Interaction>, With<MissionReportTrigger>)>,
    mut windows: Query<&mut Window>,
    mut daily_events_modal_visibility: ResMut<DailyEventsModalVisible>,
    mut mission_reports_modal_visibility: ResMut<MissionReportsModalVisible>,
    mut mission_modal_visibility: ResMut<MissionModalVisible>,
    mut selected_recruit_for_mission: ResMut<SelectedRecruitForMission>,
    mut tuto_messages_modal_visibility: ResMut<TutoMessagesModalVisible>,
) {
    let _window = windows.single_mut();

    for interaction in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                reset_modals_visibility(
                    &mut mission_modal_visibility,
                    &mut mission_reports_modal_visibility,
                    &mut daily_events_modal_visibility,
                    &mut selected_recruit_for_mission,
                    &mut tuto_messages_modal_visibility,
                );
                mission_reports_modal_visibility.0 = true;
                play_sound(&my_assets, &mut commands, SoundEnum::PaperTouch);
            }
            Interaction::Hovered => {
                // window.cursor.icon = CursorIcon::Pointer;
            }
            Interaction::None => {
                // window.cursor.icon = CursorIcon::Default;
            }
        }
    }
}
