use crate::my_assets::FONT_FIRA;
use crate::structs::missions::MissionReports;
use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct MissionReportTrigger;

#[derive(Component)]
pub struct MissionReportTextTrigger;

pub fn mission_report_documents(
    my_assets: &Res<AssetServer>,
    elements_on_desk: &mut ChildBuilder,
    mission_reports: &Res<MissionReports>,
) {
    let mission_reports_number = mission_reports.0.len();

    elements_on_desk
        .spawn((
            Button,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(250.),
                top: Val::Px(170.),
                ..default()
            },
            MissionReportTrigger,
        ))
        .with_children(|mission_report_button| {
            mission_report_button
                .spawn((
                    ImageNode {
                        image: my_assets
                            .load("images/rooms/office/mission_notification_document.png"),
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
                                Text::new(format!("{}", mission_reports_number)),
                                TextFont {
                                    font: my_assets.load(FONT_FIRA),
                                    font_size: 25.0,
                                    ..default()
                                },
                                TextColor(Color::BLACK),
                                MissionReportTextTrigger,
                            ));
                        });
                });
        });
}
