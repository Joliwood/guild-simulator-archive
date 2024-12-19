use crate::{
    enums::TextureAtlasLayoutEnum,
    my_assets::get_item_atlas_path,
    structs::{
        equipments::ItemEnum, general_structs::ItemInInventoryTrigger, recruits::RecruitStats,
    },
    utils::{get_item_image_atlas_index, get_item_tooltip_description, get_layout},
};
use bevy::prelude::*;
use pyri_tooltip::{Tooltip, TooltipActivation};

pub fn armor_button(
    top_container: &mut ChildBuilder,
    my_assets: &Res<AssetServer>,
    recruit_stats: &RecruitStats,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let recruit_stats_inventory = recruit_stats.recruit_inventory.clone();
    let recruit_stats_armor = recruit_stats_inventory.armor;

    if let Some(recruit_stats_armor) = recruit_stats_armor {
        let item = ItemEnum::Armor(recruit_stats_armor);
        let item_image_atlas_index = get_item_image_atlas_index(&item);
        let item_layout = get_layout(TextureAtlasLayoutEnum::Item(&item));
        let tooltip_text = get_item_tooltip_description(&item);
        let item_atlas_path = get_item_atlas_path(&item);

        top_container
            .spawn((
                Button,
                Node {
                    width: Val::Px(40.),
                    height: Val::Px(40.),
                    border: UiRect::all(Val::Px(3.)),
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
                ItemInInventoryTrigger(None),
                Tooltip::cursor(t!(tooltip_text).to_string())
                    .with_activation(TooltipActivation::IMMEDIATE),
            ))
            .insert(PickingBehavior {
                should_block_lower: false,
                ..default()
            });
    } else {
        // Empty armor button
        top_container
            .spawn((
                Button,
                Node {
                    width: Val::Px(40.),
                    height: Val::Px(40.),
                    border: UiRect::all(Val::Px(3.)),
                    ..default()
                },
                BorderColor(Color::BLACK),
                BorderRadius::all(Val::Px(10.)),
                ImageNode {
                    image: my_assets.load("images/equipments/empty_inventory_slot.png"),
                    ..default()
                },
                ItemInInventoryTrigger(None),
            ))
            .insert(PickingBehavior {
                should_block_lower: false,
                ..default()
            });
    }
}
