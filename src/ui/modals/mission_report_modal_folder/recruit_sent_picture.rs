use crate::{
    enums::TextureAtlasLayoutEnum, my_assets::FONT_FIRA, structs::recruits::RecruitStats,
    utils::get_layout,
};
use bevy::prelude::*;

pub fn recruit_sent_picture(
    commands: &mut ChildBuilder,
    recruit_sent: &RecruitStats,
    my_assets: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let recruit_layout = get_layout(TextureAtlasLayoutEnum::Recruit);
    let recruit_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(recruit_layout);

    // Recruit Send Block
    commands
        .spawn(Node {
            display: Display::Flex,
            width: Val::Px(100.),
            height: Val::Px(200.),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|recruit_block| {
            recruit_block.spawn((
                Text::new(t!("recruit_sent")),
                TextFont {
                    font: my_assets.load(FONT_FIRA),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));

            recruit_block.spawn((
                ImageNode::from_atlas_image(
                    my_assets.load("images/recruits/recruit_picture_atlas.png"),
                    TextureAtlas {
                        index: recruit_sent.image_atlas_index.into(),
                        layout: recruit_texture_atlas_layout.clone(),
                    },
                ),
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                GlobalZIndex(4),
            ));
        });
}
