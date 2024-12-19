use crate::{
    enums::TextureAtlasLayoutEnum,
    my_assets::get_item_atlas_path,
    structs::{
        equipments::ItemEnum, general_structs::ItemInInventoryTrigger, player_stats::PlayerStats,
        recruits::RecruitStats,
    },
    utils::{get_item_image_atlas_index, get_item_tooltip_description, get_layout},
};
use bevy::prelude::*;
use pyri_tooltip::{Tooltip, TooltipActivation};

pub fn scroll_button(
    player_stats: &Res<PlayerStats>,
    scrolls_row: &mut ChildBuilder,
    my_assets: &Res<AssetServer>,
    recruit: &RecruitStats,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    scroll_index: u8,
) {
    let recruit_id = recruit.id;
    let recruit = player_stats.get_recruit_by_id(recruit_id).unwrap();
    let recruit_inventory = recruit.recruit_inventory;
    let recruit_scrolls = recruit_inventory.scrolls;
    let recruit_scroll = recruit_scrolls.get(scroll_index as usize);

    if let Some(recruit_scroll) = recruit_scroll {
        let item = ItemEnum::Scroll(recruit_scroll.clone(), 1);
        let item_image_atlas_index = get_item_image_atlas_index(&item);
        let item_layout = get_layout(TextureAtlasLayoutEnum::Item(&item));
        let tooltip_text = get_item_tooltip_description(&item);
        let item_atlas_path = get_item_atlas_path(&item);

        // Scroll button
        scrolls_row
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
        // Empty scroll button
        scrolls_row
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
