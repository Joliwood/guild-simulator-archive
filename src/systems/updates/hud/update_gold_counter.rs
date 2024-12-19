use crate::{structs::player_stats::PlayerStats, ui::hud_folder::left_hud::GoldCountTrigger};
use bevy::prelude::{Entity, Res, Single, Text, TextUiWriter, With};

pub fn update_gold_counter(
    player_stats: Res<PlayerStats>,
    query: Single<Entity, (With<GoldCountTrigger>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*query, 0) = player_stats.golds.to_string();
}
