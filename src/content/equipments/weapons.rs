#![allow(dead_code)]

use crate::{
    enums::{ClassEnum, ItemRaretyEnum},
    structs::equipments::Weapon,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum WeaponsEnum {
    /// ## Integrated in :
    /// - Mission 5
    /// - Daily discussion 10
    WoodenSword,

    /// ## Integrated in :
    /// - Mission 4
    BowWithoutString,

    /// ## Integrated in :
    /// - Mission 5
    LumberjackAxe,

    /// ## Integrated in :
    /// - Mission 1
    MagicToothpick,

    /// ## Integrated in :
    /// - Mission 4
    UnsharpDagger,

    /// ## Integrated in :
    /// - Mission 2
    WalkingStick,
}

impl WeaponsEnum {
    /// Get a weapon by its enum variant
    pub fn get_weapon(&self) -> Weapon {
        match self {
            WeaponsEnum::WoodenSword => Weapon {
                id: 1,
                image_atlas_index: 0,
                name: t!("weapon1_name").to_string(),
                price: 10,
                rarety: ItemRaretyEnum::Common,
                attack: Some(7),
                defense: Some(2),
                ..Default::default()
            },
            WeaponsEnum::BowWithoutString => Weapon {
                id: 2,
                image_atlas_index: 1,
                name: t!("weapon2_name").to_string(),
                price: 2,
                rarety: ItemRaretyEnum::Common,
                optimized_for: (vec![ClassEnum::Hunter], (5)),
                attack: Some(3),
                ..Default::default()
            },
            WeaponsEnum::LumberjackAxe => Weapon {
                id: 3,
                image_atlas_index: 2,
                name: t!("weapon3_name").to_string(),
                price: 5,
                rarety: ItemRaretyEnum::Common,
                attack: Some(7),
                ..Default::default()
            },
            WeaponsEnum::MagicToothpick => Weapon {
                id: 4,
                image_atlas_index: 3,
                name: t!("weapon4_name").to_string(),
                price: 2,
                rarety: ItemRaretyEnum::Common,
                optimized_for: (vec![ClassEnum::Mage], (20)),
                attack: Some(2),
                ..Default::default()
            },
            WeaponsEnum::UnsharpDagger => Weapon {
                id: 5,
                image_atlas_index: 4,
                name: t!("weapon5_name").to_string(),
                price: 4,
                rarety: ItemRaretyEnum::Common,
                attack: Some(4),
                ..Default::default()
            },
            WeaponsEnum::WalkingStick => Weapon {
                id: 6,
                image_atlas_index: 5,
                name: t!("weapon6_name").to_string(),
                price: 4,
                rarety: ItemRaretyEnum::Common,
                attack: Some(5),
                defense: Some(1),
                ..Default::default()
            },
        }
    }
}
