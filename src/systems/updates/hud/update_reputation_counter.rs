use crate::{
    structs::player_stats::PlayerStats, ui::hud_folder::right_hud::ReputationCountTrigger,
};
use bevy::prelude::{Entity, Res, Single, Text, TextUiWriter, With};

pub fn update_reputation_counter(
    player_stats: Res<PlayerStats>,
    query: Single<Entity, (With<ReputationCountTrigger>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*query, 0) = player_stats.reputation.to_string();
}
