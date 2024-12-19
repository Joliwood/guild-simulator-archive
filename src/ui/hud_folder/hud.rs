use super::{left_hud::left_hud, right_hud::right_hud, sleep_button::sleep_button};
use crate::{
    custom_components::notification_count_indicator,
    enums::{RoomEnum, TextureAtlasLayoutEnum},
    structs::{
        general_structs::{DayTime, NotificationCount},
        player_stats::PlayerStats,
    },
    utils::get_layout,
};
use bevy::prelude::*;
use pyri_tooltip::Tooltip;

#[derive(Component)]
pub struct RoomButtonTrigger(pub RoomEnum);

pub fn hud(
    my_assets: Res<AssetServer>,
    mut commands: Commands,
    player_stats: Res<PlayerStats>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    _day_time: Res<DayTime>,
    notification_count: Res<NotificationCount>,
) {
    let hud_icons_layout = get_layout(TextureAtlasLayoutEnum::HudIcon);
    let hud_icons_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(hud_icons_layout);

    commands
        // Main Container
        .spawn((
            Name::new("HUD"),
            ImageNode {
                image: my_assets.load("images/hud/hud4.png"),
                ..default()
            },
            Node {
                width: Val::Percent(100.),
                height: Val::Px(50.),
                position_type: PositionType::Absolute,
                top: Val::Px(0.),
                justify_content: JustifyContent::SpaceBetween,
                display: Display::Flex,
                ..default()
            },
            GlobalZIndex(4),
        ))
        // Left Container
        .with_children(|parent| {
            left_hud(
                parent,
                &my_assets,
                &player_stats,
                &hud_icons_texture_atlas_layout,
            );

            // Middle Container
            parent
                .spawn((
                    Name::new("HUD > Middle Container"),
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        height: Val::Px(35.0),
                        width: Val::Px(165.0),
                        margin: UiRect {
                            right: Val::Px(2.),
                            ..default()
                        },
                        column_gap: Val::Px(7.),
                        ..default()
                    },
                ))
                .with_children(|middle_container| {
                    middle_container
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(30.),
                                height: Val::Px(30.),
                                position_type: PositionType::Relative,
                                ..default()
                            },
                            ImageNode::from_atlas_image(
                                my_assets.load("images/hud/hud_icon_atlas.png"),
                                TextureAtlas {
                                    index: 4,
                                    layout: hud_icons_texture_atlas_layout.clone(),
                                },
                            ),
                            Tooltip::cursor(t!("tooltip_command_room_indications").to_string()),
                        ))
                        .insert(RoomButtonTrigger(RoomEnum::CommandRoom))
                        .with_children(|parent| {
                            notification_count_indicator(
                                parent,
                                notification_count.command_room_count,
                                &my_assets,
                                RoomEnum::CommandRoom,
                            );
                        });

                    middle_container
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(30.0),
                                height: Val::Px(30.),
                                ..default()
                            },
                            ImageNode::from_atlas_image(
                                my_assets.load("images/hud/hud_icon_atlas.png"),
                                TextureAtlas {
                                    index: 1,
                                    layout: hud_icons_texture_atlas_layout.clone(),
                                },
                            ),
                            Tooltip::cursor(t!("tooltip_office_room_indications").to_string()),
                        ))
                        .insert(RoomButtonTrigger(RoomEnum::Office))
                        .with_children(|parent| {
                            notification_count_indicator(
                                parent,
                                notification_count.office_count,
                                &my_assets,
                                RoomEnum::Office,
                            );
                        });

                    middle_container
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(30.0),
                                height: Val::Px(30.),
                                ..default()
                            },
                            ImageNode::from_atlas_image(
                                my_assets.load("images/hud/hud_icon_atlas.png"),
                                TextureAtlas {
                                    index: 2,
                                    layout: hud_icons_texture_atlas_layout.clone(),
                                },
                            ),
                            Tooltip::cursor(t!("tooltip_barrack_room_indications").to_string()),
                        ))
                        .insert(RoomButtonTrigger(RoomEnum::Barrack))
                        .with_children(|parent| {
                            notification_count_indicator(
                                parent,
                                notification_count.barrack_count,
                                &my_assets,
                                RoomEnum::Barrack,
                            );
                        });
                });

            right_hud(
                parent,
                &my_assets,
                &player_stats,
                &hud_icons_texture_atlas_layout,
            );
        });

    sleep_button(
        &mut commands,
        &my_assets,
        &player_stats,
        &mut texture_atlas_layouts,
    );
}
