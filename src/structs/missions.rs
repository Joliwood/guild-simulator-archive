use super::{
    daily_events_folder::daily_events::get_random_index_from_percent_arr,
    equipments::ItemEnum,
    general_structs::{load_armor, load_scroll, load_weapon, Ennemy},
    player_stats::PlayerStats,
    recruits::RecruitStats,
};
use crate::{
    content::{
        equipments::{armors::ArmorsEnum, scrolls::ScrollsEnum, weapons::WeaponsEnum},
        missions::generate_all_missions,
    },
    structs::daily_events_folder::daily_events::calculate_total_apparition_chance,
    utils::{calculate_fight, calculate_price_range},
};
use bevy::prelude::*;
use rand::Rng;
use uuid::Uuid;

#[derive(Default, Debug, Component, Resource)]
pub struct MissionReports(pub Vec<MissionReport>);

#[derive(Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct MissionReport {
    pub experience_gained: Option<u32>,
    pub golds_gained: Option<i32>,
    pub loots: Vec<ItemEnum>,
    pub mission_id: u16,
    pub mission_ids_to_unlock: Vec<u16>,
    pub percent_of_victory: u32,
    pub recruit_id: Uuid,
    pub success: bool,
}

impl MissionReport {
    pub fn add_item(&mut self, item: ItemEnum) {
        match item {
            ItemEnum::Scroll(scroll, quantity) => {
                let scroll_id = scroll.id;
                if self.loots.iter().any(|item| match item {
                    ItemEnum::Scroll(scroll, _) => scroll.id == scroll_id,
                    _ => false,
                }) {
                    self.loots.iter_mut().for_each(|item| {
                        if let ItemEnum::Scroll(scroll, q) = item {
                            if scroll.id == scroll_id {
                                *q += quantity;
                            }
                        }
                    });
                } else {
                    self.loots.push(ItemEnum::Scroll(scroll, quantity));
                }
            }
            _ => {
                self.loots.push(item);
            }
        }
    }

    pub fn calculate_loots(&mut self, loots: Loots, recruit: &RecruitStats) {
        let item_loots = loots.0;

        if item_loots.is_empty() {
            return;
        }

        // Helper function to add item based on its enum type
        let mut add_item_to_inventory = |item_loot: &ItemLoot| match &item_loot.item {
            ItemLootEnum::Armor(armor) => {
                let armor = load_armor(armor.clone());
                self.add_item(ItemEnum::Armor(armor));
            }
            ItemLootEnum::Scroll(scroll) => {
                let scroll = load_scroll(scroll.clone());
                self.add_item(ItemEnum::Scroll(scroll, 1));
            }
            ItemLootEnum::Weapon(weapon) => {
                let weapon = load_weapon(weapon.clone());
                self.add_item(ItemEnum::Weapon(weapon));
            }
        };

        // We convert u8 to u16 because we can exceed the max of u8 with loot addition
        let all_item_chance_vec_for_first_loot: Vec<u16> = item_loots
            .iter()
            .map(|item| item.percent as u16)
            .collect::<Vec<u16>>();

        let first_random_item_index =
            get_random_index_from_percent_arr(&all_item_chance_vec_for_first_loot);

        // Step 1: Pick one guaranteed random item
        let first_item = &item_loots[first_random_item_index];
        add_item_to_inventory(first_item);

        let recruit_second_loot_chance_to_add = recruit
            .recruit_inventory
            .get_second_loot_chance_to_additionate_from_scroll_bonus();

        // Step 2: 50% chance to pick a second item (must be different)
        if item_loots.len() > 1 {
            let total_apparition_chance =
                calculate_total_apparition_chance(&all_item_chance_vec_for_first_loot);
            let second_chance = rand::thread_rng().gen_range(0..total_apparition_chance) as u8;
            if second_chance < 50 + recruit_second_loot_chance_to_add {
                let mut second_random_item_index =
                    get_random_index_from_percent_arr(&all_item_chance_vec_for_first_loot);

                // Ensure the second item is different from the first one
                while second_random_item_index == first_random_item_index {
                    second_random_item_index =
                        get_random_index_from_percent_arr(&all_item_chance_vec_for_first_loot);
                }

                let second_item = &item_loots[second_random_item_index];
                add_item_to_inventory(second_item);
            }
        }
    }
}

impl MissionReports {
    pub fn add_mission_report(&mut self, report: MissionReport) {
        self.0.push(report);
    }

    pub fn get_last_mission_report(&self) -> Option<MissionReport> {
        if let Some(report) = self.0.last() {
            return Some(report.clone());
        }
        None
    }

