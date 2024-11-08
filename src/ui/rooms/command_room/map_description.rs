use crate::{my_assets::FONT_FIRA, structs::maps::Map};
use bevy::prelude::*;

pub fn map_description(
    parent: &mut ChildBuilder,
    my_assets: &Res<AssetServer>,
    selected_map: &Option<Map>,
) {
    parent
        .spawn((
            UiImage {
                image: my_assets.load("images/maps/map_description.png"),
                ..default()
            },
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                row_gap: Val::Px(10.),
                align_items: AlignItems::FlexStart,
                width: Val::Percent(100.),
                height: Val::Percent(35.),
                padding: UiRect {
                    left: Val::Px(25.),
                    right: Val::Px(25.),
                    top: Val::Px(20.),
                    bottom: Val::Px(20.),
                },
                ..default()
            },
        ))
        .with_children(|column| {
            if let Some(map) = selected_map {
                column.spawn((
                    Text::new(map.name.clone()),
                    TextFont {
                        font: my_assets.load(FONT_FIRA),
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::BLACK),
                ));

                column.spawn((
                    Text::new(map.description.clone()),
                    TextFont {
                        font: my_assets.load(FONT_FIRA),
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::BLACK),
                ));
            }
        });
}
