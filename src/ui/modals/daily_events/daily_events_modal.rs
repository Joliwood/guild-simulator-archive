use super::{
    discussion_event_doc::discussion_event_doc,
    spontaneous_application_event_doc::SelectAnswerTrigger,
};
use crate::{
    content::daily_events::{
        discussions::get_daily_discussion, spontaneous_applications::get_spontaneous_application,
    },
    structs::{
        daily_events_folder::daily_events::{DailyEvent, DailyEventTypeEnum, DailyEvents},
        general_structs::DailyEventsModalVisible,
        player_stats::PlayerStats,
    },
    ui::modals::daily_events::spontaneous_application_event_doc::spontaneous_application_event_doc,
};
use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
// Show daily events documents on desk
pub fn daily_events_modal(
    mut commands: Commands,
    my_assets: Res<AssetServer>,
    query: Query<Entity, With<SelectAnswerTrigger>>,
    daily_events_modal_visibility: Res<DailyEventsModalVisible>,
    daily_events: Res<DailyEvents>,
    _player_stats: ResMut<PlayerStats>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Despawn existing modals if visibility is set to false
    if daily_events_modal_visibility.is_changed() {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    let daily_events_len = daily_events.0.len();

    // Spawn the mission report modal if visibility is true and there are mission reports
    if daily_events_modal_visibility.is_changed()
        && daily_events_modal_visibility.0
        && daily_events_len > 0
    {
        // let container_image: Handle<ImageNode> =
        //     asset_server.load("images/rooms/barrack/inventory_container.png");

        let last_daily_event: &DailyEvent = match daily_events.get_last_daily_event() {
            Some(report) => report,
            None => return,
        };

        match &last_daily_event.daily_event_type {
            DailyEventTypeEnum::Discussion(discussion) => {
                discussion_event_doc(
                    &mut commands,
                    &my_assets,
                    get_daily_discussion(discussion),
                    texture_atlas_layouts,
                );
            }
            DailyEventTypeEnum::SpontaneousApplication(spontaneous_application) => {
                spontaneous_application_event_doc(
                    &mut commands,
                    &my_assets,
                    get_spontaneous_application(spontaneous_application),
                    texture_atlas_layouts,
                );
            }
        }
    }
}
