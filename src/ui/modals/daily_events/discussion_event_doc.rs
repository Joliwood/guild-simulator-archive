use crate::{
    enums::TextureAtlasLayoutEnum, my_assets::FONT_FIRA,
    structs::daily_events_folder::discussions::DailyDiscussion, utils::get_layout,
};
use bevy::prelude::*;

use super::spontaneous_application_event_doc::SelectAnswerTrigger;

pub fn discussion_event_doc(
    commands: &mut Commands,
    my_assets: &Res<AssetServer>,
    discussion: DailyDiscussion,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let daily_discussions_layout = get_layout(TextureAtlasLayoutEnum::Discussion);
    let daily_discussions_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(daily_discussions_layout);

    commands
        .spawn((
            Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            GlobalZIndex(2),
            SelectAnswerTrigger,
        ))
        .with_children(|commands| {
            commands
                .spawn((
                    ImageNode {
                        image: my_assets.load("images/rooms/office/daily_event_document.png"),
                        ..default()
                    },
                    Node {
                        display: Display::Flex,
                        align_self: AlignSelf::Center,
                        justify_self: JustifySelf::Center,
                        width: Val::Px(330.),
                        height: Val::Px(500.),
                        padding: UiRect {
                            left: Val::Px(20.),
                            right: Val::Px(20.),
                            top: Val::Px(20.),
                            bottom: Val::Px(20.),
                        },
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    GlobalZIndex(3),
                    Name::new("Daily event document"),
                ))
                .with_children(|parent| {
                    // Container with flex column layout
                    parent
                        .spawn(Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::FlexStart,
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            ..default()
                        })
                        .with_children(|column| {
                            // Title at the top
                            column.spawn((
                                Text::new(t!(&discussion.title)),
                                TextFont {
                                    font: my_assets.load(FONT_FIRA),
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextLayout {
                                    linebreak: LineBreak::NoWrap,
                                    ..Default::default()
                                },
                                TextColor(Color::BLACK),
                                Node {
                                    margin: UiRect::bottom(Val::Px(8.)),
                                    ..default()
                                },
                            ));

                            // ImageNode below the title
                            column.spawn((
                                ImageNode::from_atlas_image(
                                    my_assets
                                        .load("images/daily_events/daily_discussions_atlas.png"),
                                    TextureAtlas {
                                        index: discussion.image_atlas_index.into(),
                                        layout: daily_discussions_texture_atlas_layout.clone(),
                                    },
                                ),
                                Node {
                                    width: Val::Percent(100.),
                                    height: Val::Px(150.),
                                    margin: UiRect::bottom(Val::Px(8.)),
                                    ..default()
                                },
                                GlobalZIndex(2),
                            ));

                            // Description below the image
                            column.spawn((
                                Text::new(t!(&discussion.description)),
                                TextFont {
                                    font: my_assets.load(FONT_FIRA),
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(Color::BLACK),
                                Node {
                                    margin: UiRect::bottom(Val::Px(12.)),
                                    ..default()
                                },
                            ));

                            // Map through answers and display each below the description
                            for answer in discussion.answers.iter() {
                                column
                                    .spawn((
                                        Button,
                                        Node {
                                            width: Val::Percent(100.0),
                                            margin: UiRect::top(Val::Px(4.0)),
                                            padding: UiRect::all(Val::Px(8.0)),
                                            border: UiRect::all(Val::Px(1.)),
                                            ..default()
                                        },
                                        BorderRadius::all(Val::Px(5.)),
                                        BackgroundColor(Color::srgba(0., 0., 0., 0.7)),
                                    ))
                                    .insert(answer.clone())
                                    .insert(discussion.clone())
                                    .with_children(|button| {
                                        button.spawn((
                                            Text::new(t!(&answer.message).to_string()),
                                            TextFont {
                                                font: my_assets.load(FONT_FIRA),
                                                font_size: 12.0,
                                                ..default()
                                            },
                                            TextColor(Color::BLACK),
                                        ));
                                    });
                            }
                        });
                });
        });
}
