use super::map_card::map_card;
use crate::structs::maps::Maps;
use bevy::prelude::*;

pub fn map_list(
    parent: &mut ChildBuilder,
    my_assets: &Res<AssetServer>,
    maps: &Res<Maps>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    // Create a vertical container for the map list
    parent
        .spawn((
            Name::new("map_list"),
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                row_gap: Val::Px(5.0),
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                height: Val::Percent(57.),
                padding: UiRect {
                    left: Val::Px(1.),
                    right: Val::Px(5.),
                    ..default()
                },
                overflow: Overflow::scroll_y(),
                ..default()
            },
        ))
        .with_children(|column| {
            // Loop through each map and create an ImageNode with text overlay for each
            for map in maps.0.iter().filter(|map| map.unlocked) {
                map_card(column, my_assets, map, texture_atlas_layouts);
            }
        });
}
