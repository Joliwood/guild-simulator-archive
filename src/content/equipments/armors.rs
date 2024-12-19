#![allow(dead_code)]

use crate::{
    enums::{ClassEnum, ItemRaretyEnum},
    structs::equipments::Armor,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ArmorsEnum {
    /// ## Integrated in :
    /// - Mission 2
    UsedLeatherToga,

    /// ## Integrated in :
    /// - Mission 4
    ApprenticeCoat,

    /// ## Integrated in :
    /// - Mission 6
    RecycledMagicianRobe,

    /// ## Integrated in :
    /// - Mission 6
    LeatherTunic,

    /// ## Integrated in :
    /// - Mission 1
    MakeshiftVest,

    /// ## Integrated in :
    /// - Daily discussion 9
    TravelToga,
}

impl ArmorsEnum {
    /// Get an armor by its enum variant
    pub fn get_armor(&self) -> Armor {
        match self {
            ArmorsEnum::UsedLeatherToga => Armor {
                id: 1,
                image_atlas_index: 0,
                name: t!("armor1_name").to_string(),
                defense: Some(4),
                price: 5,
                rarety: ItemRaretyEnum::Common,
                ..Default::default()
            },
            ArmorsEnum::ApprenticeCoat => Armor {
                id: 2,
                image_atlas_index: 1,
                name: t!("armor2_name").to_string(),
                defense: Some(6),
                price: 7,
                rarety: ItemRaretyEnum::Common,
                ..Default::default()
            },
            ArmorsEnum::RecycledMagicianRobe => Armor {
                id: 3,
                image_atlas_index: 2,
                name: t!("armor3_name").to_string(),
                optimized_for: (vec![ClassEnum::Mage], (15)),
                defense: Some(4),
                price: 10,
                rarety: ItemRaretyEnum::Common,
                ..Default::default()
            },
            ArmorsEnum::LeatherTunic => Armor {
                id: 4,
                image_atlas_index: 3,
                name: t!("armor4_name").to_string(),
                optimized_for: (vec![ClassEnum::Hunter, ClassEnum::Rogue], (3)),
                defense: Some(12),
                price: 12,
                rarety: ItemRaretyEnum::Common,
                ..Default::default()
            },
            ArmorsEnum::MakeshiftVest => Armor {
                id: 5,
                image_atlas_index: 4,
                name: t!("armor5_name").to_string(),
                defense: Some(5),
                price: 5,
                rarety: ItemRaretyEnum::Common,
                ..Default::default()
            },
            ArmorsEnum::TravelToga => Armor {
                id: 6,
                image_atlas_index: 5,
                name: t!("armor6_name").to_string(),
                defense: Some(7),
                price: 7,
                rarety: ItemRaretyEnum::Common,
                ..Default::default()
            },
        }
    }
}
