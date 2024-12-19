#![allow(dead_code)]

use super::equipments::{armors::ArmorsEnum, scrolls::ScrollsEnum, weapons::WeaponsEnum};
use crate::structs::{
    general_structs::Ennemy,
    missions::{ItemLoot, ItemLootEnum, Loots, Mission, Missions},
};

pub fn generate_all_missions() -> Missions {
    Missions(vec![
        Mission {
            days_left: None,
            days: 1,
            id: 1,
            level: 1,
            name: "mission1_name".to_string(),
            percent_of_victory: None,
            recruit_send: None,
            ennemy: Ennemy {
                image_atlas_index: 5,
                experience: 0,
                level: 1,
                name: "mission1_ennemy_name".to_string(),
                attack: 12,
                defense: 5,
            },
            unlocked: true,
            description: "mission1_desc".to_string(),
            golds: 5,
            loots: Loots(vec![
                ItemLoot {
                    item: ItemLootEnum::Weapon(WeaponsEnum::MagicToothpick),
                    percent: 50,
                },
                ItemLoot {
                    item: ItemLootEnum::Armor(ArmorsEnum::MakeshiftVest),
                    percent: 25,
                },
                ItemLoot {
                    item: ItemLootEnum::Scroll(ScrollsEnum::ScrollOfRawNaturalDefenseI),
                    percent: 35,
                },
            ]),
            unlock_mission_ids: vec![4],
        },
        Mission {
            days_left: None,
            days: 1,
            id: 2,
            level: 2,
            name: "mission2_name".to_string(),
            percent_of_victory: None,
            recruit_send: None,
            ennemy: Ennemy {
                image_atlas_index: 4,
                experience: 0,
                level: 1,
                name: "mission2_ennemy_name".to_string(),
                attack: 12,
                defense: 10,
            },
            unlocked: true,
            description: "mission2_desc".to_string(),
            golds: 10,
            loots: Loots(vec![
                ItemLoot {
                    item: ItemLootEnum::Weapon(WeaponsEnum::WalkingStick),
                    percent: 50,
                },
                ItemLoot {
                    item: ItemLootEnum::Armor(ArmorsEnum::UsedLeatherToga),
                    percent: 20,
                },
                ItemLoot {
                    item: ItemLootEnum::Scroll(ScrollsEnum::ScrollOfExperienceI),
                    percent: 40,
                },
            ]),
            unlock_mission_ids: vec![3],
        },
        Mission {
            days_left: Some(1),
            days: 2,
            id: 3,
            level: 3,
            name: "mission3_name".to_string(),
            percent_of_victory: None,
            recruit_send: None,
            ennemy: Ennemy {
                image_atlas_index: 3,
                experience: 0,
                level: 3,
                name: "mission3_ennemy_name".to_string(),
                attack: 15,
                defense: 15,
            },
            unlocked: false,
            description: "mission3_desc".to_string(),
            golds: 30,
            loots: Loots(vec![
                ItemLoot {
                    item: ItemLootEnum::Scroll(ScrollsEnum::ScrollOfRawAttackI),
                    percent: 60,
                },
                ItemLoot {
                    item: ItemLootEnum::Scroll(ScrollsEnum::ScrollOfReinforcementI),
                    percent: 15,
                },
                ItemLoot {
                    item: ItemLootEnum::Scroll(ScrollsEnum::ScrollOfTheOutstandingFighterI),
                    percent: 20,
                },
            ]),
            unlock_mission_ids: vec![],
        },
        Mission {
            days_left: None,
            days: 1,
            id: 4,
            level: 1,
            name: "mission4_name".to_string(),
            percent_of_victory: None,
            recruit_send: None,
            ennemy: Ennemy {
                image_atlas_index: 5,
                experience: 0,
                level: 1,
                name: "mission4_ennemy_name".to_string(),
                attack: 17,
                defense: 8,
            },
            unlocked: false,
            description: "mission4_desc".to_string(),
            golds: 12,
            loots: Loots(vec![
                ItemLoot {
                    item: ItemLootEnum::Weapon(WeaponsEnum::BowWithoutString),
                    percent: 50,
                },
                ItemLoot {
                    item: ItemLootEnum::Armor(ArmorsEnum::ApprenticeCoat),
                    percent: 50,
                },
                ItemLoot {
                    item: ItemLootEnum::Weapon(WeaponsEnum::UnsharpDagger),
                    percent: 50,
                },
            ]),
            unlock_mission_ids: vec![5],
        },
        Mission {
            days_left: None,
            days: 1,
            id: 5,
            level: 2,
            name: "mission5_name".to_string(),
            percent_of_victory: None,
            recruit_send: None,
            ennemy: Ennemy {
                image_atlas_index: 4,
                experience: 0,
                level: 2,
                name: "mission5_ennemy_name".to_string(),
                attack: 15,
                defense: 22,
            },
            unlocked: false,
            description: "mission5_desc".to_string(),
            golds: 15,
            loots: Loots(vec![
                ItemLoot {
                    item: ItemLootEnum::Weapon(WeaponsEnum::WoodenSword),
                    percent: 10,
                },
                ItemLoot {
                    item: ItemLootEnum::Weapon(WeaponsEnum::LumberjackAxe),
                    percent: 50,
                },
                ItemLoot {
                    item: ItemLootEnum::Scroll(ScrollsEnum::ScrollOfTheResearcherI),
                    percent: 10,
                },
                ItemLoot {
                    item: ItemLootEnum::Scroll(ScrollsEnum::ScrollOfGaladornFailedPower),
                    percent: 10,
                },
            ]),
            unlock_mission_ids: vec![6],
        },
        Mission {
            days_left: Some(1),
            days: 2,
            id: 6,
            level: 3,
            name: "mission6_name".to_string(),
            percent_of_victory: None,
            recruit_send: None,
            ennemy: Ennemy {
                image_atlas_index: 3,
                experience: 0,
                level: 3,
                name: "mission6_ennemy_name".to_string(),
                attack: 42,
                defense: 8,
            },
            unlocked: false,
            description: "mission6_desc".to_string(),
            golds: 50,
            loots: Loots(vec![
                ItemLoot {
                    item: ItemLootEnum::Armor(ArmorsEnum::RecycledMagicianRobe),
                    percent: 50,
                },
                ItemLoot {
                    item: ItemLootEnum::Armor(ArmorsEnum::LeatherTunic),
                    percent: 50,
                },
                ItemLoot {
                    item: ItemLootEnum::Scroll(ScrollsEnum::ScrollOfTheMiserI),
                    percent: 10,
                },
            ]),
            unlock_mission_ids: vec![],
        },
    ])
}