    pub fn remove_mission_report_by_id(&mut self, mission_id: u16) {
        if let Some(index) = self
            .0
            .iter()
            .position(|report| report.mission_id == mission_id)
        {
            self.0.remove(index);
        }
    }
}

#[derive(Default, Resource, Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct SelectedMission {
    pub mission_id: Option<u16>,
    pub percent_of_victory: Option<u32>,
    pub recruit_id: Option<Uuid>,
}

impl SelectedMission {
    pub fn calculate_percent_of_victory(
        &mut self,
        mission: &Mission,
        player_stats: &Res<PlayerStats>,
    ) {
        // This function should be call only if the mission_id is set
        if let Some(recruit_id) = self.recruit_id {
            let recruit = match player_stats.get_recruit_by_id(recruit_id) {
                Some(recruit) => recruit,
                None => {
                    error!(
                        "The recruit id doesn't exist in player recruits for this recruit_id : {}",
                        recruit_id
                    );
                    return;
                }
            };

            let victory_percentage = calculate_fight(&recruit, &mission.ennemy) as u32;

            self.percent_of_victory = Some(victory_percentage);
        }
    }

    pub fn reset(&mut self) {
        self.mission_id = None;
        self.percent_of_victory = None;
        self.recruit_id = None;
    }
}

#[derive(Debug, Component, Resource)]
pub struct Missions(pub Vec<Mission>);

impl Missions {
    pub fn get_mission_by_id(&self, id: &u16) -> Option<Mission> {
        if let Some(mission) = self.0.iter().find(|mission| mission.id == *id) {
            return Some(mission.clone());
        }
        return None;
    }

    pub fn assign_recruit_id_to_mission_by_id(&mut self, mission_id: u16, recruit_id: Uuid) {
        if let Some(mission) = self.0.iter_mut().find(|mission| mission.id == mission_id) {
            mission.assign_recruit_by_id(recruit_id);
        }
    }

    pub fn decrement_days_left_by_mission_id(&mut self, mission_id: u16) {
        if let Some(mission) = self.0.iter_mut().find(|mission| mission.id == mission_id) {
            mission.decrement_days_left();
        }
    }

    pub fn attribute_days_left_by_mission_id(&mut self, mission_id: u16) {
        if let Some(mission) = self.0.iter_mut().find(|mission| mission.id == mission_id) {
            mission.attribute_days_left();
        }
    }

    pub fn is_mission_over(&self, mission_id: u16) -> bool {
        if let Some(mission) = self.0.iter().find(|mission| mission.id == mission_id) {
            return mission.days_left.is_none();
        }
        return false;
    }

    pub fn get_recruit_send_id_by_mission_id(&self, mission_id: u16) -> Option<Uuid> {
        if let Some(mission) = self.0.iter().find(|mission| mission.id == mission_id) {
            return mission.recruit_send;
        }
        return None;
    }

    pub fn desassign_recruit_to_mission(&mut self, mission_id: u16) {
        if let Some(mission) = self.0.iter_mut().find(|mission| mission.id == mission_id) {
            mission.desassign_recruit();
        }
    }

    pub fn get_mission_enemmy_level_by_id(&self, mission_id: u16) -> Option<u8> {
        if let Some(mission) = self.0.iter().find(|mission| mission.id == mission_id) {
            return Some(mission.ennemy.level);
        }
        return None;
    }

    pub fn get_percent_of_victory_by_mission_id(&self, mission_id: u16) -> Option<u32> {
        if let Some(mission) = self.0.iter().find(|mission| mission.id == mission_id) {
            return mission.percent_of_victory;
        }
        return None;
    }

    pub fn attribute_percent_of_victory_to_mission(
        &mut self,
        mission_id: u16,
        percent_of_victory: u32,
    ) {
        if let Some(mission) = self.0.iter_mut().find(|mission| mission.id == mission_id) {
            mission.percent_of_victory = Some(percent_of_victory);
        }
    }

    pub fn get_missions_by_ids(&self, ids: &Vec<u16>) -> Vec<Mission> {
        let mut missions = vec![];

        for id in ids {
            if let Some(mission) = self.get_mission_by_id(id) {
                missions.push(mission);
            }
        }

        return missions;
    }

    pub fn get_golds_earned_by_mission_id(&self, mission_id: u16) -> Option<u32> {
        if let Some(mission) = self.0.iter().find(|mission| mission.id == mission_id) {
            return Some(mission.golds);
        }
        return None;
    }

