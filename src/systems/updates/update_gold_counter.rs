use crate::structs::{GoldCountTrigger, PlayerStats};
use bevy::{
    prelude::{Query, Res, With},
    text::Text,
};

/// # Update the gold counter text (top left of the user screen)
///
/// ## Parameters
/// - `player_stats`: Where we take the informations to update the query
/// - `query`: The element that will be updated (has to ba added in an .insert() method in the node)
pub fn update_gold_counter(
    player_stats: Res<PlayerStats>,
    mut query: Query<&mut Text, With<GoldCountTrigger>>,
) -> () {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{}", player_stats.golds);
    }
}
