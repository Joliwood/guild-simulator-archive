use super::{
    loots_earned::loots_earned, mission_ennemy_picture::mission_ennemy_picture,
    mission_ennemy_stats::mission_ennemy_stats, recruit_sent_picture::recruit_sent_picture,
    recruit_sent_stats::recruit_sent_stats as recruit_sent_stats_fn,
};
use crate::{
    enums::ColorPaletteEnum,
    my_assets::FONT_FIRA,
    structs::{
        general_structs::MissionReportsModalVisible,
        missions::{MissionReports, Missions},
        player_stats::PlayerStats,
    },
};
use bevy::prelude::*;

#[derive(Component)]
pub struct MissionReportModalContentTrigger;

#[derive(Component)]
pub struct MissionReportModalSignButtonTrigger;

#[allow(clippy::too_many_arguments)]
// Function to spawn the mission report modal
pub fn mission_report_modal(
    mut commands: Commands,
    my_assets: Res<AssetServer>,
    query: Query<Entity, With<MissionReportModalContentTrigger>>,
    mission_reports_modal_visibility: Res<MissionReportsModalVisible>,
    mission_reports: Res<MissionReports>,
    missions: Res<Missions>,
    player_stats: ResMut<PlayerStats>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Despawn existing modals if visibility is set to false
    if mission_reports_modal_visibility.is_changed() || !mission_reports_modal_visibility.0 {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    let mission_reports_len = mission_reports.0.len();

    // Spawn the mission report modal if visibility is true and there are mission reports
    if mission_reports_modal_visibility.is_changed()
        && mission_reports_modal_visibility.0
        && mission_reports_len > 0
    {
        let last_mission_report = match mission_reports.get_last_mission_report() {
            Some(report) => report,
            None => return,
        };

        let mission = match missions.get_mission_by_id(&last_mission_report.mission_id) {
            Some(mission) => mission,
            None => return,
        };

        // Prepare variables to display in the modal
        let success_message = if last_mission_report.success {
            "mission_is_a_success"
        } else {
            "mission_is_a_failure"
        };

        let recruit_sent_stats =
            match player_stats.get_recruit_by_id(last_mission_report.recruit_id) {
                Some(recruit) => recruit,
                None => return,
            };

        let recruit_xp_multiplicator = recruit_sent_stats
            .recruit_inventory
            .get_experience_multiplicator_from_scroll_bonus();

        let ennemy = mission.ennemy;
        let percent_of_victory = last_mission_report.percent_of_victory;
        let golds_gained = last_mission_report.golds_gained.unwrap_or(0);
        let experience_gained = (last_mission_report.experience_gained.unwrap_or(0) as f64
            * recruit_xp_multiplicator) as u32;

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
                MissionReportModalContentTrigger,
            ))
            .with_children(|commands| {
                // Spawn the mission report modal container
                commands
                    .spawn((
                        ImageNode {
                            image: my_assets.load("images/rooms/barrack/inventory_container.png"),
                            ..default()
                        },
                        Node {
                            width: Val::Px(300.),
                            height: Val::Px(450.),
                            display: Display::Flex,
                            justify_self: JustifySelf::Center,
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::FlexStart,
                            align_items: AlignItems::Center,
                            position_type: PositionType::Absolute,
                            padding: UiRect::all(Val::Px(10.)),
                            top: Val::Px(155.),
                            ..default()
                        },
                        GlobalZIndex(3),
                    ))
                    // .insert(MissionReportModalContentTrigger)
                    .with_children(|parent| {
                        // Title: "Report of the mission : name_mission"
                        parent.spawn((
                            Text::new(format!(
                                "{}: {}",
                                t!("mission_report_desc"),
                                t!(mission.name)
                            )),
                            TextFont {
                                font: my_assets.load(FONT_FIRA),
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::BLACK),
                        ));

                        // Mission Success or Failure Message
                        parent.spawn((
                            Text::new(t!(success_message)),
                            TextFont {
                                font: my_assets.load(FONT_FIRA),
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(if last_mission_report.success {
                                Color::srgb(0.0, 0.5, 0.0)
                            } else {
                                Color::srgb(0.5, 0.0, 0.0)
                            }),
                        ));

                        // Recruit Send / Versus / Enemy Block
                        parent
                            .spawn(Node {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceBetween,
                                width: Val::Percent(100.0),
                                ..default()
                            })
                            .with_children(|row| {
                                recruit_sent_picture(
                                    row,
                                    &recruit_sent_stats,
                                    &my_assets,
                                    &mut texture_atlas_layouts,
                                );

                                // Text "Versus"
                                row.spawn((
                                    Text::new("-- VS --"),
                                    TextFont {
                                        font: my_assets.load(FONT_FIRA),
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::BLACK),
                                ));

                                mission_ennemy_picture(
                                    row,
                                    &ennemy,
                                    &my_assets,
                                    &mut texture_atlas_layouts,
                                );
                            });

                        // Recruit Send Stats / Percent of Victory / Enemy Stats
                        parent
                            .spawn(Node {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceBetween,
                                width: Val::Percent(100.0),
                                ..default()
                            })
                            .with_children(|row| {
                                recruit_sent_stats_fn(row, &recruit_sent_stats, &my_assets);

                                // Percent of Victory
                                row.spawn((
                                    Text::new(format!("-- {}% --", percent_of_victory)),
                                    TextFont {
                                        font: my_assets.load(FONT_FIRA),
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::BLACK),
                                ));

                                mission_ennemy_stats(row, &ennemy, &my_assets);
                            });

                        loots_earned(
                            parent,
                            &my_assets,
                            golds_gained,
                            experience_gained,
                            &last_mission_report,
                            &mut texture_atlas_layouts,
                        );

                        // After the existing children have been added
                        parent
                            .spawn((
                                Button,
                                Node {
                                    position_type: PositionType::Absolute,
                                    bottom: Val::Px(10.0),
                                    right: Val::Px(10.0),
                                    width: Val::Px(120.0),
                                    height: Val::Px(40.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BorderColor(Color::BLACK),
                                BorderRadius::all(Val::Px(10.0)),
                                BackgroundColor(ColorPaletteEnum::DarkBrown.as_color()),
                            ))
                            .with_children(|button| {
                                button.spawn((
                                    Text::new(t!("sign_report")),
                                    TextFont {
                                        font: my_assets.load(FONT_FIRA),
                                        font_size: 12.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                            })
                            .insert(MissionReportModalSignButtonTrigger)
                            .insert(last_mission_report.clone());
                    });
            });
    }
}
