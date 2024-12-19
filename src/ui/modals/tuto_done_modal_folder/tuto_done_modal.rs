use crate::{
    my_assets::FONT_FIRA,
    structs::{general_structs::TutoDoneModalVisible, player_stats::PlayerStats},
    ui::modals::tuto_messages::tuto_message_modal::TutoDoneCloseModalTrigger,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct CloseTutoDoneMessageTrigger;

pub fn tuto_done_modal(
    mut commands: Commands,
    my_assets: Res<AssetServer>,
    query: Query<Entity, With<TutoDoneCloseModalTrigger>>,
    tuto_done_modal_visibility: Res<TutoDoneModalVisible>,
    player_stats: Res<PlayerStats>,
) {
    // Only run if the resource has changed
    if !tuto_done_modal_visibility.is_changed() {
        return;
    }

    // Despawn existing modals
    if !tuto_done_modal_visibility.0 {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        return;
    }

    // Check player tutorial status before spawning
    if player_stats.tuto.is_tuto_completed {
        return;
    }

    commands
        .spawn((
            Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            TutoDoneCloseModalTrigger,
            GlobalZIndex(7),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ImageNode {
                        image: my_assets.load("images/tuto/tuto_message_document.png"),
                        ..default()
                    },
                    Node {
                        display: Display::Flex,
                        width: Val::Px(600.),
                        height: Val::Px(370.),
                        margin: UiRect {
                            right: Val::Px(70.),
                            bottom: Val::Px(50.),
                            ..default()
                        },
                        padding: UiRect {
                            left: Val::Px(20.),
                            right: Val::Px(20.),
                            top: Val::Px(20.),
                            bottom: Val::Px(20.),
                        },
                        ..default()
                    },
                    GlobalZIndex(9),
                ))
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        Text::new(t!("tuto_done_title")),
                        TextFont {
                            font: my_assets.load(FONT_FIRA),
                            font_size: 14.0,
                            ..default()
                        },
                        TextLayout {
                            justify: JustifyText::Center,
                            ..default()
                        },
                        TextColor(Color::BLACK),
                        Node {
                            position_type: PositionType::Absolute,
                            top: Val::Px(65.),
                            right: Val::Px(100.),
                            width: Val::Px(220.),
                            ..default()
                        },
                    ));

                    // All the texts
                    parent
                        .spawn(Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(10.),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            position_type: PositionType::Absolute,
                            top: Val::Px(130.),
                            left: Val::Px(130.),
                            width: Val::Px(400.),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new(t!("tuto_done_desc_1")),
                                TextFont {
                                    font: my_assets.load(FONT_FIRA),
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(Color::BLACK),
                            ));

                            parent.spawn((
                                Text::new(t!("tuto_done_desc_2", days = player_stats.day)),
                                TextFont {
                                    font: my_assets.load(FONT_FIRA),
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(Color::BLACK),
                            ));
                        });

                    parent.spawn((
                        Button,
                        Text::new(t!("tuto_message_accept")),
                        TextFont {
                            font: my_assets.load(FONT_FIRA),
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        TextLayout {
                            justify: JustifyText::Center,
                            ..default()
                        },
                        Node {
                            position_type: PositionType::Absolute,
                            align_self: AlignSelf::Center,
                            bottom: Val::Px(36.),
                            right: Val::Px(32.),
                            width: Val::Px(110.),
                            ..default()
                        },
                        CloseTutoDoneMessageTrigger,
                    ));

                    // Avatar of the mayor
                    parent.spawn((
                        ImageNode {
                            image: my_assets.load("images/tuto/creator_avatar.png"),
                            ..default()
                        },
                        Node {
                            position_type: PositionType::Absolute,
                            top: Val::Px(10.),
                            left: Val::Px(10.),
                            width: Val::Px(130.),
                            height: Val::Px(130.),
                            overflow: Overflow {
                                x: OverflowAxis::Clip,
                                y: OverflowAxis::Clip,
                            },
                            ..default()
                        },
                        BorderRadius::MAX,
                        GlobalZIndex(8),
                    ));
                });
        });
    // }
}
