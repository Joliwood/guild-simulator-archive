#![allow(dead_code)]

use crate::{
    enums::{ClassEnum, RecruitStateEnum},
    structs::recruits::{RecruitInventory, RecruitStats},
};
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RecruitEnum {
    JeanLouisDavid,
    Hubert,
    BigNoob,
}

impl RecruitEnum {
    pub fn get_recruit(&self) -> RecruitStats {
        match self {
            RecruitEnum::JeanLouisDavid => RecruitStats {
                class: ClassEnum::Warrior,
                experience: 0,
                id: Uuid::new_v4(),
                image_atlas_index: 0,
                level: 1,
                max_experience: 100,
                name: "Jean-Louis-David".to_string(),
                recruit_inventory: RecruitInventory::default(),
                state: RecruitStateEnum::Available,
                attack: 7,
                defense: 4,
            },
            RecruitEnum::Hubert => RecruitStats {
                class: ClassEnum::Mage,
                experience: 0,
                id: Uuid::new_v4(),
                image_atlas_index: 1,
                level: 1,
                max_experience: 100,
                name: "Hubert".to_string(),
                // recruit_inventory: RecruitInventory {
                //     weapon: Some(WeaponsEnum::MagicToothpick.get_weapon()),
                //     armor: Some(ArmorsEnum::LeatherTunic.get_armor()),
                //     scrolls: vec![],
                // },
                recruit_inventory: RecruitInventory::default(),
                state: RecruitStateEnum::Available,
                attack: 8,
                defense: 2,
            },
            RecruitEnum::BigNoob => RecruitStats {
                class: ClassEnum::Rogue,
                experience: 0,
                id: Uuid::new_v4(),
                image_atlas_index: 3,
                level: 1,
                max_experience: 100,
                name: "Big noob".to_string(),
                recruit_inventory: RecruitInventory::default(),
                state: RecruitStateEnum::Available,
                attack: 5,
                defense: 3,
            },
        }
    }
}
