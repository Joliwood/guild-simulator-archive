use crate::my_assets::FONT_FIRA;
use crate::structs::daily_events_folder::daily_events::DailyEvents;
use bevy::prelude::*;

#[derive(Component)]
pub struct DailyEventTrigger;

#[derive(Component)]
pub struct DailyEventTextTrigger;

pub fn daily_event_documents(
    my_assets: &Res<AssetServer>,
    elements_on_desk: &mut ChildBuilder,
    daily_events: &Res<DailyEvents>,
) {
    let daily_events_number = daily_events.0.len();

    elements_on_desk
        .spawn((
            Name::new("Daily event documents"),
            Button,
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(250.),
                top: Val::Px(170.),
                ..default()
            },
            DailyEventTrigger,
        ))
        .with_children(|mission_report_button| {
            mission_report_button
                .spawn((
                    ImageNode {
                        image: my_assets
                            .load("images/rooms/office/daily_event_documents_on_desk.png"),
                        ..default()
                    },
                    Node {
                        display: Display::Flex,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Px(150.),
                        height: Val::Px(133. + 66.5),
                        ..default()
                    },
                ))
                .insert(Interaction::default())
                .with_children(|parent| {
                    parent
                        .spawn((
                            ImageNode {
                                image: my_assets
                                    .load("images/rooms/office/notification_token_in_wood.png"),
                                ..default()
                            },
                            Node {
                                display: Display::Flex,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                width: Val::Px(40.),
                                height: Val::Px(40.),
                                right: Val::Px(5.),
                                top: Val::Px(5.),
                                ..default()
                            },
                        ))
                        .with_children(|overlay| {
                            overlay.spawn((
                                Text::new(format!("{}", daily_events_number)),
                                TextFont {
                                    font: my_assets.load(FONT_FIRA),
                                    font_size: 25.0,
                                    ..default()
                                },
                                TextColor(Color::BLACK),
                                DailyEventTextTrigger,
                            ));
                        });
                });
        });
}
