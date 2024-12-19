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
    ui::rooms::office::daily_event_documents::DailyEventTrigger,
    utils::reset_modals_visibility,
};
use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn toggle_daily_event_documents(
    my_assets: Res<AssetServer>,
    mut commands: Commands,
    mut query: Query<&Interaction, (Changed<Interaction>, With<DailyEventTrigger>)>,
    mut windows: Query<&mut Window>,
    mut daily_events_modal_visibility: ResMut<DailyEventsModalVisible>,
    mut mission_modal_visibility: ResMut<MissionModalVisible>,
    mut mission_reports_modal_visibility: ResMut<MissionReportsModalVisible>,
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
                daily_events_modal_visibility.0 = true;
                play_sound(&my_assets, &mut commands, SoundEnum::PickingGolds);
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
