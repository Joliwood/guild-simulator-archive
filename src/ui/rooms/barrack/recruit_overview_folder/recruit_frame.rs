use crate::{
    enums::TextureAtlasLayoutEnum, my_assets::FONT_FIRA,
    structs::recruits::SelectedRecruitForEquipment, utils::get_layout,
};
use bevy::prelude::*;

use super::recruit_overview::RecruitOverviewChildTrigger;

#[derive(Component)]
pub struct RecruitFrameTrigger;

#[derive(Component)]
pub struct RecruitFrameTextTrigger;

pub fn recruit_frame(
    parent: &mut ChildBuilder,
    my_assets: &Res<AssetServer>,
    selected_recruit_for_equipment: &Res<SelectedRecruitForEquipment>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let recruit_layout = get_layout(TextureAtlasLayoutEnum::Recruit);
    let recruit_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(recruit_layout);

    let selected_recruit_for_equipment =
        selected_recruit_for_equipment.get_selected_recruit_for_equipment();

    let selected_recruit_for_equipment_name =
        if let Some(selected_recruit_for_equipment) = &selected_recruit_for_equipment {
            selected_recruit_for_equipment.name.clone()
        } else {
            "".to_string()
        };

    let selected_recruit_for_equipment_image_atlas_index =
        if let Some(selected_recruit_for_equipment) = selected_recruit_for_equipment {
            selected_recruit_for_equipment.image_atlas_index
        } else {
            // Empty frame in the atlas
            4
        };

    parent
        .spawn((
            Name::new("Barrack > recruit overview > recruit frame"),
            ImageNode {
                image: my_assets.load("images/rooms/barrack/recruit_frame.png"),
                ..default()
            },
            Node {
                width: Val::Px(180.),
                height: Val::Px(330.),
                ..default()
            },
            GlobalZIndex(2),
            RecruitOverviewChildTrigger,
        ))
        .with_children(|parent| {
            // if let Some(selected_recruit_for_equipment_data) = selected_recruit_for_equipment {
            parent.spawn((
                Text::new(selected_recruit_for_equipment_name),
                TextFont {
                    font: my_assets.load(FONT_FIRA),
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::BLACK),
                Node {
                    position_type: PositionType::Absolute,
                    margin: UiRect {
                        top: Val::Px(20.0),
                        left: Val::Auto,
                        right: Val::Auto,
                        ..default()
                    },
                    ..Default::default()
                },
                GlobalZIndex(3),
                RecruitFrameTextTrigger,
            ));

            parent.spawn((
                ImageNode::from_atlas_image(
                    my_assets.load("images/recruits/recruit_picture_atlas.png"),
                    TextureAtlas {
                        layout: recruit_texture_atlas_layout.clone(),
                        index: selected_recruit_for_equipment_image_atlas_index.into(),
                    },
                ),
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                GlobalZIndex(1),
                RecruitFrameTrigger,
            ));
            // }
        });
}
