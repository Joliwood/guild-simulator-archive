#![allow(dead_code)]

use crate::structs::{equipments::ItemEnum, missions::ItemLootEnum};
use bevy::{color::Color, prelude::Component, state::state::States};
use serde::Deserialize;
use std::fmt::{self, Display};

#[derive(Default, Debug, PartialEq, Clone, Eq, Hash, Deserialize)]
pub enum ClassEnum {
    Hunter,
    Mage,
    Rogue,
    #[default]
    Warrior,
}

// #[derive(Component, Debug, PartialEq)]
#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States, Component)]
///  Where the player is
///
/// - Office : Main place, at our desk
/// - Store : The counter when player can buy stuff from a marchand (once at a time, with a CD), or sell stuff
/// - Barrack : Where the troups are, can equip them with stuff, check their level, hp, etc
/// - CommandRoom : Where the player can send troups to missions, check the missions, etc
pub enum RoomEnum {
    Barrack,
    CommandRoom,
    #[default]
    Office,
    Store,
}

#[derive(Debug, PartialEq)]
pub enum RoomDirectionEnum {
    Bottom,
    Left,
    Right,
    Top,
}

impl Display for RoomEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for ClassEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Default, Debug, Clone, Deserialize, Eq, PartialEq, Hash)]
pub enum ItemRaretyEnum {
    #[default]
    Common,
    UnCommon,
    Rare,
}

#[derive(Debug, Clone)]
pub enum SoundEnum {
    BookThrowDown,
    CockrelMorning,
    EquipArmor,
    EquipmentEquip,
    EquipScroll,
    EquipWeapon,
    KeysRemovedFromDoor,
    PaperTouch,
    PencilSign,
    PickingGolds,
    SimpleHolidaysV3,
    DeadMale,
}

impl SoundEnum {
    pub fn get_path(&self) -> &'static str {
        match self {
            SoundEnum::BookThrowDown => "sounds/book_throw_down.ogg",
            SoundEnum::CockrelMorning => "sounds/cockrel_morning.ogg",
            SoundEnum::EquipArmor => "sounds/equip_armor.ogg",
            SoundEnum::EquipmentEquip => "sounds/equipment_equip.ogg",
            SoundEnum::EquipScroll => "sounds/equip_scroll.ogg",
            SoundEnum::EquipWeapon => "sounds/equip_weapon.ogg",
            SoundEnum::KeysRemovedFromDoor => "sounds/keys_removed_from_door.ogg",
            SoundEnum::PaperTouch => "sounds/paper_touch.ogg",
            SoundEnum::PencilSign => "sounds/pencil_sign.ogg",
            SoundEnum::PickingGolds => "sounds/picking_golds.ogg",
            SoundEnum::SimpleHolidaysV3 => "sounds/Simple-Holidays-V3.ogg",
            SoundEnum::DeadMale => "sounds/dead_male.ogg",
        }
    }
}

#[derive(Debug)]
pub enum ColorPaletteEnum {
    Danger,
    DarkBrown,
    Success,
    Warning,
    Brown,
    Wood,
    DarkGray,
    SunnyLight,
}

impl ColorPaletteEnum {
    pub fn as_color(&self) -> Color {
        match *self {
            ColorPaletteEnum::Danger => Color::srgb(0.9, 0.1, 0.1),
            ColorPaletteEnum::DarkBrown => Color::srgb(0.13, 0.11, 0.09),
            ColorPaletteEnum::Success => Color::srgb(0.1, 0.8, 0.1),
            ColorPaletteEnum::Warning => Color::srgb(0.9, 0.7, 0.0),
            ColorPaletteEnum::Brown => Color::srgba(0.647, 0.165, 0.165, 0.5),
            ColorPaletteEnum::Wood => {
                Color::srgba(193.0 / 255.0, 154.0 / 255.0, 107.0 / 255.0, 1.0)
            }
            ColorPaletteEnum::DarkGray => Color::srgba(0.2, 0.2, 0.2, 1.0),
            ColorPaletteEnum::SunnyLight => Color::srgba(0.9686, 0.9686, 0.7137, 1.0),
        }
    }
}

#[derive(Default, Debug, Clone, Deserialize, Eq, PartialEq, Hash)]
pub enum RecruitStateEnum {
    #[default]
    Available,
    Injured,
    InMission,
    InRecuperation,
    WaitingReportSignature,
}

// WIP - Ajouter i18n
impl RecruitStateEnum {
    pub fn get_description(&self) -> &'static str {
        match self {
            RecruitStateEnum::Available => "recruit_state_available",
            RecruitStateEnum::Injured => "recruit_state_injured",
            RecruitStateEnum::InMission => "recruit_state_in_mission",
            RecruitStateEnum::InRecuperation => "recruit_state_in_recuperation",
            RecruitStateEnum::WaitingReportSignature => "recruit_state_waiting_report_signature",
        }
    }
}

#[derive(Debug, Clone)]
pub enum MapImageEnum {
    CampagnTuto,
    // Campaign1,
    // Campaign2,
}

impl MapImageEnum {
    pub fn get_path(&self) -> &'static str {
        match self {
            MapImageEnum::CampagnTuto => "images/maps/map_tuto_v1.png",
        }
    }
}

#[derive(Debug, Clone)]
pub enum TextureAtlasLayoutEnum<'a> {
    Armor,
    Discussion,
    HudIcon,
    Scroll,
    Weapon,
    SpontaneousApplication,
    Loot(&'a ItemLootEnum),
    Item(&'a ItemEnum),
    Button,
    Recruit,
    Map,
    MapType,
    Ennemy,
    SleepButton,
}
