use super::spawn_inventory::spawn_inventory;
use crate::{
    enums::ColorPaletteEnum,
    my_assets::FONT_FIRA,
    structs::{player_stats::PlayerStats, recruits::SelectedRecruitForEquipment},
};
use bevy::prelude::*;

pub fn spawn_right_container(
    parent: &mut ChildBuilder,
    my_assets: &Res<AssetServer>,
    player_stats: &Res<PlayerStats>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    selected_recruit_for_equipment: &Res<SelectedRecruitForEquipment>,
) {
    // Container for the inventory
    parent
        .spawn((
            Name::new("Room barrack > inventory"),
            ImageNode {
                image: my_assets.load("images/rooms/barrack/inventory_container.png"),
                ..default()
            },
            Node {
                display: Display::Flex,
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(10.0),
                align_items: AlignItems::Center,
                width: Val::Px(400.),
                height: Val::Px(450.),
                padding: UiRect::all(Val::Px(15.0)),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(t!("inventory")),
                TextFont {
                    font: my_assets.load(FONT_FIRA),
                    font_size: 26.,
                    ..default()
                },
                TextColor(ColorPaletteEnum::DarkBrown.as_color()),
            ));
            // Spawn the inventory below the buttons
            spawn_inventory(
                parent,
                player_stats,
                my_assets,
                texture_atlas_layouts,
                selected_recruit_for_equipment,
            );
        });
}
