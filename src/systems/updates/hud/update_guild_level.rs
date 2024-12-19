use crate::{structs::player_stats::PlayerStats, ui::hud_folder::left_hud::GuildLvlTrigger};
use bevy::prelude::{Entity, Res, Single, Text, TextUiWriter, With};

pub fn update_guild_level(
    player_stats: Res<PlayerStats>,
    query: Single<Entity, (With<GuildLvlTrigger>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*query, 0) = format!(
        "{} : {}",
        t!("lvl", level = player_stats.guild_level),
        player_stats.guild_level
    );
}
