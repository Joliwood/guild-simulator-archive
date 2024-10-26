#![allow(dead_code)]
use crate::enums::MapImageEnum;
use bevy::prelude::*;

#[derive(Debug, Component, Resource, Clone)]
pub struct Maps(pub Vec<Map>);

impl Maps {
    pub fn get_map_by_optional_id(&self, id: Option<u16>) -> Option<Map> {
        if let Some(id) = id {
            return self.0.iter().find(|map| map.id == id).cloned();
        }
        None
    }

    pub fn get_uuid_of_tuto_map(&self) -> Option<u16> {
        self.0
            .iter()
            .find(|map| map.name == "Campagn tuto")
            .map(|map| map.id)
    }

    pub fn finish_mission_by_id(&mut self, mission_id: u16) {
        for map in self.0.iter_mut() {
            map.finish_mission_by_id(mission_id);
        }
    }
}

#[derive(Debug, Clone)]
pub enum MapTypeEnum {
    Campaign,
    BossMission,
    HelpAssistance,
    CombatMission,
}

impl MapTypeEnum {
    pub fn get_icon_atlas_index(&self) -> u16 {
        match self {
            MapTypeEnum::BossMission => 1,
            MapTypeEnum::Campaign => 3,
            MapTypeEnum::CombatMission => 2,
            MapTypeEnum::HelpAssistance => 0,
        }
    }
}

#[derive(Debug, Component, Resource, Clone)]
pub struct Map {
    pub description: String,
    pub id: u16,
    pub image_atlas_index: u16,
    pub image: MapImageEnum,
    pub limited_in_time: bool,
    pub map_mission_ids: Vec<u16>,
    pub map_type: MapTypeEnum,
    pub name: String,
    pub recommanded_power_level: u16,
    pub unlocked: bool,
    pub mission_ids_finished: Vec<u16>,
}

impl Map {
    pub fn finish_mission_by_id(&mut self, mission_id: u16) {
        if !self.mission_ids_finished.contains(&mission_id) {
            self.mission_ids_finished.push(mission_id);
        }
    }

    pub fn get_finished_missions_number(&self) -> usize {
        self.mission_ids_finished.len()
    }
}

// let first_map_uuid: Uuid = Uuid::new_v4();

#[derive(Debug, Component, Resource, Clone)]
pub struct SelectedMapId(pub Option<u16>);

impl Default for SelectedMapId {
    fn default() -> Self {
        Self(Some(1))
    }
}

impl Default for Maps {
    fn default() -> Self {
        Self(vec![
            Map {
                description: "Map gived by the mayor, marking vagrant camps causing trouble. Taking out their leader could make the town safer.".to_string(),
                id: 1,
                image_atlas_index: 0,
                image: MapImageEnum::CampagnTuto,
                limited_in_time: false,
                map_mission_ids: vec![1, 2, 3, 4, 5, 6],
                map_type: MapTypeEnum::Campaign,
                mission_ids_finished: vec![],
                name: "Troublemaker's Area".to_string(),
                recommanded_power_level: 25,
                unlocked: true,
            },
            Map {
                description: "Campaign 1 description".to_string(),
                id: 2,
                image_atlas_index: 1,
                image: MapImageEnum::CampagnTuto,
                limited_in_time: true,
                map_mission_ids: vec![],
                map_type: MapTypeEnum::BossMission,
                mission_ids_finished: vec![],
                name: "Campaign 2".to_string(),
                recommanded_power_level: 40,
                unlocked: true,
            },
            Map {
                description: "Campaign 2 description".to_string(),
                id: 3,
                image_atlas_index: 1,
                image: MapImageEnum::CampagnTuto,
                limited_in_time: false,
                map_mission_ids: vec![],
                map_type: MapTypeEnum::Campaign,
                mission_ids_finished: vec![],
                name: "Campaign 2".to_string(),
                recommanded_power_level: 40,
                unlocked: false,
            },
        ])
    }
}
