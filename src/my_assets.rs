// #![allow(dead_code)]
// use crate::{
//     enums::{MapImageEnum, SoundEnum},
//     structs::{equipments::ItemEnum, missions::ItemLootEnum},
// };
// use bevy::{asset, prelude::*};
// use bevy_asset_loader::asset_collection::AssetCollection;
// use serde::Deserialize;

// use std::ops::Deref;

// use bevy::prelude::Resource;
// use unic_langid::LanguageIdentifier;

use crate::{
    // enums::{MapImageEnum, SoundEnum},
    structs::{equipments::ItemEnum, missions::ItemLootEnum},
};

// #[derive(AssetCollection, Resource)]
// pub struct MyAssets {
//     // --- Recruits --- //
//     #[asset(path = "images/recruits/recruit_picture_atlas.png")]
//     pub recruit_picture_atlas: Handle<ImageNode>,
//     // #[asset(path = "images/recruits/recruit_card_win_percent.png")]
//     // pub recruit_card_win_percent: Handle<ImageNode>,

//     // --- Ennemies --- //
//     #[asset(path = "images/missions/ennemy_picture_atlas.png")]
//     pub ennemy_image_handle: Handle<ImageNode>,

//     // --- Equipments --- //
//     #[asset(path = "images/equipments/empty_inventory_slot.png")]
//     pub empty_inventory_slot: Handle<ImageNode>,
//     #[asset(path = "images/equipments/armors_atlas.png")]
//     pub armors_atlas: Handle<ImageNode>,
//     #[asset(path = "images/equipments/weapons_atlas.png")]
//     pub weapons_atlas: Handle<ImageNode>,
//     #[asset(path = "images/equipments/scrolls_atlas.png")]
//     pub scrolls_atlas: Handle<ImageNode>,

//     // --- HUD --- //
//     #[asset(path = "images/hud/art_v0_buttons.png")]
//     pub art_v0_buttons: Handle<ImageNode>,
//     #[asset(path = "images/hud/buttons_atlas.png")]
//     pub buttons_atlas: Handle<ImageNode>,
//     #[asset(path = "images/hud/gold.png")]
//     pub gold: Handle<ImageNode>,
//     #[asset(path = "images/hud/notification_atlas.png")]
//     pub notification_atlas: Handle<ImageNode>,
//     #[asset(path = "images/hud/play.png")]
//     pub _play: Handle<ImageNode>,
//     #[asset(path = "images/hud/hud3.png")]
//     pub hud: Handle<ImageNode>,
//     #[asset(path = "images/hud/hud_icon_atlas.png")]
//     pub hud_icon_atlas: Handle<ImageNode>,

//     // --- Rooms > Barrack --- //
//     #[asset(path = "images/rooms/barrack/barrack_background.png")]
//     pub barrack_background: Handle<ImageNode>,
//     #[asset(path = "images/rooms/barrack/inventory_container.png")]
//     pub inventory_container: Handle<ImageNode>,
//     #[asset(path = "images/rooms/barrack/recruit_frame.png")]
//     pub recruit_frame: Handle<ImageNode>,
//     #[asset(path = "images/rooms/barrack/recruit_infos.png")]
//     pub recruit_infos: Handle<ImageNode>,

//     // --- Rooms > Office ---//
//     #[asset(path = "images/rooms/office/office_room_background.png")]
//     pub office_background: Handle<ImageNode>,
//     #[asset(path = "images/rooms/office/mission_notification_document.png")]
//     pub mission_notification_document: Handle<ImageNode>,
//     #[asset(path = "images/rooms/office/talents_on_desk.png")]
//     pub talents_on_desk: Handle<ImageNode>,
//     #[asset(path = "images/rooms/office/notification_token_in_wood.png")]
//     pub notification_token_in_wood: Handle<ImageNode>,
//     #[asset(path = "images/rooms/office/set_of_keys.png")]
//     pub set_of_keys: Handle<ImageNode>,
//     #[asset(path = "images/rooms/office/daily_event_documents_on_desk.png")]
//     pub daily_event_documents_on_desk: Handle<ImageNode>,
//     #[asset(path = "images/rooms/office/daily_event_document.png")]
//     pub daily_event_document: Handle<ImageNode>,

