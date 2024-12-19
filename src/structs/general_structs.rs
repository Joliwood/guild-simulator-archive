#![allow(dead_code)]

use super::{
    equipments::{Armor, ItemEnum, Scroll, Weapon},
    player_stats::PlayerStats,
};
use crate::{
    content::equipments::{armors::ArmorsEnum, scrolls::ScrollsEnum, weapons::WeaponsEnum},
    enums::RoomEnum,
};
use bevy::{
    color::Color,
    prelude::{Component, Resource},
};

#[derive(Resource)]
pub struct MissionNotificationsNumber(pub u8);

#[derive(Component, Resource)]
pub struct MissionModalVisible(pub bool);

#[derive(Component, Resource)]
pub struct MissionReportsModalVisible(pub bool);

#[derive(Component, Resource)]
pub struct DailyEventsModalVisible(pub bool);

#[derive(Component, Resource)]
pub struct TutoMessagesModalVisible(pub bool);

#[derive(Component, Resource)]
pub struct TutoDoneModalVisible(pub bool);

#[derive(Component)]
pub struct UniqueId(pub String);

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Component, Resource)]
pub struct Ennemy {
    pub experience: u32,
    pub level: u8,
    pub name: String,
    pub attack: u32,
    pub defense: u32,
    pub image_atlas_index: u16,
}

pub fn load_weapon(weapon: WeaponsEnum) -> Weapon {
    return WeaponsEnum::get_weapon(&weapon);
}

pub fn load_scroll(scroll: ScrollsEnum) -> Scroll {
    return ScrollsEnum::get_scroll(&scroll);
}

pub fn load_armor(armor: ArmorsEnum) -> Armor {
    return ArmorsEnum::get_armor(&armor);
}

#[derive(Default, Resource)]
pub struct DayTime {
    pub current_time: f32,
    pub elapsed_time: f32,
    pub second_count: u32,
}

impl DayTime {
    fn get_hours_minutes(&self) -> (u32, u32) {
        let total_minutes = self.current_time as u32;
        let hours = 8 + total_minutes / 60;
        let minutes = total_minutes % 60;
        (hours, minutes)
    }

    pub fn reset(&mut self) {
        self.current_time = 0.0;
        self.elapsed_time = 0.0;
        self.second_count = 0;
    }
}

#[derive(Default, Debug, Resource)]
pub struct NotificationCount {
    pub command_room_count: u8,
    pub office_count: u8,
    pub barrack_count: u8,
}

impl NotificationCount {
    pub fn increment_command_room_count(&mut self, count: u8, player_stats: &mut PlayerStats) {
        if player_stats.room != RoomEnum::CommandRoom {
            self.command_room_count += count;
        }
    }

    pub fn increment_office_count(&mut self, count: u8, player_stats: &mut PlayerStats) {
        if player_stats.room != RoomEnum::Office {
            self.office_count += count;
        }
    }

    pub fn increment_barrack_count(&mut self, count: u8, player_stats: &mut PlayerStats) {
        if player_stats.room != RoomEnum::Barrack {
            self.barrack_count += count;
        }
    }
}

pub struct OverlayColor;

impl OverlayColor {
    pub const GREEN: Color = Color::srgb(0.0, 1.0, 0.0);
}

#[derive(Component)]
pub struct RoomTag(pub RoomEnum);

#[derive(Component)]
pub struct ItemInInventoryTrigger(pub Option<ItemEnum>);
