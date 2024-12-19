#![allow(dead_code)]

use super::{
    daily_events_folder::daily_events::{Answer, ImpactAction},
    equipments::ItemEnum,
    general_structs::NotificationCount,
    recruits::RecruitStats,
};
use crate::enums::{RecruitStateEnum, RoomEnum};
use bevy::prelude::*;
use uuid::Uuid;

#[derive(Default, Component, Resource, Clone, PartialEq)]
pub struct Stats {
    pub golds_earned: i32,
    pub mission_completed: u16,
}

/// Tutorial struct
///
/// When it is None, the tutorial is not available
/// When it is Some(false), the tutorial is available but not done
/// When it is Some(true), the tutorial is done
#[derive(Default, Component, Resource, Clone, PartialEq)]
pub struct Tuto {
    pub is_barrack_room_tuto_done: Option<bool>,
    pub is_command_room_tuto_done: Option<bool>,
    pub is_first_daily_events_done: Option<bool>,
    pub is_tuto_completed: bool,
}

impl Tuto {
    pub fn count_tuto_available(&self) -> u8 {
        let mut count = 0;
        if let Some(is_barrack_room_tuto_done) = self.is_barrack_room_tuto_done {
            if !is_barrack_room_tuto_done {
                count += 1;
            }
        }
        if let Some(is_command_room_tuto_done) = self.is_command_room_tuto_done {
            if !is_command_room_tuto_done {
                count += 1;
            }
        }
        if let Some(is_first_daily_events_done) = self.is_first_daily_events_done {
            if !is_first_daily_events_done {
                count += 1;
            }
        }

        return count;
    }

    pub fn skip_tuto(&mut self) {
        self.is_barrack_room_tuto_done = Some(true);
        self.is_command_room_tuto_done = Some(true);
        self.is_first_daily_events_done = Some(true);
    }
}

pub enum TutoEnum {
    BarrackRoom,
    CommandRoom,
    DailyEvents,
}

#[derive(Debug, Component, Resource, Clone)]
pub struct TutoMessage {
    pub id: u8,
    pub title: String,
    pub messages: Vec<String>,
}

#[derive(Debug, Component, Resource, Clone)]
pub struct TutoMessages(pub Vec<TutoMessage>);

impl Default for TutoMessages {
    fn default() -> Self {
        Self(vec![TutoMessage {
            id: 0,
            title: t!("tuto_message_init_title").to_string(),
            messages: vec![
                t!("tuto_message_init_desc_1").to_string(),
                t!("tuto_message_init_desc_2").to_string(),
                t!("tuto_message_init_desc_3").to_string(),
            ],
        }])
    }
}

impl TutoMessages {
    pub fn get_first_tuto_message(&self) -> Option<&TutoMessage> {
        self.0.first()
    }

    fn remove_initial_tuto_message(&mut self) {
        self.0.retain(|tuto_message| tuto_message.id != 0);
    }

    pub fn add_tuto_message(&mut self, tuto_type: TutoEnum) {
        Self::remove_initial_tuto_message(self);

        match tuto_type {
            TutoEnum::BarrackRoom => {
                self.0.push(TutoMessage {
                    id: 1,
                    title: t!("tuto_message_barrack_title").to_string(),
                    messages: vec![
                        t!("tuto_message_barrack_desc_1").to_string(),
                        t!("tuto_message_barrack_desc_2").to_string(),
                        t!("tuto_message_barrack_desc_3").to_string(),
                    ],
                });
            }
            TutoEnum::CommandRoom => {
                self.0.push(TutoMessage {
                    id: 2,
                    title: t!("tuto_message_command_room_title").to_string(),
                    messages: vec![
                        t!("tuto_message_command_room_desc_1").to_string(),
                        t!("tuto_message_command_room_desc_2").to_string(),
                        t!("tuto_message_command_room_desc_3").to_string(),
                    ],
                });
            }
            TutoEnum::DailyEvents => {
                self.0.push(TutoMessage {
                    id: 3,
                    title: t!("tuto_message_daily_events_title").to_string(),
                    messages: vec![
                        t!("tuto_message_daily_events_desc_1").to_string(),
                        t!("tuto_message_daily_events_desc_2").to_string(),
                        t!("tuto_message_daily_events_desc_3").to_string(),
                        t!("tuto_message_daily_events_desc_4").to_string(),
                    ],
                });
            }
        }
    }