//     // --- Rooms > Command room --- //
//     #[asset(path = "images/rooms/command_room/command_room_background.png")]
//     pub command_room_background: Handle<ImageNode>,
//     #[asset(path = "images/rooms/command_room/command_table.png")]
//     pub command_table: Handle<ImageNode>,
//     #[asset(path = "images/rooms/command_room/wood_box_container.png")]
//     pub wood_box_container: Handle<ImageNode>,
//     #[asset(path = "images/rooms/command_room/recruit_card.png")]
//     pub recruit_card: Handle<ImageNode>,

//     // --- Rooms > Store --- //
//     #[asset(path = "images/rooms/store/store.png")]
//     pub store: Handle<ImageNode>,

//     // --- Fonts --- //
//     #[asset(path = "fonts/FiraSans-Bold.ttf")]
//     pub fira_sans_bold: Handle<Font>,

//     // --- Sounds --- //
//     #[asset(path = "sounds/Simple-Holidays-V3.ogg")] // Change path as necessary
//     pub simple_holidays_v3: Handle<AudioSource>,
//     #[asset(path = "sounds/book_throw_down.ogg")] // Add more sounds as needed
//     pub book_throw_down: Handle<AudioSource>,
//     #[asset(path = "sounds/cockrel_morning.ogg")]
//     pub cockrel_morning: Handle<AudioSource>,
//     #[asset(path = "sounds/equip_armor.ogg")]
//     pub equip_armor: Handle<AudioSource>,
//     #[asset(path = "sounds/equipment_equip.ogg")]
//     pub equipment_equip: Handle<AudioSource>,
//     #[asset(path = "sounds/equip_scroll.ogg")]
//     pub equip_scroll: Handle<AudioSource>,
//     #[asset(path = "sounds/equip_weapon.ogg")]
//     pub equip_weapon: Handle<AudioSource>,
//     #[asset(path = "sounds/keys_removed_from_door.ogg")]
//     pub keys_removed_from_door: Handle<AudioSource>,
//     #[asset(path = "sounds/paper_touch.ogg")]
//     pub paper_touch: Handle<AudioSource>,
//     #[asset(path = "sounds/pencil_sign.ogg")]
//     pub pencil_sign: Handle<AudioSource>,
//     #[asset(path = "sounds/picking_golds.ogg")]
//     pub picking_golds: Handle<AudioSource>,
//     #[asset(path = "sounds/dead_male.ogg")]
//     pub dead_male: Handle<AudioSource>,

//     // --- Maps --- //
//     #[asset(path = "images/maps/map_tuto.png")]
//     pub map_tuto: Handle<ImageNode>,
//     #[asset(path = "images/maps/map_description.png")]
//     pub map_description: Handle<ImageNode>,
//     #[asset(path = "images/maps/map_atlas.png")]
//     pub map_atlas: Handle<ImageNode>,
//     #[asset(path = "images/maps/map_card.png")]
//     pub map_card: Handle<ImageNode>,
//     #[asset(path = "images/maps/map_type_atlas.png")]
//     pub map_type_atlas: Handle<ImageNode>,
//     #[asset(path = "images/maps/limited_time.png")]
//     pub limited_time: Handle<ImageNode>,

//     // --- Missions --- //
//     #[asset(path = "images/missions/c1_mission_1.png")]
//     pub c1_mission_1: Handle<ImageNode>,
//     #[asset(path = "images/missions/c1_mission_2.png")]
//     pub c1_mission_2: Handle<ImageNode>,
//     #[asset(path = "images/missions/c1_mission_3.png")]
//     pub c1_mission_3: Handle<ImageNode>,
//     #[asset(path = "images/missions/c1_mission_4.png")]
//     pub c1_mission_4: Handle<ImageNode>,
//     #[asset(path = "images/missions/c1_mission_5.png")]
//     pub c1_mission_5: Handle<ImageNode>,
//     #[asset(path = "images/missions/c1_mission_6.png")]
//     pub c1_mission_6: Handle<ImageNode>,
//     #[asset(path = "images/missions/recap_guild_scroll.png")]
//     pub recap_guild_scroll: Handle<ImageNode>,

//     // --- Daily events --- //
//     #[asset(path = "images/daily_events/daily_discussions_atlas.png")]
//     pub daily_discussions_atlas: Handle<ImageNode>,
//     #[asset(path = "images/daily_events/daily_spontaneous_applications_atlas.png")]
//     pub daily_spontaneous_applications_atlas: Handle<ImageNode>,
// }

// impl MyAssets {
//     pub fn get_item_atlas_path(&self, item: &ItemEnum) -> Handle<ImageNode> {
//         return match item {
//             ItemEnum::Weapon(_) => self.weapons_atlas.clone(),
//             ItemEnum::Armor(_) => self.armors_atlas.clone(),
//             ItemEnum::Scroll(_, _) => self.scrolls_atlas.clone(),
//         };
//     }

