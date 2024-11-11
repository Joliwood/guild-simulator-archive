use crate::{
    custom_components::CustomButton,
    enums::TextureAtlasLayoutEnum,
    my_assets::{get_item_loot_atlas_path, FONT_FIRA},
    structs::{
        general_structs::UniqueId,
        missions::{Missions, SelectedMission},
    },
    utils::get_layout,
};
use bevy::prelude::*;
// use pyri_tooltip::{Tooltip, TooltipActivation};

pub fn loots_and_start(
    parent: &mut ChildBuilder,
    my_assets: &Res<AssetServer>,
    missions: &Res<Missions>,
    selected_mission: &Res<SelectedMission>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    if let Some(mission_id) = selected_mission.mission_id {
        let mission_loots = match missions.get_mission_by_id(&mission_id) {
            Some(mission) => mission.loots,
            None => {
                error!(
                    "The mission doesn't have any loots, for mission_id : {}",
                    mission_id
                );
                return;
            }
        };

        // Loots (Text / Loot Icons)
        parent
            .spawn(Node {
                height: Val::Percent(20.),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn(Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    })
                    .with_children(|parent| {
                        // Loots in text
                        parent.spawn((
                            Text::new("Loots :"),
                            TextFont {
                                font: my_assets.load(FONT_FIRA),
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::BLACK),
                        ));

                        // Loots in display row
                        parent
                            .spawn(Node {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                ..default()
                            })
                            .with_children(|parent| {
                                for loot in mission_loots.0.iter() {
                                    let item_image_atlas_index = loot.get_atlas_index();
                                    // let layout = loot.get_item_loot_layout();
                                    let layout =
                                        get_layout(TextureAtlasLayoutEnum::Loot(&loot.item));
                                    // let tooltip_text = loot.get_item_loot_tooltip_description();
                                    let item_loot_atlas_path = get_item_loot_atlas_path(&loot.item);
                                    parent.spawn((
                                        Button,
                                        Node {
                                            width: Val::Px(50.0),
                                            height: Val::Px(50.0),
                                            border: UiRect::all(Val::Px(3.)),
                                            margin: UiRect::all(Val::Px(5.)),
                                            ..default()
                                        },
                                        BorderColor(Color::BLACK),
                                        BorderRadius::all(Val::Px(10.)),
                                        UiImage::from_atlas_image(
                                            my_assets.load(item_loot_atlas_path),
                                            TextureAtlas {
                                                index: item_image_atlas_index.into(),
                                                layout: texture_atlas_layouts.add(layout),
                                            },
                                        ),
                                        // Tooltip::cursor(tooltip_text.to_string())
                                        //     .with_activation(TooltipActivation::IMMEDIATE),
                                    ));
                                }
                            });
                    });

                // Button inside the middle container
                parent
                    .spawn(CustomButton::MissionStart.bundle(my_assets))
                    .with_children(|button| {
                        button.spawn((
                            Text::new("Start the mission"),
                            TextFont {
                                font: my_assets.load(FONT_FIRA),
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                            Node {
                                margin: UiRect::all(Val::Auto),
                                ..default()
                            },
                        ));
                    })
                    .insert(if selected_mission.recruit_id.is_some() {
                        UniqueId("start_mission".to_string())
                    } else {
                        UniqueId("".to_string())
                    });
            });
    }
}
