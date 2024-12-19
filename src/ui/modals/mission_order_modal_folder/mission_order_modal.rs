use super::{
    loots_and_start::loots_and_start, mission_recap::mission_recap, recruit_recap::recruit_recap,
};
use crate::{
    enums::{ColorPaletteEnum, TextureAtlasLayoutEnum},
    my_assets::FONT_FIRA,
    structs::{
        general_structs::MissionModalVisible,
        missions::{Missions, SelectedMission},
        player_stats::PlayerStats,
        recruits::SelectedRecruitForMission,
    },
    ui::ui_constants::WOOD_COLOR,
    utils::get_layout,
};
use bevy::{prelude::*, ui::widget::NodeImageMode};
use pyri_tooltip::Tooltip;

#[derive(Component)]
pub struct MissionModalContentTrigger;

#[derive(Component)]
pub struct CloseMissionModalTrigger;

#[allow(clippy::too_many_arguments)]
pub fn mission_order_modal(
    mut commands: Commands,
    my_assets: Res<AssetServer>,
    mission_modal_visibility: Res<MissionModalVisible>,
    query: Query<Entity, With<MissionModalContentTrigger>>,
    player_stats: Res<PlayerStats>,
    selected_mission: Res<SelectedMission>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    missions: Res<Missions>,
    selected_recruit_for_mission: Res<SelectedRecruitForMission>,
) {
    let buttons_layout = get_layout(TextureAtlasLayoutEnum::Button);
    let buttons_texture_atlas_layout = texture_atlas_layouts.add(buttons_layout);

    // Despawn existing modals
    if mission_modal_visibility.is_changed() && !mission_modal_visibility.0 {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    // Despawn and retrigger if the recruit is changed
    if (selected_recruit_for_mission.is_changed() || mission_modal_visibility.is_changed())
        && mission_modal_visibility.0
        && selected_mission.mission_id.is_some()
    {
        // Despawn existing modals when retriggering
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }

        if let Some(mission) = missions.get_mission_by_id(&selected_mission.mission_id.unwrap()) {
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
                    GlobalZIndex(3),
                    MissionModalContentTrigger,
                ))
                .with_children(|commands| {
                    commands
                        .spawn((
                            Name::new("Mission details modal"),
                            Node {
                                position_type: PositionType::Absolute,
                                align_items: AlignItems::Center,
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                row_gap: Val::Px(10.0),
                                width: Val::Px(570.0),
                                height: Val::Px(470.0),
                                margin: UiRect::all(Val::Auto),
                                padding: UiRect::all(Val::Px(10.0)),
                                border: UiRect::all(Val::Px(3.0)),
                                ..default()
                            },
                            BorderRadius::all(Val::Px(20.0)),
                            BorderColor(Color::BLACK),
                            BackgroundColor(WOOD_COLOR),
                            GlobalZIndex(4),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Button,
                                Node {
                                    position_type: PositionType::Absolute,
                                    right: Val::Px(5.),
                                    top: Val::Px(5.),
                                    width: Val::Px(30.),
                                    height: Val::Px(30.),
                                    border: UiRect::all(Val::Px(3.)),
                                    ..default()
                                },
                                BorderColor(WOOD_COLOR),
                                BorderRadius::all(Val::Px(10.)),
                                ImageNode::from_atlas_image(
                                    my_assets.load("images/hud/buttons_atlas.png"),
                                    TextureAtlas {
                                        index: 16,
                                        layout: buttons_texture_atlas_layout.clone(),
                                    },
                                )
                                .with_mode(NodeImageMode::Stretch),
                                CloseMissionModalTrigger,
                            ));

                            // Title
                            parent
                                .spawn((
                                    Text::new(t!(&mission.name)),
                                    TextFont {
                                        font: my_assets.load(FONT_FIRA),
                                        font_size: 18.0,
                                        ..default()
                                    },
                                    TextColor(Color::BLACK),
                                ))
                                .insert(Name::new("Mission details modal > title"));

                            // Main contents / loots
                            parent
                                .spawn(Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::SpaceBetween,
                                    ..default()
                                })
                                .with_children(|parent| {
                                    // Main contents : mission image and description, percent of win, recruit info, and button
                                    parent
                                        .spawn(Node {
                                            flex_direction: FlexDirection::Row,
                                            justify_content: JustifyContent::SpaceBetween,
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(80.),
                                            row_gap: Val::Px(20.0),
                                            ..default()
                                        })
                                        .with_children(|parent| {
                                            mission_recap(
                                                parent,
                                                &my_assets,
                                                &mission,
                                                &mut texture_atlas_layouts,
                                            );

                                            if let Some(percent_of_victory) =
                                                selected_mission.percent_of_victory
                                            {
                                                parent
                                                    .spawn((Node {
                                                        flex_direction: FlexDirection::Row,
                                                        align_items: AlignItems::Center,
                                                        ..default()
                                                    },))
                                                    .with_children(|parent| {
                                                        parent.spawn((
                                                            Text::new(format!(
                                                                "{}%",
                                                                percent_of_victory
                                                            )),
                                                            TextFont {
                                                                font: my_assets.load(FONT_FIRA),
                                                                font_size: 16.0,
                                                                ..default()
                                                            },
                                                            TextColor(Color::BLACK),
                                                        ));

                                                        if percent_of_victory < 50 {
                                                            parent.spawn((
                                                                Text::new("!"),
                                                                TextFont {
                                                                    font: my_assets.load(FONT_FIRA),
                                                                    font_size: 16.0,
                                                                    ..default()
                                                                },
                                                                TextColor(
                                                                    ColorPaletteEnum::Danger
                                                                        .as_color(),
                                                                ),
                                                                Node {
                                                                    margin: UiRect::left(Val::Px(
                                                                        5.0,
                                                                    )),
                                                                    ..default()
                                                                },
                                                                Tooltip::cursor(
                                                                    t!("risky_mission_warning")
                                                                        .to_string(),
                                                                ),
                                                            ));
                                                        }
                                                    });
                                            }

                                            recruit_recap(
                                                parent,
                                                selected_recruit_for_mission,
                                                &my_assets,
                                                &mut texture_atlas_layouts,
                                                &player_stats,
                                            );
                                        });

                                    loots_and_start(
                                        parent,
                                        &my_assets,
                                        &missions,
                                        &selected_mission,
                                        &mut texture_atlas_layouts,
                                    );
                                });
                        });
                });
        }
    }
}
