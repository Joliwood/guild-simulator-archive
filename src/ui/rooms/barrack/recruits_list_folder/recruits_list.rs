use super::recruit_card::recruit_card;
use crate::{enums::TextureAtlasLayoutEnum, structs::player_stats::PlayerStats, utils::get_layout};
use bevy::prelude::*;

#[derive(Component)]
pub struct UpdateBarrackRecruitListParentTrigger;

#[derive(Component)]
pub struct UpdateBarrackRecruitListChildrenTrigger;

/// Spawns the left container, displaying the player's recruits.
pub fn spawn_left_container(
    parent: &mut ChildBuilder,
    my_assets: &Res<AssetServer>,
    player_stats: &Res<PlayerStats>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let recruit_layout = get_layout(TextureAtlasLayoutEnum::Recruit);
    let recruit_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(recruit_layout);

    parent
        .spawn((Node {
            display: Display::Flex,
            align_self: AlignSelf::Center,
            align_items: AlignItems::Center,
            width: Val::Px(400.),
            height: Val::Px(450.),
            ..default()
        },))
        .insert(Name::new("Barrack > Recruits list"))
        .with_children(|left_container| {
            // Background image
            left_container
                .spawn((
                    ImageNode {
                        image: my_assets.load("images/rooms/barrack/inventory_container.png"),
                        ..default()
                    },
                    Node {
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect {
                            top: Val::Percent(6.),
                            bottom: Val::Percent(5.),
                            left: Val::Percent(5.),
                            right: Val::Percent(6.),
                        },
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(5.0),
                                display: Display::Flex,
                                width: Val::Percent(88.),
                                height: Val::Percent(89.),
                                overflow: Overflow::scroll_y(),
                                ..default()
                            },
                            UpdateBarrackRecruitListParentTrigger,
                        ))
                        .with_children(|parent| {
                            // Barrack room > left container > recruit buttons
                            for recruit in player_stats.recruits.iter() {
                                recruit_card(
                                    parent,
                                    my_assets,
                                    player_stats,
                                    recruit,
                                    recruit_texture_atlas_layout.clone(),
                                    texture_atlas_layouts,
                                );
                            }
                        });
                });
        });
}
