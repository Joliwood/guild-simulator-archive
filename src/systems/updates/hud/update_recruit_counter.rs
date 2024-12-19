use crate::{structs::player_stats::PlayerStats, ui::hud_folder::right_hud::RecruitCountTrigger};
use bevy::prelude::{Entity, Res, Single, Text, TextUiWriter, With};

pub fn update_recruit_counter(
    player_stats: Res<PlayerStats>,
    query: Single<Entity, (With<RecruitCountTrigger>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*query, 0) = player_stats.recruits.len().to_string();
}
