use crate::{my_assets::FONT_FIRA, structs::player_stats::PlayerStats};
use bevy::prelude::*;
use pyri_tooltip::Tooltip;

#[derive(Resource, Component)]
pub struct GoldCountTrigger;

#[derive(Component)]
pub struct GuildLvlTrigger;

pub fn left_hud(
    commands: &mut ChildBuilder,
    my_assets: &Res<AssetServer>,
    player_stats: &Res<PlayerStats>,
    texture_atlas_layouts: &Handle<TextureAtlasLayout>,
) {
    commands
        .spawn((
            Name::new("HUD > Left Container"),
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                width: Val::Px(205.0),
                padding: UiRect {
                    right: Val::Px(60.),
                    ..default()
                },
                height: Val::Px(25.0),
                ..default()
            },
        ))
        .with_children(|left_container| {
            left_container
                .spawn((
                    Button,
                    Node {
                        display: Display::Flex,
                        column_gap: Val::Px(5.),
                        ..default()
                    },
                    Tooltip::cursor(t!("tooltip_hud_golds").to_string()),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ImageNode::from_atlas_image(
                            my_assets.load("images/hud/hud_icon_atlas.png"),
                            TextureAtlas {
                                index: 3,
                                layout: texture_atlas_layouts.clone(),
                            },
                        ),
                        Node {
                            width: Val::Px(16.),
                            height: Val::Px(16.),
                            ..default()
                        },
                    ));
                    // Adding 3 text elements
                    parent.spawn((
                        Text::new(player_stats.golds.to_string()),
                        TextFont {
                            font: my_assets.load(FONT_FIRA),
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        GoldCountTrigger,
                    ));
                });

            left_container.spawn((
                Text::new(format!("{} : {}", t!("day"), player_stats.guild_level)),
                TextFont {
                    font: my_assets.load(FONT_FIRA),
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                GuildLvlTrigger,
            ));
        });
}
