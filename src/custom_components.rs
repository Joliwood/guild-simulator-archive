#![allow(dead_code)]

use crate::{
    enums::{ColorPaletteEnum, RoomEnum},
    my_assets::{get_mission_image, FONT_FIRA},
    ui::ui_constants::WOOD_COLOR,
};
use bevy::{prelude::*, ui::widget::NodeImageMode};

#[derive(Component)]
pub struct CommandRoomNotificationContainerTrigger;

#[derive(Component)]
pub struct CommandRoomNotificationTrigger;

#[derive(Component)]
pub struct OfficeRoomNotificationContainerTrigger;

#[derive(Component)]
pub struct OfficeRoomNotificationTrigger;

#[derive(Component)]
pub struct BarrackRoomNotificationContainerTrigger;

#[derive(Component)]
pub struct BarrackRoomNotificationTrigger;

pub enum CustomButton {
    Primary,
    GoldButton,
    RoomArrow,
    SquareIcon,
    EarnGold,
    MissionStart,
    MissionOnMap,
}

impl CustomButton {
    pub fn bundle(
        &self,
        my_assets: &Res<AssetServer>,
    ) -> (
        Button,
        Node,
        BorderRadius,
        BackgroundColor,
        ImageNode,
        BorderColor,
    ) {
        // the sprite sheet has 16 sprites arranged in a row, and they are all 500px x 500px

        match self {
            CustomButton::Primary => (
                Button,
                Node {
                    border: UiRect::all(Val::Px(5.)),
                    width: Val::Px(150.),
                    height: Val::Px(65.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderRadius::MAX,
                BackgroundColor(WOOD_COLOR),
                ImageNode::default(),
                BorderColor(Color::BLACK),
            ),
            CustomButton::GoldButton => (
                Button,
                Node {
                    margin: UiRect::all(Val::Px(10.)),
                    width: Val::Px(60.),
                    height: Val::Px(60.),
                    border: UiRect::all(Val::Px(5.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderRadius::MAX,
                BackgroundColor::DEFAULT,
                ImageNode::default(),
                BorderColor::DEFAULT,
            ),
            // image_assets
            CustomButton::RoomArrow => (
                Button,
                Node {
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.),
                    ..default()
                },
                BorderRadius::MAX,
                BackgroundColor::DEFAULT,
                ImageNode::default().with_color(Color::WHITE),
                BorderColor(Color::BLACK),
            ),
            CustomButton::SquareIcon => (
                Button,
                Node {
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    width: Val::Px(50.),
                    aspect_ratio: Some(1.),
                    ..default()
                },
                BorderRadius::all(Val::Px(10.)),
                BackgroundColor(WOOD_COLOR),
                ImageNode::default().with_color(Color::WHITE),
                BorderColor(Color::BLACK),
            ),
            CustomButton::EarnGold => (
                Button,
                Node {
                    margin: UiRect::all(Val::Px(10.)),
                    width: Val::Px(60.),
                    height: Val::Px(60.),
                    border: UiRect::all(Val::Px(3.)),
                    ..default()
                },
                BorderRadius::all(Val::Px(10.)),
                BackgroundColor::DEFAULT,
                // my_assets.buttons_atlas.clone().into(),
                my_assets.load("images/hud/buttons_atlas.png").into(),
                BorderColor(WOOD_COLOR),
            ),
            CustomButton::MissionStart => (
                Button,
                Node {
                    justify_self: JustifySelf::Center,
                    align_self: AlignSelf::Center,
                    width: Val::Px(200.),
                    height: Val::Px(40.),
                    border: UiRect::all(Val::Px(2.)),
                    ..default()
                },
                BorderRadius::all(Val::Px(10.)),
                BackgroundColor::DEFAULT,
                ImageNode {
                    image: my_assets.load("images/rooms/command_room/wood_box_container.png"),
                    image_mode: NodeImageMode::Stretch,
                    ..default()
                },
                BorderColor(Color::BLACK),
            ),
            _ => panic!("The mission button has to be called in the mission_bundle method"),
        }
    }

    pub fn mission_bundle(
        &self,
        my_assets: &Res<AssetServer>,
        mission_id: u16,
    ) -> (
        Button,
        Node,
        BorderRadius,
        BackgroundColor,
        ImageNode,
        BorderColor,
    ) {
        let mission_image = get_mission_image(mission_id);
        let mission_position = get_mission_position(mission_id);

        match self {
            CustomButton::MissionOnMap => (
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(mission_position.0),
                    top: Val::Px(mission_position.1),
                    justify_self: JustifySelf::Center,
                    align_self: AlignSelf::Center,
                    width: Val::Px(130.),
                    height: Val::Px(130.),
                    border: UiRect::all(Val::Px(2.)),
                    ..default()
                },
                BorderRadius::all(Val::Px(10.)),
                BackgroundColor::DEFAULT,
                my_assets.load(mission_image).into(),
                BorderColor(Color::BLACK),
            ),
            _ => panic!("This button is not a mission on map button"),
        }
    }
}

pub fn get_mission_position(mission_id: u16) -> (f32, f32) {
    match mission_id {
        1 => (432., 270.),
        2 => (236., 270.),
        3 => (45., 270.),
        4 => (432., 60.),
        5 => (236., 60.),
        6 => (45., 60.),
        _ => panic!("Mission id not found"),
    }
}

pub fn notification_count_indicator(
    parent: &mut ChildBuilder,
    notification_count: u8,
    my_assets: &Res<AssetServer>,
    room_enum: RoomEnum,
) {
    parent
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(-5.),
                right: Val::Px(-10.),
                width: Val::Px(16.),
                height: Val::Px(16.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderRadius::MAX,
            BackgroundColor(ColorPaletteEnum::Danger.as_color()),
        ))
        .insert_if(CommandRoomNotificationContainerTrigger, || {
            room_enum == RoomEnum::CommandRoom
        })
        .insert_if(OfficeRoomNotificationContainerTrigger, || {
            room_enum == RoomEnum::Office
        })
        .insert_if(BarrackRoomNotificationContainerTrigger, || {
            room_enum == RoomEnum::Barrack
        })
        .with_children(|indicator| {
            indicator
                .spawn((
                    Text::new(notification_count.to_string()),
                    TextFont {
                        font: my_assets.load(FONT_FIRA),
                        font_size: 10.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ))
                .insert_if(CommandRoomNotificationTrigger, || {
                    room_enum == RoomEnum::CommandRoom
                })
                .insert_if(OfficeRoomNotificationTrigger, || {
                    room_enum == RoomEnum::Office
                })
                .insert_if(BarrackRoomNotificationTrigger, || {
                    room_enum == RoomEnum::Barrack
                });
        });
}