    pub fn remove_first_tuto_message(&mut self, player_stats: &mut ResMut<PlayerStats>) {
        if let Some(tuto_message) = self.0.first() {
            match tuto_message.id {
                1 => {
                    player_stats.tuto.is_barrack_room_tuto_done = Some(true);
                }
                2 => {
                    player_stats.tuto.is_command_room_tuto_done = Some(true);
                }
                3 => {
                    player_stats.tuto.is_first_daily_events_done = Some(true);
                }
                _ => {}
            }
        }
        self.0.remove(0);
    }
}

#[derive(Component, Resource, Clone, PartialEq)]
pub struct PlayerStats {
    pub day: u16,
    pub experience: u32,
    pub golds: i32,
    pub guild_level: i8,
    pub inventory: Vec<ItemEnum>,
    pub max_experience: u32,
    pub max_inventory_size: usize,
    pub recruits: Vec<RecruitStats>,
    pub room: RoomEnum,
    pub toxicity: i8,
    pub stats: Stats,
    pub reputation: i8,
    pub tuto: Tuto,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            day: 1,
            experience: 0,
            golds: 30,
            guild_level: 1,
            inventory: vec![
                // ItemEnum::Weapon(WeaponsEnum::MagicToothpick.get_weapon()),
                // ItemEnum::Weapon(WeaponsEnum::MagicToothpick.get_weapon()),
                // ItemEnum::Armor(ArmorsEnum::LeatherTunic.get_armor()),
                // ItemEnum::Armor(ArmorsEnum::LeatherTunic.get_armor()),
                // ItemEnum::Scroll(ScrollsEnum::ScrollOfRawAttackI.get_scroll(), 2),
                // ItemEnum::Scroll(ScrollsEnum::ScrollOfGaladornFailedPower.get_scroll(), 2),
            ],
            max_experience: 1000,
            max_inventory_size: 50,
            recruits: vec![
                // RecruitEnum::Hubert.get_recruit(),
                // RecruitEnum::JeanLouisDavid.get_recruit(),
                // RecruitEnum::JeanLouisDavid.get_recruit(),
                // RecruitEnum::JeanLouisDavid.get_recruit(),
                // RecruitEnum::JeanLouisDavid.get_recruit(),
            ],
            room: RoomEnum::Office,
            toxicity: 0,
            reputation: 10,
            stats: Stats::default(),
            tuto: Tuto::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemChangeEnum {
    Add(Uuid),
    Update(Uuid),
    Remove(Uuid),
}

pub const SKIP_TUTO: bool = false;

impl PlayerStats {
    pub fn increment_golds(&mut self, amount: i32) {
        self.golds += amount;
        if self.golds < 0 {
            self.golds = 0;
        }
    }

    pub fn gain_xp(&mut self, xp: u32) {
        self.experience += xp;

        // Reset the experience with left experience after leveling up
        // Then level up
        if self.experience >= self.max_experience {
            self.experience -= self.max_experience;
            self.level_up();
        }
    }

    pub fn level_up(&mut self) {
        self.guild_level += 1;
        // Set the max experience to the current experience * 2
        self.max_experience *= 2;
    }

    pub fn find_item_by_id(&self, id: u16) -> Option<ItemEnum> {
        if let Some(item) = self.inventory.iter().find(|item| match item {
            ItemEnum::Weapon(weapon) => weapon.id == id,
            ItemEnum::Armor(armor) => armor.id == id,
            ItemEnum::Scroll(scroll, _) => scroll.id == id,
        }) {
            return Some(item.clone());
        }

        return None;
    }

    pub fn add_item(&mut self, item: ItemEnum) {
        match item.clone() {
            ItemEnum::Scroll(scroll, quantity) => {
                let scroll_id = scroll.id;
                if self.inventory.iter().any(|item| match item {
                    ItemEnum::Scroll(scroll, _) => scroll.id == scroll_id,
                    _ => false,
                }) {
                    self.inventory.iter_mut().for_each(|item| {
                        if let ItemEnum::Scroll(scroll, q) = item {
                            if scroll.id == scroll_id {
                                *q += quantity;
                            }
                        }
                    });
                } else {
                    self.inventory.push(ItemEnum::Scroll(scroll, quantity));
                }
            }
            item => {
                if self.inventory.len() < self.max_inventory_size {
                    self.inventory.push(item.clone());
                }
            }
        }
    }

