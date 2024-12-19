#![allow(unused_imports)]
use super::{
    daily_event_documents::daily_event_documents,
    mission_report_documents::mission_report_documents,
};
use crate::enums::RoomEnum;
use crate::my_assets;
use crate::structs::general_structs::RoomTag;
use crate::structs::missions::MissionReports;
use crate::structs::{
    daily_events_folder::daily_events::DailyEvents, general_structs::MissionReportsModalVisible,
};
use crate::systems::updates::init_rooms::ResetRoomTrigger;
use bevy::{prelude::*, text::cosmic_text::Align};

pub fn room_office(
    my_assets: &Res<AssetServer>,
    commands: &mut Commands,
    mission_reports: &Res<MissionReports>,
    _mission_reports_modal_visibility: ResMut<MissionReportsModalVisible>,
    daily_events: &Res<DailyEvents>,
) {
    commands
        .spawn((
            Name::new("Office room"),
            ImageNode {
                image: my_assets.load("images/rooms/office/office_v4.png"),
                ..default()
            },
            Node {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Vw(100.),
                height: Val::Vh(100.),
                ..default()
            },
            GlobalZIndex(-1),
            RoomTag(RoomEnum::Office),
            ResetRoomTrigger,
        ))
        // Nested image background node
        .with_children(|desk_container: &mut ChildBuilder| {
            desk_container
                .spawn(Node {
                    display: Display::Flex,
                    align_self: AlignSelf::Center,
                    width: Val::Percent(90.0),
                    height: Val::Percent(80.0),
                    ..default()
                })
                // Adding child nodes with different positions
                .with_children(|elements_on_desk: &mut ChildBuilder| {
                    mission_report_documents(my_assets, elements_on_desk, mission_reports);
                    // recap_guild_scroll(&asset_server, elements_on_desk);
                    // talents_on_desk(&asset_server, elements_on_desk);
                    daily_event_documents(my_assets, elements_on_desk, daily_events);
                });
        });
}
