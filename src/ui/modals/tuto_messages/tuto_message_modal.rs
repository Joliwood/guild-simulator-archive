use crate::{
    my_assets::FONT_FIRA,
    structs::{
        general_structs::TutoMessagesModalVisible,
        player_stats::{TutoMessage, TutoMessages},
    },
};
use bevy::prelude::*;

#[derive(Component)]
pub struct TutoMessageModalTrigger;

#[derive(Component)]
pub struct TutoDoneCloseModalTrigger;

#[derive(Component)]
pub struct CloseTutoMessageTrigger;

#[derive(Component)]
pub struct SkipTutoMessageTrigger;

pub fn tuto_message_modal(
    mut commands: Commands,
    my_assets: Res<AssetServer>,
    query: Query<Entity, With<TutoMessageModalTrigger>>,
    tuto_messages_modal_visibility: Res<TutoMessagesModalVisible>,
    tuto_messages: ResMut<TutoMessages>,
) {
    // Despawn existing modals if visibility is set to false
    if tuto_messages_modal_visibility.is_changed() {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    let tuto_messages_len = tuto_messages.0.len();

    // Spawn the mission report modal if visibility is true and there are mission reports
    if tuto_messages_modal_visibility.is_changed()
        && tuto_messages_modal_visibility.0
        && tuto_messages_len > 0
    {
        let first_tuto_message: &TutoMessage = match tuto_messages.get_first_tuto_message() {
            Some(tuto_messages) => tuto_messages,
            None => return,
        };

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
                TutoMessageModalTrigger,
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
                            Text::new(&first_tuto_message.title),
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
                                for tuto_message_desc in first_tuto_message.messages.iter() {
                                    parent.spawn((
                                        Text::new(tuto_message_desc.to_string()),
                                        TextFont {
                                            font: my_assets.load(FONT_FIRA),
                                            font_size: 12.0,
                                            ..default()
                                        },
                                        TextColor(Color::BLACK),
                                    ));
                                }
                            });

                        parent
                            .spawn((
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
                                CloseTutoMessageTrigger,
                            ))
                            .insert(first_tuto_message.clone());

                        // Avatar of the mayor
                        parent.spawn((
                            ImageNode {
                                image: my_assets.load("images/tuto/mayor_avatar.png"),
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

                        // Button with text inside "Skip the tutorial"
                        parent.spawn((
                            Button,
                            Text::new(t!("tuto_message_skip")),
                            TextFont {
                                font: my_assets.load(FONT_FIRA),
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::BLACK),
                            Node {
                                position_type: PositionType::Absolute,
                                bottom: Val::Px(36.),
                                left: Val::Px(140.),
                                width: Val::Px(150.),
                                ..default()
                            },
                            SkipTutoMessageTrigger,
                        ));
                    });
            });
    }
}
