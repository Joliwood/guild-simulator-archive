use super::build_inventory_grid::build_inventory_grid;
use crate::structs::{player_stats::PlayerStats, recruits::SelectedRecruitForEquipment};
use bevy::prelude::*;

#[derive(Component)]
pub struct SpawnInventoryTrigger;

#[derive(Component)]
pub struct SpawnInventoryParentTrigger;

pub fn spawn_inventory(
    parent: &mut ChildBuilder,
    player_stats: &Res<PlayerStats>,
    my_assets: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    _selected_recruit_for_equipment: &Res<SelectedRecruitForEquipment>,
) {
    parent
        .spawn((
            Node {
                flex_wrap: FlexWrap::Wrap,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                width: Val::Percent(100.0),
                height: Val::Px(350.),
                padding: UiRect {
                    left: Val::Px(10.),
                    ..default()
                },
                overflow: Overflow::clip(),
                ..default()
            },
            SpawnInventoryParentTrigger,
        ))
        .with_children(|grid| {
            build_inventory_grid(grid, player_stats, my_assets, texture_atlas_layouts);
        });
}
