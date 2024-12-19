use crate::{
    enums::RoomEnum,
    structs::{general_structs::RoomTag, player_stats::PlayerStats},
};
use bevy::prelude::*;

pub fn switch_room(
    previous_room: Local<Option<RoomEnum>>,
    player_stats: Res<PlayerStats>,
    mut query: Query<(&RoomTag, &mut Visibility)>,
) {
    if player_stats.is_changed()
        && previous_room
            .as_ref()
            .map_or(true, |prev| *prev != player_stats.room)
    {
        for (room_tag, mut visibility) in query.iter_mut() {
            *visibility = if room_tag.0 == player_stats.room {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}
