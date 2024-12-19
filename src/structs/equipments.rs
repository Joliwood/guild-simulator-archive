use crate::{
    enums::{ClassEnum, ItemRaretyEnum},
    utils::calculate_price_range,
};
use bevy::prelude::Component;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Component, PartialEq, Eq, Hash)]
pub enum ItemEnum {
    Weapon(Weapon),
    Armor(Armor),
    Scroll(Scroll, u16),
}

impl ItemEnum {
    pub fn get_atlas_index(&self) -> u16 {
        match self {
            ItemEnum::Weapon(weapon) => return weapon.image_atlas_index,
            ItemEnum::Armor(armor) => return armor.image_atlas_index,
            ItemEnum::Scroll(scroll, _) => return scroll.image_atlas_index,
        }
    }

    pub fn get_item_loot_tooltip_description(&self) -> String {
        match self {
            ItemEnum::Weapon(weapon) => {
                let mut description = weapon.name.to_string();
                let price_range = calculate_price_range(weapon.price);

                if let Some(attack) = weapon.attack {
                    description.push_str(&format!("\n{}: {}", t!("attack"), attack));
                }

                if let Some(defense) = weapon.defense {
                    description.push_str(&format!("\n{}: {}", t!("defense"), defense));
                }

                description.push_str(&format!(
                    "\n\n{}: {} {} {} G",
                    t!("price"),
                    price_range.0,
                    t!("to"),
                    price_range.1
                ));

                description
            }
            ItemEnum::Armor(armor) => {
                let mut description = armor.name.to_string();
                let price_range = calculate_price_range(armor.price);

                if let Some(attack) = armor.attack {
                    description.push_str(&format!("\n{}: {}", t!("attack"), attack));
                }

                if let Some(defense) = armor.defense {
                    description.push_str(&format!("\n{}: {}", t!("defense"), defense));
                }

                description.push_str(&format!(
                    "\n\n{}: {} {} {} G",
                    t!("price"),
                    price_range.0,
                    t!("to"),
                    price_range.1
                ));

                description
            }
            ItemEnum::Scroll(scroll, _) => {
                let mut description = scroll.name.to_string();
                let price_range = calculate_price_range(scroll.price);

                if let Some(attack) = scroll.attack {
                    description.push_str(&format!("\n\n{}: {}", t!("attack"), attack));
                }

                if let Some(defense) = scroll.defense {
                    description.push_str(&format!("\n{}: {}", t!("defense"), defense));
                }

                for bonus in scroll.bonus.iter() {
                    description.push_str(&format!("\n{:?}", bonus));
                }

                description.push_str(&format!(
                    "\n\n{}: {} {} {} G",
                    t!("price"),
                    price_range.0,
                    t!("to"),
                    price_range.1
                ));

                description
            }
        }
    }
}

#[derive(Default, Debug, Clone, Deserialize, Eq, PartialEq, Hash)]
pub struct Weapon {
    pub id: u16,
    pub image_atlas_index: u16,
    pub name: String,
    pub price: u16,
    pub attack: Option<u32>,
    pub defense: Option<u32>,
    pub rarety: ItemRaretyEnum,
    /// The best bonus for the attack, it adds a bonus in % for attack (recruit
    /// + raw bonus + equipments)
    pub optimized_for: (Vec<ClassEnum>, u32),
}

#[derive(Default, Debug, Clone, Deserialize, Eq, PartialEq, Hash)]
pub struct Armor {
    pub id: u16,
    pub image_atlas_index: u16,
    pub name: String,
    pub price: u16,
    pub rarety: ItemRaretyEnum,
    pub attack: Option<u32>,
    pub defense: Option<u32>,
    /// The best bonus for the attack, it adds a bonus in % for attack (recruit
    /// + raw bonus + equipments)
    pub optimized_for: (Vec<ClassEnum>, u32),
}

#[derive(Default, Debug, Clone, Deserialize, Eq, PartialEq, Hash)]
pub struct Scroll {
    pub id: u16,
    pub image_atlas_index: u16,
    pub name: String,
    pub price: u16,
    pub attack: Option<u32>,
    pub defense: Option<u32>,
    pub bonus: Vec<BonusEnum>,
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Hash)]
pub enum BonusEnum {
    /// Raw attack
    RawAttack(u32),

    /// Add % golds for each success mission
    Gold(u32),

    /// Increase the chance to earn a second loot in % for each success mission
    LuckyLoot(u8),

    /// Add % experience for each success mission
    Experience(u32),

    /// Increate the recruit's equipment (all of them) stats by %
    EnhanceEquipment(u32),

    /// Increate the native recruit stats (all of them) by %
    NaturalGrowth(u32),

    /// Amount of golds it can be increased when a buyer is interested
    Collector(u32),

    /// Increase the native recruit defense
    NaturalRawDefense(u32),

    /// Increase the native recruit attack
    NaturalRawAttack(u32),
}
