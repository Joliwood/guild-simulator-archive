use crate::{
    enums::ColorPaletteEnum,
    my_assets::FONT_FIRA,
    structs::player_stats::{PlayerStats, TutoMessages},
};
use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct NotificationToastTrigger;

pub fn mayor_notification_toast(
    mut commands: Commands,
    my_assets: Res<AssetServer>,
    player_stats: Res<PlayerStats>,
    query: Query<Entity, With<NotificationToastTrigger>>,
    tuto_messages: Res<TutoMessages>,
) {
    let tuto_message_available = player_stats.tuto.count_tuto_available();

    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    if tuto_message_available > 0 && tuto_messages.get_first_tuto_message().is_some() {
        commands
            .spawn((
                Name::new("---> Notification toast"),
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(65.),
                    height: Val::Px(62.),
                    right: Val::Px(0.),
                    top: Val::Px(120.),
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    padding: UiRect {
                        left: Val::Px(10.),
                        right: Val::ZERO,
                        top: Val::ZERO,
                        bottom: Val::ZERO,
                    },
                    ..default()
                },
                BorderRadius {
                    top_left: Val::Px(10.),
                    top_right: Val::ZERO,
                    bottom_left: Val::Px(10.),
                    bottom_right: Val::ZERO,
                },
                ImageNode {
                    image: my_assets.load("images/tuto/mayor_notification_frame.png"),
                    ..default()
                },
                NotificationToastTrigger,
                GlobalZIndex(5),
            ))
            // .insert(first_tuto_message.clone())
            .with_children(|parent| {
                // Avatar of the mayor
                parent.spawn((
                    ImageNode {
                        image: my_assets.load("images/tuto/mayor_avatar.png"),
                        ..default()
                    },
                    Node {
                        position_type: PositionType::Absolute,
                        top: Val::Px(1.),
                        left: Val::Px(1.),
                        width: Val::Px(60.),
                        height: Val::Px(60.),
                        overflow: Overflow {
                            x: OverflowAxis::Clip,
                            y: OverflowAxis::Clip,
                        },
                        ..default()
                    },
                    BorderRadius::MAX,
                    GlobalZIndex(3),
                ));

                parent
                    .spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            top: Val::Px(0.),
                            right: Val::Px(8.),
                            width: Val::Px(16.),
                            height: Val::Px(16.),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BorderRadius::MAX,
                        BackgroundColor(ColorPaletteEnum::Danger.as_color()),
                    ))
                    .with_children(|indicator| {
                        indicator.spawn((
                            Text::new(tuto_message_available.to_string()),
                            TextFont {
                                font: my_assets.load(FONT_FIRA),
                                font_size: 10.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
            });
    }

    // }
}
