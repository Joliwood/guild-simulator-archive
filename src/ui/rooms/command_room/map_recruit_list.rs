use super::map_recruit_card::map_recruit_card;
use crate::{
    enums::TextureAtlasLayoutEnum,
    structs::player_stats::PlayerStats,
    utils::{get_layout, sort_recruits_by_total_power},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct UpdateMapRecruitListParentTrigger;

#[derive(Component)]
pub struct UpdateMapRecruitListChildrenTrigger;

pub fn map_recruit_list(
    parent: &mut ChildBuilder,
    my_assets: &Res<AssetServer>,
    player_stats: &Res<PlayerStats>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let recruit_layout = get_layout(TextureAtlasLayoutEnum::Recruit);
    let recruit_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(recruit_layout);

    parent
        .spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                row_gap: Val::Px(5.0),
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                height: Val::Auto,
                padding: UiRect {
                    left: Val::Px(2.),
                    ..default()
                },
                overflow: Overflow::scroll_y(),
                ..default()
            },
            UpdateMapRecruitListParentTrigger,
        ))
        .with_children(|left_container| {
            let sorted_recruits = sort_recruits_by_total_power(player_stats.recruits.clone());

            // Barrack room > left container > recruit buttons
            for recruit in sorted_recruits.iter() {
                map_recruit_card(
                    left_container,
                    my_assets,
                    recruit,
                    recruit_texture_atlas_layout.clone(),
                );
            }
        });
}
