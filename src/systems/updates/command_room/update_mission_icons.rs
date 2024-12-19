use crate::{
    structs::missions::{Mission, Missions},
    ui::rooms::command_room::mission_icon::{MissionIconTrigger, MissionIconUnavailableTrigger},
};
use bevy::prelude::*;

pub fn update_mission_icons(
    missions: Res<Missions>,
    mut query: Query<(&mut Node, &mut MissionIconTrigger, &Mission)>,
) {
    for (mut node, _, mission) in query.iter_mut() {
        let is_mission_unlocked = missions.is_mission_unlocked_by_id(mission.id);

        if is_mission_unlocked {
            node.display = Display::Flex;
        } else {
            node.display = Display::None;
        }
    }
}

pub fn update_unavailable_mission_icons(
    missions: Res<Missions>,
    mut query: Query<(&mut Node, &mut MissionIconUnavailableTrigger)>,
) {
    for (mut node, trigger) in query.iter_mut() {
        let mission = missions.get_mission_by_id(&trigger.0);

        if let Some(mission) = mission {
            if mission.recruit_send.is_none() {
                node.display = Display::None;
            } else {
                node.display = Display::Flex;
            }
        }
    }
}
