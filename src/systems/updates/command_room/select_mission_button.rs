use crate::{
    structs::{
        general_structs::MissionModalVisible,
        missions::{Mission, Missions, SelectedMission},
    },
    ui::rooms::command_room::mission_icon::SelectMissionTrigger,
};
use bevy::prelude::*;

pub fn select_mission_button(
    mut interaction_query: Query<
        (
            &Interaction,
            &SelectMissionTrigger,
            &Mission,
            &mut Transform,
        ),
        Changed<Interaction>,
    >,
    mut windows: Query<&mut Window>,
    missions: Res<Missions>,
    mut selected_mission: ResMut<SelectedMission>,
    mut mission_modal_visibility: ResMut<MissionModalVisible>,
) {
    let _window = windows.single_mut();
    if !mission_modal_visibility.0 {
        for (interaction, _trigger, mission, mut transform) in &mut interaction_query {
            if let Some(real_mission) = missions.get_mission_by_id(&mission.id) {
                if real_mission.recruit_send.is_some() {
                    continue;
                }
            }

            match *interaction {
                Interaction::Pressed => {
                    transform.scale = Vec3::splat(1.0);
                    selected_mission.mission_id = Some(mission.id);
                    mission_modal_visibility.0 = true;
                }
                Interaction::Hovered => {
                    transform.scale = Vec3::splat(1.05);
                }
                Interaction::None => {
                    transform.scale = Vec3::splat(1.0);
                }
            }
        }
    }
}
