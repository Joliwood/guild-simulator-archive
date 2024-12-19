use crate::{
    enums::TextureAtlasLayoutEnum,
    my_assets::FONT_FIRA,
    structs::{player_stats::PlayerStats, recruits::SelectedRecruitForMission},
    ui::rooms::barrack::recruits_list_folder::{
        armor_button::armor_button, scroll_button::scroll_button, weapon_button::weapon_button,
    },
    utils::get_layout,
};
use bevy::prelude::*;

pub fn recruit_recap(
    parent: &mut ChildBuilder,
    selected_recruit_for_mission: Res<SelectedRecruitForMission>,
    my_assets: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    player_stats: &Res<PlayerStats>,
) {
    let recruit_layout = get_layout(TextureAtlasLayoutEnum::Recruit);
    let recruit_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(recruit_layout);

    // Recruits (ImageNode / Name / Stats / Node Container)
    parent
        .spawn((
            Node {
                width: Val::Percent(40.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                row_gap: Val::Px(10.0),
                overflow: Overflow {
                    x: OverflowAxis::Hidden,
                    y: OverflowAxis::Hidden,
                },
                border: UiRect::all(Val::Px(2.)),
                padding: UiRect {
                    top: Val::Px(10.),
                    left: Val::Px(10.),
                    right: Val::Px(10.),
                    bottom: Val::Px(10.),
                },
                ..default()
            },
            BorderColor(Color::BLACK),
            BorderRadius::all(Val::Px(10.)),
        ))
        .with_children(|parent| {
            if selected_recruit_for_mission.0.is_none() {
                parent.spawn((
                    Text::new(t!("no_recruit_selected")),
                    TextFont {
                        font: my_assets.load(FONT_FIRA),
                        font_size: 14.0,
                        ..default()
                    },
                    Node {
                        border: UiRect::all(Val::Px(2.)),
                        margin: UiRect::all(Val::Auto),
                        ..default()
                    },
                    TextColor(Color::BLACK),
                ));
                return;
            } else {
                let recruit = selected_recruit_for_mission.clone().0.unwrap();

                // Recruit image
                parent
                    .spawn(Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(100.0),
                        overflow: Overflow {
                            x: OverflowAxis::Hidden,
                            y: OverflowAxis::Hidden,
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        // Recruit image
                        parent.spawn((
                            ImageNode::from_atlas_image(
                                my_assets.load("images/recruits/recruit_picture_atlas.png"),
                                TextureAtlas {
                                    index: recruit.image_atlas_index.into(),
                                    layout: recruit_texture_atlas_layout.clone(),
                                },
                            ),
                            Node {
                                width: Val::Percent(100.),
                                height: Val::Px(400.),
                                ..default()
                            },
                        ));
                    });

                parent
                    // Outer container to hold both rows (Name/Level and Stats)
                    .spawn(Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(5.),
                        width: Val::Percent(100.0),
                        ..default()
                    })
                    .with_children(|parent| {
                        // Recruit Name
                        parent.spawn((
                            Text::new(format!("{} : {}", t!("recruit"), recruit.name)),
                            TextFont {
                                font: my_assets.load(FONT_FIRA),
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::BLACK),
                        ));

                        // Recruit Level
                        parent.spawn((
                            Text::new(format!("{} : {}", t!("level"), recruit.level)),
                            TextFont {
                                font: my_assets.load(FONT_FIRA),
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::BLACK),
                        ));

                        // Recruit attack
                        parent.spawn((
                            Text::new(format!("ATT : {}", recruit.get_total_stats().0)),
                            TextFont {
                                font: my_assets.load(FONT_FIRA),
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::BLACK),
                        ));

                        // Recruit defense
                        parent.spawn((
                            Text::new(format!("DEF : {}", recruit.get_total_stats().1)),
                            TextFont {
                                font: my_assets.load(FONT_FIRA),
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::BLACK),
                        ));

                        // Equipments
                        parent
                            .spawn(Node {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(2.),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                margin: UiRect {
                                    top: Val::Px(10.),
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|equipments_container| {
                                // Top container for weapon and armor
                                equipments_container
                                    .spawn(Node {
                                        display: Display::Flex,
                                        column_gap: Val::Px(2.0),
                                        align_self: AlignSelf::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    })
                                    .with_children(|top_container| {
                                        weapon_button(
                                            top_container,
                                            my_assets,
                                            &recruit,
                                            texture_atlas_layouts,
                                        );

                                        armor_button(
                                            top_container,
                                            my_assets,
                                            &recruit,
                                            texture_atlas_layouts,
                                        );
                                    });

                                // Bottom container for scrolls
                                equipments_container
                                    .spawn(Node {
                                        display: Display::Flex,
                                        flex_direction: FlexDirection::Row,
                                        column_gap: Val::Px(2.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    })
                                    .with_children(|bottom_container| {
                                        scroll_button(
                                            player_stats,
                                            bottom_container,
                                            my_assets,
                                            &recruit,
                                            texture_atlas_layouts,
                                            0,
                                        );

                                        scroll_button(
                                            player_stats,
                                            bottom_container,
                                            my_assets,
                                            &recruit,
                                            texture_atlas_layouts,
                                            1,
                                        );

                                        scroll_button(
                                            player_stats,
                                            bottom_container,
                                            my_assets,
                                            &recruit,
                                            texture_atlas_layouts,
                                            2,
                                        );
                                    });
                            });
                    });
            }
        });
}
