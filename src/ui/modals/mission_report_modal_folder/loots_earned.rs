use crate::{
    enums::TextureAtlasLayoutEnum,
    my_assets::{get_item_atlas_path, FONT_FIRA},
    structs::missions::MissionReport,
    utils::get_layout,
};
use bevy::prelude::*;
use pyri_tooltip::{Tooltip, TooltipActivation};

pub fn loots_earned(
    parent: &mut ChildBuilder,
    my_assets: &Res<AssetServer>,
    golds_gained: i32,
    experience_gained: u32,
    mission_report: &MissionReport,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    parent
        .spawn(Node {
            width: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            ..default()
        })
        .with_children(|parent| {
            // Loots Text
            parent.spawn((
                Text::new(t!("loots")),
                TextFont {
                    font: my_assets.load(FONT_FIRA),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));

            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexStart,
                    justify_content: JustifyContent::Center,
                    ..default()
                })
                .with_children(|parent| {
                    // Loots in text
                    parent.spawn((
                        Text::new(format!(
                            "+ {} {} | + {} XP {}",
                            golds_gained,
                            t!("golds"),
                            experience_gained,
                            t!("for_recruit")
                        )),
                        TextFont {
                            font: my_assets.load(FONT_FIRA),
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::BLACK),
                    ));
                });

            // Loots in display row
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                })
                .with_children(|parent| {
                    for loot in mission_report.loots.iter() {
                        let item_image_atlas_index = loot.get_atlas_index();
                        let item_layout = get_layout(TextureAtlasLayoutEnum::Item(loot));
                        let tooltip_text = loot.get_item_loot_tooltip_description();
                        let item_atlas_path = get_item_atlas_path(loot);
                        parent.spawn((
                            Button,
                            Node {
                                width: Val::Px(50.0),
                                height: Val::Px(50.0),
                                border: UiRect::all(Val::Px(3.)),
                                margin: UiRect::all(Val::Px(5.)),
                                ..default()
                            },
                            BorderColor(Color::BLACK),
                            BorderRadius::all(Val::Px(10.)),
                            ImageNode::from_atlas_image(
                                my_assets.load(item_atlas_path),
                                TextureAtlas {
                                    index: item_image_atlas_index.into(),
                                    layout: texture_atlas_layouts.add(item_layout),
                                },
                            ),
                            Tooltip::cursor(t!(tooltip_text).to_string())
                                .with_activation(TooltipActivation::IMMEDIATE),
                        ));
                    }
                });
        });
}
