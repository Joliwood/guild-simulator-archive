use crate::{my_assets::FONT_FIRA, structs::missions::Mission};
use bevy::prelude::*;

pub fn mission_recap(
    parent: &mut ChildBuilder,
    my_assets: &Res<AssetServer>,
    mission: &Mission,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let ennemy_layout = TextureAtlasLayout::from_grid(
        UVec2::new(200, 200),
        6,
        1,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    );
    let ennemy_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(ennemy_layout);

    // Mission Image and Description (A / B)
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                justify_content: JustifyContent::FlexStart,
                width: Val::Percent(50.0),
                overflow: Overflow {
                    x: OverflowAxis::Hidden,
                    y: OverflowAxis::Hidden,
                },
                border: UiRect::all(Val::Px(2.)),
                padding: UiRect {
                    top: Val::Px(10.),
                    left: Val::Px(10.),
                    right: Val::Px(10.),
                    bottom: Val::Px(10.),
                },
                ..default()
            },
            BorderColor(Color::BLACK),
            BorderRadius::all(Val::Px(10.)),
        ))
        .with_children(|parent| {
            // Mission image
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(100.0),
                    overflow: Overflow {
                        x: OverflowAxis::Hidden,
                        y: OverflowAxis::Hidden,
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        UiImage::from_atlas_image(
                            my_assets.load("images/missions/ennemy_picture_atlas.png"),
                            TextureAtlas {
                                index: mission.ennemy.image_atlas_index.into(),
                                layout: ennemy_texture_atlas_layout.clone(),
                            },
                        ),
                        Node {
                            margin: UiRect {
                                top: Val::Px(-100.0),
                                ..default()
                            },
                            width: Val::Percent(100.0),
                            height: Val::Px(450.),
                            ..default()
                        },
                    ));
                });

            // Mission description
            parent.spawn((
                Text::new(mission.description.clone()),
                TextFont {
                    font: my_assets.load(FONT_FIRA),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));

            parent
                // Outer container to hold both rows (Name/Level and Stats)
                .spawn(Node {
                    flex_direction: FlexDirection::Column, // Organize rows in a column
                    justify_content: JustifyContent::FlexStart,
                    row_gap: Val::Px(3.),
                    width: Val::Percent(100.0), // Full width
                    ..default()
                })
                .with_children(|parent| {
                    // Row 1: Name (left) and Level (right)
                    parent.spawn((
                        Text::new(format!("Target : {}", mission.ennemy.name)),
                        TextFont {
                            font: my_assets.load(FONT_FIRA),
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::BLACK),
                    ));

                    // Enemy Level
                    parent.spawn((
                        Text::new(format!("Level : {}", mission.ennemy.level)),
                        TextFont {
                            font: my_assets.load(FONT_FIRA),
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::BLACK),
                    ));

                    // Enemy Strength
                    parent.spawn((
                        Text::new(format!("Str : {}", mission.ennemy.strength)),
                        TextFont {
                            font: my_assets.load(FONT_FIRA),
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::BLACK),
                    ));

                    // Enemy Defense
                    parent.spawn((
                        Text::new(format!("Def : {}", mission.ennemy.endurance)),
                        TextFont {
                            font: my_assets.load(FONT_FIRA),
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::BLACK),
                    ));

                    // Enemy Intelligence
                    parent.spawn((
                        Text::new(format!("Int : {}", mission.ennemy.intelligence)),
                        TextFont {
                            font: my_assets.load(FONT_FIRA),
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::BLACK),
                    ));
                });
        });
}
