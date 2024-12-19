use crate::{
    enums::{ColorPaletteEnum, TextureAtlasLayoutEnum},
    my_assets::FONT_FIRA,
    structs::maps::Map,
    utils::get_layout,
};
use bevy::{prelude::*, ui::widget::NodeImageMode};

#[derive(Component)]
pub struct SelectMapTrigger;

pub fn map_card(
    column: &mut ChildBuilder,
    my_assets: &Res<AssetServer>,
    map: &Map,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let map_layout = get_layout(TextureAtlasLayoutEnum::Map);
    let map_atlas_layout: Handle<TextureAtlasLayout> = texture_atlas_layouts.add(map_layout);
    let icon_atlas_index = map.map_type.get_icon_atlas_index();

    let map_type_layout = get_layout(TextureAtlasLayoutEnum::MapType);
    let map_type_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(map_type_layout);

    let missions_finished_number = map.get_finished_missions_number();

    column
        .spawn((
            Button,
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                height: Val::Px(70.0),
                border: UiRect::all(Val::Px(2.0)),
                flex_shrink: 0.,
                ..default()
            },
            GlobalZIndex(2),
            BorderColor(ColorPaletteEnum::DarkBrown.as_color()),
            BorderRadius::all(Val::Px(5.)),
            ImageNode {
                image: my_assets.load("images/maps/map_card.png"),
                image_mode: NodeImageMode::Stretch,
                ..default()
            },
            SelectMapTrigger,
            PickingBehavior {
                should_block_lower: false,
                ..default()
            },
        ))
        .with_children(|map_container| {
            // Map ImageNode
            map_container.spawn((
                ImageNode::from_atlas_image(
                    my_assets.load("images/maps/map_atlas.png"),
                    TextureAtlas {
                        index: map.image_atlas_index.into(),
                        layout: map_atlas_layout.clone(),
                    },
                ),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(3.),
                    bottom: Val::Px(10.),
                    left: Val::Px(3.),
                    height: Val::Px(70. - 6.),
                    aspect_ratio: Some(270. / 200.),
                    ..default()
                },
                GlobalZIndex(1),
                PickingBehavior {
                    should_block_lower: false,
                    ..default()
                },
            ));

            // Map Name (Top-Right)
            map_container.spawn((
                Text::new(t!(&map.name)),
                TextFont {
                    font: my_assets.load(FONT_FIRA),
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::BLACK),
                Node {
                    position_type: PositionType::Absolute,
                    align_self: AlignSelf::FlexEnd,
                    top: Val::Px(3.),
                    right: Val::Px(7.),
                    ..default()
                },
                PickingBehavior {
                    should_block_lower: false,
                    ..default()
                },
            ));

            // Bottom-Right Container with 2 Icons
            map_container
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(5.),
                        right: Val::Px(10.),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexEnd,
                        row_gap: Val::Px(8.),
                        ..default()
                    },
                    PickingBehavior {
                        should_block_lower: false,
                        ..default()
                    },
                ))
                .with_children(|icon_container| {
                    icon_container.spawn((
                        Text::new(t!(map.recommanded_power_level.to_string())),
                        TextFont {
                            font: my_assets.load(FONT_FIRA),
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::BLACK),
                        PickingBehavior {
                            should_block_lower: false,
                            ..default()
                        },
                    ));

                    if map.limited_in_time {
                        icon_container.spawn((
                            ImageNode {
                                image: my_assets.load("images/maps/limited_time.png"),
                                ..default()
                            },
                            Node {
                                width: Val::Px(18.),
                                height: Val::Px(18.),
                                ..default()
                            },
                            PickingBehavior {
                                should_block_lower: false,
                                ..default()
                            },
                        ));
                    } else {
                        let missions_finished_text =
                            format!("{}/{}", missions_finished_number, map.map_mission_ids.len());
                        icon_container.spawn((
                            Text::new(t!(missions_finished_text)),
                            TextFont {
                                font: my_assets.load(FONT_FIRA),
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::BLACK),
                            PickingBehavior {
                                should_block_lower: false,
                                ..default()
                            },
                        ));
                    };
                });

            // Center text
            map_container.spawn((
                ImageNode::from_atlas_image(
                    my_assets.load("images/maps/map_type_atlas.png"),
                    TextureAtlas {
                        index: icon_atlas_index.into(),
                        layout: map_type_atlas_layout.clone(),
                    },
                ),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(88.),
                    bottom: Val::Px(6.),
                    width: Val::Px(40.0),
                    height: Val::Px(40.0),
                    ..default()
                },
                PickingBehavior {
                    should_block_lower: false,
                    ..default()
                },
            ));
        });
}
