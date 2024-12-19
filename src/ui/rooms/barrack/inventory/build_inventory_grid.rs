use super::spawn_inventory::SpawnInventoryTrigger;
use crate::{
    enums::TextureAtlasLayoutEnum,
    my_assets::{get_item_atlas_path, FONT_FIRA},
    structs::{
        equipments::ItemEnum, general_structs::ItemInInventoryTrigger, player_stats::PlayerStats,
    },
    utils::{get_item_image_atlas_index, get_item_tooltip_description, get_layout},
};
use bevy::{prelude::*, ui::widget::NodeImageMode};
use pyri_tooltip::{Tooltip, TooltipActivation};

pub fn build_inventory_grid(
    grid: &mut ChildBuilder,
    player_stats: &Res<PlayerStats>,
    my_assets: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    for i in 0..player_stats.max_inventory_size {
        // Check if there's an item in the player's inventory for this slot
        let item = if i < player_stats.inventory.len() {
            Some(&player_stats.inventory[i])
        } else {
            None
        };

        let tooltip_description = match item {
            Some(item) => get_item_tooltip_description(&item.clone()),
            None => "".to_string(),
        };

        grid.spawn((
            Button,
            Node {
                width: Val::Px(60.),
                height: Val::Px(60.),
                border: UiRect::all(Val::Px(3.)),
                margin: UiRect::all(Val::Px(5.)),
                ..default()
            },
            BorderColor(Color::BLACK),
            BorderRadius::all(Val::Px(10.)),
            SpawnInventoryTrigger,
            // Check if the item exists, if yes, show it, otherwise show the empty slot
            match item {
                Some(item) => ImageNode::from_atlas_image(
                    my_assets.load(get_item_atlas_path(item)),
                    TextureAtlas {
                        index: get_item_image_atlas_index(item).into(),
                        layout: texture_atlas_layouts
                            .add(get_layout(TextureAtlasLayoutEnum::Item(item))),
                    },
                )
                .with_mode(NodeImageMode::Stretch),
                None => ImageNode {
                    image: my_assets.load("images/equipments/empty_inventory_slot.png"),
                    ..Default::default()
                },
            },
            match item {
                Some(item) => ItemInInventoryTrigger(Some(item.clone())),
                None => ItemInInventoryTrigger(None),
            },
        ))
        .insert_if(
            Tooltip::cursor(tooltip_description).with_activation(TooltipActivation::IMMEDIATE),
            || item.is_some(),
        )
        .with_children(|button| {
            if let Some(ItemEnum::Scroll(_, count)) = item {
                button.spawn((
                    Text::new(format!("x{}", count)),
                    TextFont {
                        font: my_assets.load(FONT_FIRA),
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Node {
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(0.0),
                        right: Val::Px(5.0),
                        ..default()
                    },
                ));
            }
        });
    }
}
