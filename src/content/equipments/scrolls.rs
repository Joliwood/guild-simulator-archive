#![allow(dead_code)]

use crate::structs::equipments::{BonusEnum, Scroll};

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ScrollsEnum {
    /// ## Integrated in :
    /// - Mission 3
    ScrollOfRawAttackI,

    /// ## Integrated in :
    /// - Mission 6
    ScrollOfTheMiserI,

    /// ## Integrated in :
    /// - Mission 5
    ScrollOfTheResearcherI,

    /// ## Integrated in :
    /// - Mission 2
    ScrollOfExperienceI,

    /// ## Integrated in :
    /// - Mission 3
    ScrollOfReinforcementI,

    /// ## Integrated in :
    /// -
    ScrollOfNaturalGrowthI,

    /// ## Integrated in :
    /// - Mission 5
    ScrollOfGaladornFailedPower,

    /// ## Integrated in :
    /// - Mission 1
    ScrollOfRawNaturalDefenseI,

    /// ## Integrated in :
    /// - Mission 3
    ScrollOfTheOutstandingFighterI,
}

impl ScrollsEnum {
    /// Get a scroll by its enum variant
    pub fn get_scroll(&self) -> Scroll {
        match self {
            ScrollsEnum::ScrollOfRawAttackI => Scroll {
                id: 1,
                image_atlas_index: 0,
                name: t!("scroll1_name").to_string(),
                price: 8,
                bonus: vec![BonusEnum::RawAttack(10)],
                ..Default::default()
            },
            ScrollsEnum::ScrollOfTheMiserI => Scroll {
                id: 2,
                image_atlas_index: 1,
                name: t!("scroll2_name").to_string(),
                price: 4,
                bonus: vec![BonusEnum::Gold(5)],
                ..Default::default()
            },
            ScrollsEnum::ScrollOfTheResearcherI => Scroll {
                id: 3,
                image_atlas_index: 2,
                name: t!("scroll3_name").to_string(),
                price: 6,
                bonus: vec![BonusEnum::LuckyLoot(5)],
                ..Default::default()
            },
            ScrollsEnum::ScrollOfExperienceI => Scroll {
                id: 4,
                image_atlas_index: 3,
                name: t!("scroll4_name").to_string(),
                price: 5,
                bonus: vec![BonusEnum::Experience(5)],
                ..Default::default()
            },
            ScrollsEnum::ScrollOfReinforcementI => Scroll {
                id: 5,
                image_atlas_index: 4,
                name: t!("scroll5_name").to_string(),
                price: 5,
                bonus: vec![BonusEnum::EnhanceEquipment(5)],
                ..Default::default()
            },
            ScrollsEnum::ScrollOfNaturalGrowthI => Scroll {
                id: 6,
                image_atlas_index: 5,
                name: t!("scroll6_name").to_string(),
                price: 5,
                bonus: vec![BonusEnum::NaturalGrowth(5)],
                ..Default::default()
            },
            ScrollsEnum::ScrollOfGaladornFailedPower => Scroll {
                id: 7,
                image_atlas_index: 6,
                name: t!("scroll7_name").to_string(),
                price: 25,
                bonus: vec![BonusEnum::RawAttack(1), BonusEnum::Collector(65)],
                ..Default::default()
            },
            ScrollsEnum::ScrollOfRawNaturalDefenseI => Scroll {
                id: 8,
                image_atlas_index: 7,
                name: t!("scroll8_name").to_string(),
                price: 7,
                bonus: vec![BonusEnum::NaturalRawDefense(5)],
                ..Default::default()
            },
            ScrollsEnum::ScrollOfTheOutstandingFighterI => Scroll {
                id: 9,
                image_atlas_index: 8,
                name: t!("scroll9_name").to_string(),
                price: 12,
                bonus: vec![BonusEnum::RawAttack(3), BonusEnum::NaturalRawDefense(3)],
                ..Default::default()
            },
        }
    }
}
