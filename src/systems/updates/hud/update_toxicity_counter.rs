use crate::{structs::player_stats::PlayerStats, ui::hud_folder::right_hud::ToxicityCountTrigger};
use bevy::prelude::{Entity, Res, Single, Text, TextUiWriter, With};

pub fn update_toxicity_counter(
    player_stats: Res<PlayerStats>,
    query: Single<Entity, (With<ToxicityCountTrigger>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*query, 0) = player_stats.toxicity.to_string();
}
