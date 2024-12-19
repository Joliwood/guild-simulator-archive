use crate::{
    enums::{ColorPaletteEnum, RecruitStateEnum},
    my_assets::FONT_FIRA,
    structs::{general_structs::UniqueId, player_stats::PlayerStats, recruits::RecruitStats},
    ui::rooms::barrack::recruits_list_folder::{
        armor_button::armor_button, scroll_button::scroll_button, weapon_button::weapon_button,
    },
};
use bevy::{prelude::*, ui::widget::NodeImageMode};

use super::{
    recruit_attack::recruit_attack, recruit_defense::recruit_defense,
    recruits_list::UpdateBarrackRecruitListChildrenTrigger,
};

pub fn recruit_card(
    parent: &mut ChildBuilder,
    my_assets: &Res<AssetServer>,
    player_stats: &Res<PlayerStats>,
    recruit: &RecruitStats,
    recruit_texture_atlas_layout: Handle<TextureAtlasLayout>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    parent
        .spawn((
            Button,
            ImageNode {
                image: my_assets.load("images/rooms/barrack/recruit_card_background.png"),
                image_mode: NodeImageMode::Stretch,
                ..default()
            },
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                width: Val::Percent(100.0),
                height: Val::Px(100.0),
                padding: UiRect {
                    top: Val::Px(7.),
                    bottom: Val::Px(7.),
                    left: Val::Px(7.),
                    right: Val::Px(7.),
                },
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor(Color::BLACK),
            BorderRadius::all(Val::Px(10.)),
            UpdateBarrackRecruitListChildrenTrigger,
        ))
        .insert(PickingBehavior {
            should_block_lower: false,
            ..default()
        })
        .insert((UniqueId("recruit_button".to_string()), recruit.clone()))
        .with_children(|parent| {
            // Add an overlay if the recruit is in a mission
            if recruit.state == RecruitStateEnum::InMission
                || recruit.state == RecruitStateEnum::WaitingReportSignature
                || recruit.state == RecruitStateEnum::Injured
            {
                parent
                    .spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(0.),
                            right: Val::Px(0.),
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            padding: UiRect {
                                top: Val::ZERO,
                                bottom: Val::ZERO,
                                left: Val::Percent(25.),
                                right: Val::Percent(25.),
                            },
                            ..default()
                        },
                        GlobalZIndex(1),
                        BorderRadius::all(Val::Px(10.)),
                        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
                    ))
                    .insert(PickingBehavior {
                        should_block_lower: false,
                        ..default()
                    })
                    .with_children(|overlay| {
                        overlay
                            .spawn((
                                Text::new(t!(recruit.state.get_description())),
                                TextFont {
                                    font: my_assets.load(FONT_FIRA),
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ))
                            .insert(PickingBehavior {
                                should_block_lower: false,
                                ..default()
                            });
                    });
            }
        })
        .with_children(|button| {
            // Recruit portrait image (left-most side)
            button
                .spawn((
                    ImageNode {
                        image: my_assets.load("images/rooms/barrack/recruit_avatar_card_frame.png"),
                        ..default()
                    },
                    Node {
                        width: Val::Px(80.0),
                        height: Val::Px(80.0),
                        margin: UiRect {
                            top: Val::Px(0.),
                            bottom: Val::Px(0.),
                            left: Val::Px(5.),
                            right: Val::Px(5.),
                        },
                        overflow: Overflow {
                            x: OverflowAxis::Hidden,
                            y: OverflowAxis::Hidden,
                        },
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    GlobalZIndex(5),
                ))
                .insert(PickingBehavior {
                    should_block_lower: false,
                    ..default()
                })
                .with_children(|button| {
                    button
                        .spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                align_items: AlignItems::FlexStart,
                                justify_content: JustifyContent::Center,
                                overflow: Overflow {
                                    x: OverflowAxis::Clip,
                                    y: OverflowAxis::Clip,
                                },
                                ..default()
                            },
                            BackgroundColor(Color::NONE),
                            GlobalZIndex(3),
                        ))
                        .insert(PickingBehavior {
                            should_block_lower: false,
                            ..default()
                        })
                        .with_children(|frame| {
                            // ImageNode that is 200x400, clipped by the parent container
                            frame
                                .spawn((
                                    ImageNode::from_atlas_image(
                                        my_assets.load("images/recruits/recruit_picture_atlas.png"),
                                        TextureAtlas {
                                            index: recruit.image_atlas_index.into(),
                                            layout: recruit_texture_atlas_layout.clone(),
                                        },
                                    ),
                                    Node {
                                        width: Val::Px(80.),
                                        height: Val::Px(140.),
                                        position_type: PositionType::Absolute,
                                        ..default()
                                    },
                                ))
                                .insert(PickingBehavior {
                                    should_block_lower: false,
                                    ..default()
                                });
                        });
                });

            // Container for recruit name and class
            button
                .spawn(Node {
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
                })
                .insert(PickingBehavior {
                    should_block_lower: false,
                    ..default()
                })
                .with_children(|name_class_container| {
                    // Recruit name
                    name_class_container
                        .spawn((
                            Text::new(t!(recruit.name.clone())),
                            TextLayout {
                                linebreak: LineBreak::NoWrap,
                                ..default()
                            },
                            TextFont {
                                font: my_assets.load(FONT_FIRA),
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(ColorPaletteEnum::DarkBrown.as_color()),
                        ))
                        .insert(PickingBehavior {
                            should_block_lower: false,
                            ..default()
                        });

                    // Recruit class
                    name_class_container
                        .spawn((
                            Text::new(t!(recruit.class.to_string())),
                            TextFont {
                                font: my_assets.load(FONT_FIRA),
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(ColorPaletteEnum::DarkBrown.as_color()),
                        ))
                        .insert(PickingBehavior {
                            should_block_lower: false,
                            ..default()
                        });

                    // Recruit level
                    name_class_container
                        .spawn((
                            Text::new(format!("{}: {}", t!("level"), recruit.level)),
                            TextFont {
                                font: my_assets.load(FONT_FIRA),
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(ColorPaletteEnum::DarkBrown.as_color()),
                        ))
                        .insert(PickingBehavior {
                            should_block_lower: false,
                            ..default()
                        });
                });

            button
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    width: Val::Px(120.0),
                    margin: UiRect::all(Val::Px(5.0)),
                    ..default()
                })
                .insert(PickingBehavior {
                    should_block_lower: false,
                    ..default()
                })
                .with_children(|stats_container| {
                    // Recruit XP
                    stats_container
                        .spawn((
                            Text::new(format!(
                                "XP: {}/{}",
                                recruit.experience, recruit.max_experience
                            )),
                            TextFont {
                                font: my_assets.load(FONT_FIRA),
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(ColorPaletteEnum::DarkBrown.as_color()),
                        ))
                        .insert(PickingBehavior {
                            should_block_lower: false,
                            ..default()
                        });

                    let additionnal_attack_from_items = recruit.get_additional_stats_from_items().0;

                    let additionnal_defense_from_items =
                        recruit.get_additional_stats_from_items().1;

                    recruit_attack(
                        stats_container,
                        recruit,
                        additionnal_attack_from_items,
                        my_assets,
                    );

                    recruit_defense(
                        stats_container,
                        recruit,
                        additionnal_defense_from_items,
                        my_assets,
                    );
                });

            button
                .spawn(Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(2.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Start,
                    ..default()
                })
                .insert(PickingBehavior {
                    should_block_lower: false,
                    ..default()
                })
                .with_children(|equipments_container| {
                    // Top container for weapon and armor
                    equipments_container
                        .spawn(Node {
                            display: Display::Flex,
                            column_gap: Val::Px(2.0),
                            align_self: AlignSelf::FlexEnd,
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .insert(PickingBehavior {
                            should_block_lower: false,
                            ..default()
                        })
                        .with_children(|top_container| {
                            weapon_button(top_container, my_assets, recruit, texture_atlas_layouts);
                            armor_button(top_container, my_assets, recruit, texture_atlas_layouts);
                        });

                    // Bottom container for scrolls
                    equipments_container
                        .spawn(Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(2.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .insert(PickingBehavior {
                            should_block_lower: false,
                            ..default()
                        })
                        .with_children(|bottom_container| {
                            scroll_button(
                                player_stats,
                                bottom_container,
                                my_assets,
                                recruit,
                                texture_atlas_layouts,
                                0,
                            );

                            scroll_button(
                                player_stats,
                                bottom_container,
                                my_assets,
                                recruit,
                                texture_atlas_layouts,
                                1,
                            );

                            scroll_button(
                                player_stats,
                                bottom_container,
                                my_assets,
                                recruit,
                                texture_atlas_layouts,
                                2,
                            );
                        });
                });
        });
}