//     pub fn get_item_loot_atlas_path(&self, item: &ItemLootEnum) -> Handle<ImageNode> {
//         return match item {
//             ItemLootEnum::Weapon(_) => self.weapons_atlas.clone(),
//             ItemLootEnum::Armor(_) => self.armors_atlas.clone(),
//             ItemLootEnum::Scroll(_) => self.scrolls_atlas.clone(),
//         };
//     }

//     pub fn get_map_type_atlas(&self) -> Handle<ImageNode> {
//         return self.map_type_atlas.clone();
//     }

//     pub fn load_sound(&self, sound_enum: SoundEnum) -> Handle<AudioSource> {
//         return match sound_enum {
//             SoundEnum::BookThrowDown => self.book_throw_down.clone(),
//             SoundEnum::CockrelMorning => self.cockrel_morning.clone(),
//             SoundEnum::EquipArmor => self.equip_armor.clone(),
//             SoundEnum::EquipmentEquip => self.equipment_equip.clone(),
//             SoundEnum::EquipScroll => self.equip_scroll.clone(),
//             SoundEnum::EquipWeapon => self.equip_weapon.clone(),
//             SoundEnum::KeysRemovedFromDoor => self.keys_removed_from_door.clone(),
//             SoundEnum::PaperTouch => self.paper_touch.clone(),
//             SoundEnum::PencilSign => self.pencil_sign.clone(),
//             SoundEnum::PickingGolds => self.picking_golds.clone(),
//             SoundEnum::SimpleHolidaysV3 => self.simple_holidays_v3.clone(),
//             SoundEnum::DeadMale => self.dead_male.clone(),
//         };
//     }

//     pub fn get_image_map(&self, map_enum: MapImageEnum) -> Handle<ImageNode> {
//         return match map_enum {
//             MapImageEnum::CampagnTuto => self.map_tuto.clone(),
//         };
//     }

//     pub fn get_mission_image(&self, mission_id: u16) -> Handle<ImageNode> {
//         return match mission_id {
//             0 => self.c1_mission_1.clone(),
//             1 => self.c1_mission_2.clone(),
//             2 => self.c1_mission_3.clone(),
//             3 => self.c1_mission_4.clone(),
//             4 => self.c1_mission_5.clone(),
//             5 => self.c1_mission_6.clone(),
//             6 => self.c1_mission_6.clone(),
//             _ => self.c1_mission_1.clone(),
//         };
//     }
// }

pub const FONT_FIRA: &str = "fonts/FiraSans-Bold.ttf";

pub fn get_item_atlas_path(item: &ItemEnum) -> String {
    match item {
        ItemEnum::Weapon(_) => "images/equipments/weapons_atlas.png".into(),
        ItemEnum::Armor(_) => "images/equipments/armors_atlas.png".into(),
        ItemEnum::Scroll(_, _) => "images/equipments/scrolls_atlas.png".into(),
    }
}

pub fn get_item_loot_atlas_path(item: &ItemLootEnum) -> String {
    match item {
        ItemLootEnum::Weapon(_) => "images/equipments/weapons_atlas.png".into(),
        ItemLootEnum::Armor(_) => "images/equipments/armors_atlas.png".into(),
        ItemLootEnum::Scroll(_) => "images/equipments/scrolls_atlas.png".into(),
    }
}

pub fn get_mission_image(mission_id: u16) -> String {
    match mission_id {
        0 => "images/missions/c1_mission_1.png".into(),
        1 => "images/missions/c1_mission_1.png".into(),
        2 => "images/missions/c1_mission_2.png".into(),
        3 => "images/missions/c1_mission_3.png".into(),
        4 => "images/missions/c1_mission_4.png".into(),
        5 => "images/missions/c1_mission_5.png".into(),
        6 => "images/missions/c1_mission_6.png".into(),
        _ => "images/missions/c1_mission_1.png".into(),
    }
}

// #[derive(Resource)]
// pub struct Locales(pub Vec<LanguageIdentifier>);

// impl Locales {
//     pub fn index(&self, locale: &LanguageIdentifier) -> usize {
//         self.iter()
//             .position(|item| item == locale)
//             .expect("index not found")
//     }
// }

// impl Deref for Locales {
//     type Target = Vec<LanguageIdentifier>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
