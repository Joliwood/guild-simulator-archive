use super::{
    map_description::map_description, map_list::map_list, map_on_table::map_on_table,
    map_recruit_list::map_recruit_list,
};
use crate::{
    enums::RoomEnum,
    structs::{
        general_structs::RoomTag,
        maps::{Maps, SelectedMapId},
        missions::Missions,
        player_stats::PlayerStats,
    },
    systems::updates::init_rooms::ResetRoomTrigger,
};
use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn room_command_room(
    my_assets: &Res<AssetServer>,
    commands: &mut Commands,
    missions: Res<Missions>,
    selected_map_id: Res<SelectedMapId>,
    maps: Res<Maps>,
    player_stats: &Res<PlayerStats>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let selected_map = maps.get_map_by_optional_id(selected_map_id.0);

    commands
        .spawn((
            Name::new("Command room"),
            Node {
                width: Val::Vw(100.),
                height: Val::Vh(100.),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            RoomTag(RoomEnum::CommandRoom),
            ResetRoomTrigger,
        ))
        // Background ImageNode for the Command Room
        .with_children(|ui_container: &mut ChildBuilder| {
            ui_container.spawn((
                ImageNode {
                    image: my_assets.load("images/rooms/command_room/command_room_background.png"),
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    // justify_content: JustifyContent::Center,
                    // align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                GlobalZIndex(-1),
            ));
        })
        // Command Table with all child elements inside it
        .with_children(|ui_container: &mut ChildBuilder| {
            ui_container
                .spawn((
                    ImageNode {
                        image: my_assets.load("images/rooms/command_room/command_table.png"),
                        ..default()
                    },
                    Node {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(10.0),
                        width: Val::Px(1170.0),
                        height: Val::Px(600.0),
                        padding: UiRect {
                            left: Val::Px(50.0),
                            right: Val::Px(50.0),
                            top: Val::Px(50.0),
                            bottom: Val::Px(35.0),
                        },
                        ..default()
                    },
                    GlobalZIndex(0),
                ))
                .with_children(|table| {
                    // Left Column
                    table
                        .spawn(Node {
                            position_type: PositionType::Relative,
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::FlexStart,
                            width: Val::Percent(20.),
                            height: Val::Percent(100.),
                            overflow: Overflow {
                                x: OverflowAxis::Hidden,
                                y: OverflowAxis::Hidden,
                            },
                            ..default()
                        })
                        .with_children(|left_column| {
                            map_list(left_column, my_assets, &maps, texture_atlas_layouts);
                            map_description(left_column, my_assets, &selected_map);
                        });

                    // Center Area (Big node)
                    table
                        .spawn(Node {
                            width: Val::Percent(65.0),
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            padding: UiRect {
                                bottom: Val::Px(20.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|center| {
                            // External function for the center area
                            map_on_table(center, my_assets, &selected_map, &missions);
                        });

                    // Right Column
                    table
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::FlexEnd,
                            width: Val::Percent(20.0),
                            height: Val::Percent(100.0),
                            ..default()
                        })
                        .with_children(|right_column| {
                            map_recruit_list(
                                right_column,
                                my_assets,
                                player_stats,
                                texture_atlas_layouts,
                            );
                        });
                });
        });
}
