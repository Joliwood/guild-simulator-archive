use crate::{
    custom_components::get_mission_position,
    enums::ColorPaletteEnum,
    my_assets::{get_mission_image, FONT_FIRA},
    structs::missions::Mission,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct MissionIconTrigger;

#[derive(Component)]
pub struct MissionIconUnavailableTrigger(pub u16);

#[derive(Component)]
pub struct SelectMissionTrigger;

pub fn mission_icon(my_assets: &Res<AssetServer>, parent: &mut ChildBuilder, mission: &Mission) {
    let mission_position = get_mission_position(mission.id);
    let mission_image_path = get_mission_image(mission.id);

    parent
        .spawn((
            Button,
            ImageNode {
                image: my_assets.load("images/missions/mission_icon.png"),
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Px(130.),
                height: Val::Px(130.),
                left: Val::Px(mission_position.0),
                top: Val::Px(mission_position.1),
                ..default()
            },
            SelectMissionTrigger,
            mission.clone(),
            GlobalZIndex(2),
            MissionIconTrigger,
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageNode {
                    image: my_assets.load(&mission_image_path),
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Px(90.),
                    height: Val::Px(90.),
                    top: Val::Px(15.),
                    ..default()
                },
                BorderRadius::MAX,
                GlobalZIndex(1),
            ));

            // Mission name
            parent.spawn((
                Text::new(t!(&mission.name).to_string()),
                TextFont {
                    font: my_assets.load(FONT_FIRA),
                    font_size: 11.0,
                    ..default()
                },
                TextLayout {
                    justify: JustifyText::Center,
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Px(130.),
                    top: Val::Px(6.3),
                    ..default()
                },
                TextColor(Color::BLACK),
                GlobalZIndex(3),
            ));

            // Attack of the mission ennemy
            parent.spawn((
                Text::new(mission.ennemy.attack.to_string()),
                TextFont {
                    font: my_assets.load(FONT_FIRA),
                    font_size: 11.0,
                    ..default()
                },
                TextLayout {
                    justify: JustifyText::Center,
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(25.),
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    bottom: Val::Px(10.),
                    right: Val::Px(67.5),

                    ..default()
                },
                TextColor(ColorPaletteEnum::SunnyLight.as_color()),
                GlobalZIndex(3),
            ));

            // Defense of the mission ennemy
            parent.spawn((
                Text::new(mission.ennemy.defense.to_string()),
                TextFont {
                    font: my_assets.load(FONT_FIRA),
                    font_size: 11.0,
                    ..default()
                },
                TextLayout {
                    justify: JustifyText::Center,
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    display: Display::Flex,
                    width: Val::Px(25.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    bottom: Val::Px(10.),
                    left: Val::Px(65.),
                    ..default()
                },
                TextColor(ColorPaletteEnum::SunnyLight.as_color()),
                GlobalZIndex(3),
            ));

            // For mission if recruit is already sent (triggered by an updater)
            parent
                .spawn((
                    ImageNode {
                        image: my_assets.load(mission_image_path),
                        color: Color::srgba(0.0, 0.0, 0.0, 0.9),
                        ..default()
                    },
                    Node {
                        position_type: PositionType::Absolute,
                        display: Display::None,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(90.),
                        height: Val::Px(90.),
                        top: Val::Px(15.),
                        ..default()
                    },
                    BorderRadius::MAX,
                    GlobalZIndex(1),
                    MissionIconUnavailableTrigger(mission.id),
                ))
                .with_children(|overlay| {
                    overlay.spawn((
                        Text::new(t!("recruit_already_sent")),
                        TextFont {
                            font: my_assets.load(FONT_FIRA),
                            font_size: 11.0,
                            ..default()
                        },
                        TextLayout {
                            justify: JustifyText::Center,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}
