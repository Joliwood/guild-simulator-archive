use super::map_recruit_list::UpdateMapRecruitListChildrenTrigger;
use crate::{
    enums::{ColorPaletteEnum, RecruitStateEnum},
    my_assets::FONT_FIRA,
    structs::recruits::RecruitStats,
};
use bevy::{prelude::*, ui::widget::NodeImageMode};

#[derive(Component)]
pub struct SelectRecruitForMissionTrigger(pub RecruitStats);

pub fn map_recruit_card(
    left_container: &mut ChildBuilder,
    my_assets: &Res<AssetServer>,
    recruit: &RecruitStats,
    recruit_texture_atlas_layout: Handle<TextureAtlasLayout>,
) {
    let (recruit_attack, recruit_defense) = recruit.get_total_stats();

    left_container
        .spawn((
            Name::new("Map recruit button"),
            Button,
            ImageNode {
                image: my_assets.load("images/rooms/command_room/recruit_card_4.png"),
                image_mode: NodeImageMode::Stretch,
                ..default()
            },
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                column_gap: Val::Px(12.),
                width: Val::Percent(100.),
                height: Val::Px(65.),
                padding: UiRect {
                    top: Val::Px(3.),
                    bottom: Val::Px(3.),
                    left: Val::Px(7.),
                    right: Val::Px(3.),
                },
                border: UiRect::all(Val::Px(2.0)),
                flex_shrink: 0.,
                ..default()
            },
            BorderColor(ColorPaletteEnum::DarkBrown.as_color()),
            BorderRadius::all(Val::Px(7.)),
            GlobalZIndex(4),
            UpdateMapRecruitListChildrenTrigger,
            SelectRecruitForMissionTrigger(recruit.clone()),
            PickingBehavior {
                should_block_lower: false,
                ..default()
            },
        ))
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
                            justify_content: JustifyContent::FlexStart,
                            padding: UiRect {
                                left: Val::Px(42.),
                                ..default()
                            },
                            ..default()
                        },
                        BorderRadius::all(Val::Px(10.)),
                        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
                        PickingBehavior {
                            should_block_lower: false,
                            ..default()
                        },
                        GlobalZIndex(5),
                    ))
                    .with_children(|overlay| {
                        overlay.spawn((
                            Text::new(t!(recruit.state.get_description())),
                            TextFont {
                                font: my_assets.load(FONT_FIRA),
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                            PickingBehavior {
                                should_block_lower: false,
                                ..default()
                            },
                        ));
                    });
            }
        })
        .with_children(|button| {
            button
                .spawn((
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(5.),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        width: Val::Px(30.),
                        ..default()
                    },
                    PickingBehavior {
                        should_block_lower: false,
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(recruit_attack.to_string()),
                        TextFont {
                            font: my_assets.load(FONT_FIRA),
                            font_size: 16.0,
                            ..default()
                        },
                        TextLayout {
                            justify: JustifyText::Center,
                            ..default()
                        },
                        TextColor(Color::BLACK),
                        PickingBehavior {
                            should_block_lower: false,
                            ..default()
                        },
                    ));

                    parent.spawn((
                        Text::new(recruit_defense.to_string()),
                        TextFont {
                            font: my_assets.load(FONT_FIRA),
                            font_size: 16.0,
                            ..default()
                        },
                        TextLayout {
                            justify: JustifyText::Center,
                            ..default()
                        },
                        TextColor(Color::BLACK),
                        PickingBehavior {
                            should_block_lower: false,
                            ..default()
                        },
                    ));
                });

            // Recruit name
            button
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(5.),
                        left: Val::Px(47.),
                        width: Val::Percent(45.),
                        height: Val::Percent(100.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        overflow: Overflow {
                            x: OverflowAxis::Hidden,
                            y: OverflowAxis::Hidden,
                        },
                        ..default()
                    },
                    PickingBehavior {
                        should_block_lower: false,
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(t!(&recruit.name)),
                        TextFont {
                            font: my_assets.load(FONT_FIRA),
                            font_size: 12.0,
                            ..default()
                        },
                        TextLayout {
                            justify: JustifyText::Center,
                            linebreak: LineBreak::NoWrap,
                        },
                        Node {
                            overflow: Overflow {
                                x: OverflowAxis::Clip,
                                ..default()
                            },
                            ..default()
                        },
                        TextColor(Color::BLACK),
                        PickingBehavior {
                            should_block_lower: false,
                            ..default()
                        },
                    ));

                    parent.spawn((
                        Text::new(t!(recruit.class.to_string())),
                        TextFont {
                            font: my_assets.load(FONT_FIRA),
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(Color::BLACK),
                        PickingBehavior {
                            should_block_lower: false,
                            ..default()
                        },
                    ));
                });

            // Recruit picture
            button
                .spawn((
                    Node {
                        width: Val::Px(72.),
                        position_type: PositionType::Absolute,
                        right: Val::Px(0.),
                        height: Val::Percent(100.),
                        overflow: Overflow {
                            x: OverflowAxis::Hidden,
                            y: OverflowAxis::Hidden,
                        },
                        ..default()
                    },
                    PickingBehavior {
                        should_block_lower: false,
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ImageNode::from_atlas_image(
                            my_assets.load("images/recruits/recruit_picture_atlas.png"),
                            TextureAtlas {
                                index: recruit.image_atlas_index.into(),
                                layout: recruit_texture_atlas_layout.clone(),
                            },
                        ),
                        Node {
                            position_type: PositionType::Absolute,
                            right: Val::Px(0.),
                            width: Val::Percent(100.),
                            height: Val::Px(65. * 2.),
                            ..default()
                        },
                        GlobalZIndex(3),
                        PickingBehavior {
                            should_block_lower: false,
                            ..default()
                        },
                    ));
                });
        });
}
