use crate::{
    enums::TextureAtlasLayoutEnum, my_assets::FONT_FIRA, structs::general_structs::Ennemy,
    utils::get_layout,
};
use bevy::prelude::*;

pub fn mission_ennemy_picture(
    commands: &mut ChildBuilder,
    ennemy: &Ennemy,
    my_assets: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let ennemy_layout = get_layout(TextureAtlasLayoutEnum::Ennemy);
    let ennemy_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(ennemy_layout);

    // Ennemy Block
    commands
        .spawn(Node {
            display: Display::Flex,
            width: Val::Px(100.),
            height: Val::Px(200.),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|ennemy_block| {
            ennemy_block.spawn((
                Text::new(t!("ennemy_sent")),
                TextFont {
                    font: my_assets.load(FONT_FIRA),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));

            ennemy_block.spawn((
                ImageNode::from_atlas_image(
                    my_assets.load("images/missions/ennemy_picture_atlas.png"),
                    TextureAtlas {
                        index: ennemy.image_atlas_index.into(),
                        layout: ennemy_texture_atlas_layout.clone(),
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