    pub fn remove_item(&mut self, item: &ItemEnum) {
        if let Some(item_index) = self.inventory.iter().position(|i| i == item) {
            self.inventory.remove(item_index);
        }
    }

    pub fn get_recruit_by_id(&self, id: Uuid) -> Option<RecruitStats> {
        if let Some(recruit) = self.recruits.iter().find(|recruit| recruit.id == id) {
            return Some(recruit.clone());
        }

        return None;
    }

    pub fn equip_item_to_recruit(&mut self, recruit_id: Uuid, item: &ItemEnum) {
        if let Some(recruit) = self
            .recruits
            .iter_mut()
            .find(|recruit| recruit.id == recruit_id)
        {
            recruit.equip_item(item);
        }
    }

    pub fn desequip_item_to_recruit(&mut self, recruit_id: Uuid, item: &ItemEnum) {
        if let Some(recruit) = self
            .recruits
            .iter_mut()
            .find(|recruit| recruit.id == recruit_id)
        {
            recruit.desequip_item(item);
        }
    }

    pub fn gain_xp_to_recruit(&mut self, recruit_id: Uuid, xp: u32) {
        if let Some(recruit) = self
            .recruits
            .iter_mut()
            .find(|recruit| recruit.id == recruit_id)
        {
            let recruit_xp_multiplicator = recruit
                .recruit_inventory
                .get_experience_multiplicator_from_scroll_bonus();

            recruit.gain_xp((xp as f64 * recruit_xp_multiplicator) as u32);
        }
    }

    pub fn remove_one_scroll_from_inventory(&mut self, scroll_id: u16) {
        if let Some(scroll_index) = self.inventory.iter().position(|item| match item {
            ItemEnum::Scroll(scroll, _) => scroll.id == scroll_id,
            _ => false,
        }) {
            if let ItemEnum::Scroll(_scroll, quantity) = &mut self.inventory[scroll_index] {
                if *quantity > 1 {
                    *quantity -= 1;
                } else {
                    self.inventory.remove(scroll_index);
                }
            }
        }
    }

    pub fn update_state_of_recruit(&mut self, recruit_id: Uuid, state: RecruitStateEnum) {
        if let Some(recruit) = self
            .recruits
            .iter_mut()
            .find(|recruit| recruit.id == recruit_id)
        {
            recruit.state = state;
        }
    }

    pub fn gain_toxitiy(&mut self, toxicity: i8) {
        self.toxicity = (self.toxicity + toxicity).clamp(0, 100);
    }

    pub fn gain_reputation(&mut self, reputation: i8) {
        self.reputation = (self.reputation + reputation).clamp(0, 100);
    }

    pub fn apply_equipment_impact(
        &mut self,
        answer: &Answer,
        notification_count: &mut ResMut<NotificationCount>,
    ) {
        if let Some(experience_impact) = &answer.experience_impact {
            self.gain_xp(*experience_impact);
        }

        if let Some(gold_impact) = &answer.gold_impact {
            self.increment_golds(*gold_impact);
        }

        if let Some(recruit_impact) = &answer.recruit_impact {
            self.recruits.push(recruit_impact.clone());
            notification_count.increment_barrack_count(1, self);
        }

        if let Some(reputation_impact) = &answer.reputation_impact {
            self.gain_reputation(*reputation_impact);
        }

        if let Some(toxicity_impact) = &answer.toxicity_impact {
            self.gain_toxitiy(*toxicity_impact);
        }
        if let Some(equipment_impact) = &answer.equipment_impact {
            for impact_action in equipment_impact {
                match impact_action {
                    ImpactAction::Add(item) => self.add_item(item.clone()),
                    ImpactAction::Remove(item) => self.remove_item(item),
                }
            }

            notification_count.increment_barrack_count(equipment_impact.len() as u8, self);
        }
    }

    pub fn remove_recruit_by_id(&mut self, id: Uuid) {
        if let Some(recruit_index) = self.recruits.iter().position(|recruit| recruit.id == id) {
            self.recruits.remove(recruit_index);
        }
    }

    pub fn heal_all_injured_recruits(&mut self) {
        for recruit in self.recruits.iter_mut() {
            if recruit.state == RecruitStateEnum::Injured {
                recruit.state = RecruitStateEnum::Available;
            }
        }
    }
}
