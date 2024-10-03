use crate::{
    enums::ColorPaletteEnum,
    structs::{
        general_structs::{PlayerStats, UniqueId},
        trigger_structs::SelectedRecruitTrigger,
    },
    ui::rooms::barrack::recruits_list::{
        armor_button::armor_button, scroll_button::scroll_button, weapon_button::weapon_button,
    },
};
use bevy::prelude::*;

use super::{
    recruit_endurance::recruit_endurance, recruit_intelligence::recruit_intelligence,
    recruit_strength::recruit_strength,
};

/// Spawns the left container, displaying the player's recruits.
pub fn spawn_left_container(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    player_stats: &Res<PlayerStats>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let inventory_container_image_handle: Handle<Image> =
        asset_server.load("images/rooms/barrack/inventory_container.png");

    let recruit_image_handle: Handle<Image> =
        asset_server.load("images/recruits/recruit_picture_atlas.png");

    let recruit_layout = TextureAtlasLayout::from_grid(
        UVec2::new(800, 200),
        5,
        1,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    );
    let recruit_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(recruit_layout);

    parent
        // Left container
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Px(400.),
                height: Val::Px(450.),
                padding: UiRect::all(Val::Px(15.0)),
                overflow: Overflow::clip(),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Barrack > left container"))
        .with_children(|left_container| {
            // Background image
            left_container.spawn(ImageBundle {
                image: inventory_container_image_handle.into(),
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            });

            // Barrack room > left container > recruit buttons
            for recruit in player_stats.recruits.iter() {
                left_container
                    // Container for each recruit (button)
                    .spawn(ButtonBundle {
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            width: Val::Percent(100.0),
                            height: Val::Px(100.0),
                            margin: UiRect::all(Val::Px(5.0)),
                            padding: UiRect {
                                top: Val::Px(15.),
                                bottom: Val::Px(15.),
                                left: Val::Px(7.),
                                right: Val::Px(7.),
                            },
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        border_color: BorderColor(ColorPaletteEnum::DarkBrown.as_color()),
                        border_radius: BorderRadius::all(Val::Px(10.)),
                        ..default()
                    })
                    .insert((
                        UniqueId("recruit_button".to_string()),
                        SelectedRecruitTrigger,
                        recruit.clone(),
                    ))
                    .with_children(|button| {
                        // Recruit portrait image (left-most side)
                        button
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Px(80.0),
                                    height: Val::Px(80.0),
                                    margin: UiRect {
                                        top: Val::Px(0.),
                                        bottom: Val::Px(0.),
                                        left: Val::Px(5.),
                                        right: Val::Px(10.),
                                    },
                                    overflow: Overflow {
                                        x: OverflowAxis::Hidden,
                                        y: OverflowAxis::Hidden,
                                    },
                                    align_items: AlignItems::FlexStart,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                background_color: BackgroundColor(Color::NONE),
                                ..default()
                            })
                            .with_children(|frame| {
                                // Image that is 200x400, clipped by the parent container
                                frame.spawn((
                                    ImageBundle {
                                        image: recruit_image_handle.clone().into(),
                                        style: Style {
                                            width: Val::Px(80.),
                                            height: Val::Px(140.),
                                            position_type: PositionType::Absolute,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    TextureAtlas {
                                        index: recruit.image_atlas_index.into(),
                                        layout: recruit_texture_atlas_layout.clone(),
                                    },
                                ));
                            });

                        // Container for recruit name and class
                        button
                            .spawn(NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::FlexStart,
                                    row_gap: Val::Px(5.0),
                                    width: Val::Px(90.0),
                                    overflow: Overflow {
                                        x: OverflowAxis::Hidden,
                                        y: OverflowAxis::Hidden,
                                    },
                                    margin: UiRect::all(Val::Px(5.0)),
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|name_class_container| {
                                // Recruit name
                                name_class_container.spawn(TextBundle::from_section(
                                    recruit.name.clone(),
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 20.0,
                                        color: ColorPaletteEnum::DarkBrown.as_color(),
                                    },
                                ));

                                // Recruit class
                                name_class_container.spawn(TextBundle::from_section(
                                    recruit.class.to_string(),
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 18.0,
                                        color: ColorPaletteEnum::DarkBrown.as_color(),
                                    },
                                ));

                                // Recruit level
                                name_class_container.spawn(TextBundle::from_section(
                                    format!("Level: {}", recruit.level),
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 18.0,
                                        color: ColorPaletteEnum::DarkBrown.as_color(),
                                    },
                                ));
                            });

                        // Container for recruit stats (strength, armor, intelligence)
                        button
                            .spawn(NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Column,
                                    width: Val::Px(120.0),
                                    margin: UiRect::all(Val::Px(5.0)),
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|stats_container| {
                                // Recruit XP
                                stats_container.spawn(TextBundle::from_section(
                                    format!(
                                        "XP: {}/{}",
                                        recruit.experience, recruit.max_experience
                                    ),
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 18.0,
                                        color: ColorPaletteEnum::DarkBrown.as_color(),
                                    },
                                ));

                                let get_additional_strength_from_items =
                                    recruit.get_additional_strength_from_items();

                                recruit_strength(
                                    // TODO - Fix common type for stats
                                    stats_container,
                                    recruit.strength.into(),
                                    get_additional_strength_from_items,
                                    &asset_server,
                                );

                                recruit_endurance(
                                    stats_container,
                                    // TODO - Fix common type for stats
                                    recruit.endurance.into(),
                                    recruit.get_additional_endurance_from_items(),
                                    &asset_server,
                                );

                                recruit_intelligence(
                                    stats_container,
                                    // TODO - Fix common type for stats
                                    recruit.intelligence.into(),
                                    recruit.get_additional_intelligence_from_items(),
                                    &asset_server,
                                );
                            });

                        button
                            .spawn(NodeBundle {
                                style: Style {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(2.),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Start,
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|equipments_container| {
                                // Top container for weapon and armor
                                equipments_container
                                    .spawn(NodeBundle {
                                        style: Style {
                                            display: Display::Flex,
                                            // flex_direction: FlexDirection::Row,
                                            column_gap: Val::Px(2.0),
                                            // justify_content: JustifyContent::FlexEnd,
                                            align_self: AlignSelf::FlexEnd,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .with_children(|top_container| {
                                        weapon_button(
                                            top_container,
                                            asset_server,
                                            recruit,
                                            texture_atlas_layouts,
                                        );

                                        armor_button(
                                            top_container,
                                            asset_server,
                                            recruit,
                                            texture_atlas_layouts,
                                        );
                                    });

                                // Bottom container for scrolls
                                equipments_container
                                    .spawn(NodeBundle {
                                        style: Style {
                                            display: Display::Flex,
                                            flex_direction: FlexDirection::Row,
                                            column_gap: Val::Px(2.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .with_children(|bottom_container| {
                                        scroll_button(
                                            player_stats,
                                            bottom_container,
                                            asset_server,
                                            recruit,
                                            texture_atlas_layouts,
                                            0,
                                        );

                                        scroll_button(
                                            player_stats,
                                            bottom_container,
                                            asset_server,
                                            recruit,
                                            texture_atlas_layouts,
                                            1,
                                        );

                                        scroll_button(
                                            player_stats,
                                            bottom_container,
                                            asset_server,
                                            recruit,
                                            texture_atlas_layouts,
                                            2,
                                        );
                                    });
                            });
                    });
            }
        });
}
