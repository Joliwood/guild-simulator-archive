use super::mission_icon::mission_icon;
use crate::{
    enums::MapImageEnum,
    structs::{maps::Map, missions::Missions},
};
use bevy::prelude::*;

pub fn map_on_table(
    parent: &mut ChildBuilder,
    my_assets: &Res<AssetServer>,
    map: &Option<Map>,
    missions: &Res<Missions>,
) {
    parent
        .spawn(Node {
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            padding: UiRect::all(Val::Px(20.)),
            ..default()
        })
        .with_children(|parent| {
            if let Some(map) = map {
                let missions = missions.get_missions_by_ids(&map.map_mission_ids);
                parent
                    .spawn((
                        ImageNode {
                            image: my_assets.load(MapImageEnum::get_path(&map.image)),
                            ..default()
                        },
                        Node {
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            ..default()
                        },
                    ))
                    .with_children(|map| {
                        // Generate buttons for each mission
                        // for mission in missions.iter().filter(|mission| mission.unlocked) {
                        for mission in missions {
                            mission_icon(my_assets, map, &mission);
                        }
                    });
            }
        });
}
