use crate::{
    structs::daily_events_folder::daily_events::DailyEvents,
    ui::rooms::office::daily_event_documents::{DailyEventTextTrigger, DailyEventTrigger},
};
use bevy::prelude::*;

pub fn update_daily_event_documents(
    daily_events: Res<DailyEvents>,
    mut query: Query<(&mut Node, &mut DailyEventTrigger)>,
    text_query: Single<Entity, (With<DailyEventTextTrigger>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    let daily_events_number = daily_events.0.len();

    for (mut node, _) in query.iter_mut() {
        if daily_events_number > 0 {
            node.display = Display::Flex;
        } else {
            node.display = Display::None;
        }
    }

    *writer.text(*text_query, 0) = format!("{}", daily_events_number);
}
