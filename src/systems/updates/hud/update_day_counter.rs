use crate::{structs::player_stats::PlayerStats, ui::hud_folder::sleep_button::PlayerDayTrigger};
use bevy::prelude::{Entity, Res, Single, Text, TextUiWriter, With};

pub fn update_day_counter(
    player_stats: Res<PlayerStats>,
    query: Single<Entity, (With<PlayerDayTrigger>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*query, 0) = format!("{} : {}", t!("day"), player_stats.day);
}
