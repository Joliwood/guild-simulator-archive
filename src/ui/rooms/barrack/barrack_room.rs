use super::{
    inventory::spawn_right_container::spawn_right_container,
    recruit_overview_folder::recruit_overview::recruit_overview,
    recruits_list_folder::recruits_list::spawn_left_container,
};
use crate::{
    enums::RoomEnum,
    structs::{
        general_structs::RoomTag, player_stats::PlayerStats, recruits::SelectedRecruitForEquipment,
    },
    systems::updates::init_rooms::ResetRoomTrigger,
};
use bevy::prelude::*;

pub fn spawn_room_barrack(
    my_assets: &Res<AssetServer>,
    commands: &mut Commands,
    player_stats: &Res<PlayerStats>,
    selected_recruit_for_equipment: &Res<SelectedRecruitForEquipment>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    commands
        .spawn((
            Node {
                width: Val::Vw(100.),
                height: Val::Vh(100.),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Stretch,
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            GlobalZIndex(-1),
            RoomTag(RoomEnum::Barrack),
        ))
        .insert(Name::new("Room barrack"))
        .insert(ResetRoomTrigger)
        .with_children(|parent| {
            parent.spawn((
                ImageNode {
                    image: my_assets.load("images/rooms/barrack/barrack_background.png"),
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.),
                    height: Val::Vh(100.),
                    ..default()
                },
            ));

            spawn_left_container(parent, my_assets, player_stats, texture_atlas_layouts);
            recruit_overview(
                player_stats,
                parent,
                my_assets,
                selected_recruit_for_equipment,
                texture_atlas_layouts,
            );
            spawn_right_container(
                parent,
                my_assets,
                player_stats,
                texture_atlas_layouts,
                selected_recruit_for_equipment,
            );
        });
}
