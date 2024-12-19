use super::{
    armor_button::armor_button, scroll_button::scroll_button, weapon_button::weapon_button,
};
use crate::{
    enums::ColorPaletteEnum,
    my_assets::FONT_FIRA,
    structs::{player_stats::PlayerStats, recruits::SelectedRecruitForEquipment},
    ui::rooms::barrack::recruit_overview_folder::recruit_overview::RecruitOverviewChildTrigger,
};
use bevy::prelude::*;

pub fn recruit_infos(
    parent: &mut ChildBuilder,
    my_assets: &Res<AssetServer>,
    selected_recruit_for_equipment: &Res<SelectedRecruitForEquipment>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    player_stats: &Res<PlayerStats>,
) {
    parent
        .spawn((
            Name::new("Barrack > recruit overview > recruit infos"),
            ImageNode {
                image: my_assets.load("images/rooms/barrack/inventory_container.png"),
                ..default()
            },
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                height: Val::Auto,
                padding: UiRect {
                    top: Val::Px(15.),
                    bottom: Val::Px(15.),
                    left: Val::Px(20.),
                    right: Val::Px(20.),
                },
                ..default()
            },
            GlobalZIndex(2),
            RecruitOverviewChildTrigger,
        ))
        .with_children(|parent| {
            // Top container (holds Weapons and Armor buttons)
            parent
                .spawn(Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.),
                    height: Val::Percent(50.),
                    ..default()
                })
                .with_children(|top_container| {
                    // Left column (Weapon)
                    top_container
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .with_children(|weapon_column| {
                            // Header for Weapon
                            weapon_column.spawn((
                                Text::new(t!("weapon")),
                                TextFont {
                                    font: my_assets.load(FONT_FIRA),
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(ColorPaletteEnum::DarkBrown.as_color()),
                            ));

                            weapon_button(
                                player_stats,
                                weapon_column,
                                my_assets,
                                selected_recruit_for_equipment,
                                texture_atlas_layouts,
                            );
                        });

                    // Right column (Armor)
                    top_container
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .with_children(|armor_column| {
                            // Header for Armor
                            armor_column.spawn((
                                Text::new(t!("armor")),
                                TextFont {
                                    font: my_assets.load(FONT_FIRA),
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(ColorPaletteEnum::DarkBrown.as_color()),
                            ));

                            armor_button(
                                player_stats,
                                armor_column,
                                my_assets,
                                selected_recruit_for_equipment,
                                texture_atlas_layouts,
                            );
                        });
                });

            // Bottom container (holds Scrolls header and three buttons)
            parent
                .spawn(Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Start,
                    width: Val::Percent(100.),
                    height: Val::Percent(50.),
                    ..default()
                })
                .with_children(|bottom_container| {
                    // Header for Scrolls
                    bottom_container.spawn((
                        Text::new(t!("scrolls")),
                        TextFont {
                            font: my_assets.load(FONT_FIRA),
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(ColorPaletteEnum::DarkBrown.as_color()),
                    ));

                    // Row container for buttons
                    bottom_container
                        .spawn(Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            width: Val::Percent(100.),
                            margin: UiRect::all(Val::Px(5.)),
                            ..default()
                        })
                        .with_children(|scrolls_row| {
                            scroll_button(
                                player_stats,
                                scrolls_row,
                                my_assets,
                                selected_recruit_for_equipment,
                                texture_atlas_layouts,
                                0,
                            );

                            scroll_button(
                                player_stats,
                                scrolls_row,
                                my_assets,
                                selected_recruit_for_equipment,
                                texture_atlas_layouts,
                                1,
                            );

                            scroll_button(
                                player_stats,
                                scrolls_row,
                                my_assets,
                                selected_recruit_for_equipment,
                                texture_atlas_layouts,
                                2,
                            );
                        });
                });
        });
}
