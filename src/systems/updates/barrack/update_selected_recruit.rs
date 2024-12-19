use crate::{
    structs::{player_stats::PlayerStats, recruits::SelectedRecruitForEquipment},
    ui::rooms::barrack::recruit_overview_folder::{
        recruit_frame::recruit_frame,
        recruit_infos_folder::recruit_infos::recruit_infos,
        recruit_overview::{RecruitOverviewChildTrigger, RecruitOverviewParentTrigger},
    },
};
use bevy::prelude::*;

pub fn update_selected_recruit(
    player_stats: Res<PlayerStats>,
    mut commands: Commands,
    selected_recruit_for_equipment: Res<SelectedRecruitForEquipment>,
    my_assets: Res<AssetServer>,
    parent_query: Query<Entity, With<RecruitOverviewParentTrigger>>,
    childs_query: Query<Entity, With<RecruitOverviewChildTrigger>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if let Some(parent_entity) = parent_query.iter().next() {
        for child in childs_query.iter() {
            commands.entity(child).despawn_recursive();
        }

        commands.entity(parent_entity).with_children(|parent| {
            recruit_frame(
                parent,
                &my_assets,
                &selected_recruit_for_equipment,
                &mut texture_atlas_layouts,
            );
            recruit_infos(
                parent,
                &my_assets,
                &selected_recruit_for_equipment,
                &mut texture_atlas_layouts,
                &player_stats,
            )
        });
    }
}