    pub fn unlock_missions_by_mission_id(&mut self, mission_id: u16) {
        if let Some(mission) = self.0.iter_mut().find(|mission| mission.id == mission_id) {
            for id in mission.unlock_mission_ids.clone() {
                if let Some(mission_to_unlock) = self.0.iter_mut().find(|mission| mission.id == id)
                {
                    mission_to_unlock.unlocked = true;
                }
            }
        }
    }

    pub fn is_mission_unlocked_by_id(&self, mission_id: u16) -> bool {
        if let Some(mission) = self.0.iter().find(|mission| mission.id == mission_id) {
            return mission.unlocked;
        }
        return false;
    }
}

#[derive(Debug, Component, Clone, Eq, PartialEq, Hash)]
pub enum ItemLootEnum {
    Armor(ArmorsEnum),
    Scroll(ScrollsEnum),
    Weapon(WeaponsEnum),
}

#[derive(Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct ItemLoot {
    pub item: ItemLootEnum,
    pub percent: u8,
}

impl ItemLoot {
    pub fn get_atlas_index(&self) -> u16 {
        match &self.item {
            ItemLootEnum::Armor(armor) => armor.get_armor().image_atlas_index,
            ItemLootEnum::Scroll(scroll) => scroll.get_scroll().image_atlas_index,
            ItemLootEnum::Weapon(weapon) => weapon.get_weapon().image_atlas_index,
        }
    }

    pub fn get_item_loot_tooltip_description(&self) -> String {
        return match self {
            ItemLoot {
                item: ItemLootEnum::Armor(armor),
                ..
            } => {
                let armor = armor.get_armor();
                let mut description = format!("{}\n", armor.name);
                let price_range = calculate_price_range(armor.price);

                if let Some(attack) = armor.attack {
                    description.push_str(&format!("\n{}: {}", t!("attack"), attack));
                }

                if let Some(defense) = armor.defense {
                    description.push_str(&format!("\n{}: {}", t!("defense"), defense));
                }

                if !armor.optimized_for.0.is_empty() {
                    description.push_str(&format!(
                        "\n\n{} :",
                        t!("tooltip_optimized_for", points = armor.optimized_for.1),
                    ));

                    for classe in &armor.optimized_for.0 {
                        description.push_str(&format!("\n- {}", classe));
                    }
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
            ItemLoot {
                item: ItemLootEnum::Scroll(scroll),
                ..
            } => {
                let scroll = scroll.get_scroll();
                let mut description = format!("{}\n", scroll.name);
                let price_range = calculate_price_range(scroll.price);

                if let Some(attack) = scroll.attack {
                    description.push_str(&format!("\n{}: {}", t!("attack"), attack));
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
            ItemLoot {
                item: ItemLootEnum::Weapon(weapon),
                ..
            } => {
                let weapon = weapon.get_weapon();
                let mut description = format!("{}\n", weapon.name);
                let price_range = calculate_price_range(weapon.price);

                if let Some(attack) = weapon.attack {
                    description.push_str(&format!("\n{}: {}", t!("attack"), attack));
                }

                if let Some(defense) = weapon.defense {
                    description.push_str(&format!("\n{}: {}", t!("defense"), defense));
                }

                if !weapon.optimized_for.0.is_empty() {
                    description.push_str(&format!(
                        "\n\n{} :",
                        t!("tooltip_optimized_for", points = weapon.optimized_for.1),
                    ));

                    for classe in &weapon.optimized_for.0 {
                        description.push_str(&format!("\n- {}", classe));
                    }
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
        };
    }
}

#[derive(Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct Loots(pub Vec<ItemLoot>);

#[derive(Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct Mission {
    pub days_left: Option<u8>,
    pub days: u8,
    pub description: String,
    pub ennemy: Ennemy,
    pub golds: u32,
    pub id: u16,
    pub level: u8,
    pub loots: Loots,
    pub name: String,
    pub percent_of_victory: Option<u32>,
    pub recruit_send: Option<Uuid>,
    pub unlock_mission_ids: Vec<u16>,
    pub unlocked: bool,
}

impl Mission {
    pub fn decrement_days_left(&mut self) {
        if let Some(days_left) = &mut self.days_left {
            if *days_left == 1 {
                return self.days_left = None;
            }
            if *days_left > 0 {
                return *days_left -= 1;
            }
        }
    }

    pub fn assign_recruit_by_id(&mut self, recruit_id: Uuid) {
        self.recruit_send = Some(recruit_id);
    }

    pub fn desassign_recruit(&mut self) {
        self.recruit_send = None;
    }

    pub fn attribute_days_left(&mut self) {
        self.days_left = Some(self.days);
    }
}

impl Default for Missions {
    fn default() -> Self {
        return generate_all_missions();
    }
}
