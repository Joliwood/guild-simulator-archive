#![allow(dead_code)]

use crate::{
    enums::MapImageEnum,
    structs::maps::{Map, MapTypeEnum, Maps},
};

pub fn generate_all_maps() -> Maps {
    Maps(vec![
        Map {
            description: "campaign1_desc".to_string(),
            id: 1,
            image_atlas_index: 0,
            image: MapImageEnum::CampagnTuto,
            limited_in_time: false,
            map_mission_ids: vec![1, 2, 3, 4, 5, 6],
            map_type: MapTypeEnum::Campaign,
            mission_ids_finished: vec![],
            name: "campaign1_name".to_string(),
            recommanded_power_level: 25,
            unlocked: true,
        },
        Map {
            description: "campaign2_desc".to_string(),
            id: 2,
            image_atlas_index: 1,
            image: MapImageEnum::CampagnTuto,
            limited_in_time: true,
            map_mission_ids: vec![],
            map_type: MapTypeEnum::BossMission,
            mission_ids_finished: vec![],
            name: "campaign2_name".to_string(),
            recommanded_power_level: 40,
            unlocked: false,
        },
        Map {
            description: "campaign3_desc".to_string(),
            id: 3,
            image_atlas_index: 1,
            image: MapImageEnum::CampagnTuto,
            limited_in_time: false,
            map_mission_ids: vec![],
            map_type: MapTypeEnum::Campaign,
            mission_ids_finished: vec![],
            name: "campaign3_name".to_string(),
            recommanded_power_level: 40,
            unlocked: false,
        },
    ])
}
