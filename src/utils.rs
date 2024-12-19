use crate::{
    enums::{RecruitStateEnum, TextureAtlasLayoutEnum},
    structs::{
        equipments::ItemEnum,
        general_structs::{
            DailyEventsModalVisible, Ennemy, MissionModalVisible, MissionReportsModalVisible,
            TutoMessagesModalVisible,
        },
        missions::{ItemLootEnum, MissionReport, MissionReports, Missions},
        player_stats::PlayerStats,
        recruits::{
            RecruitInventory, RecruitStats, SelectedRecruitForEquipment, SelectedRecruitForMission,
        },
    },
};
use bevy::{math::UVec2, prelude::ResMut, sprite::TextureAtlasLayout};

// /// Determines the new room based on the given direction and current player stats.
// ///
// /// ## Parameters
// /// - `player_stats`: The current player stats containing the current room.
// /// - `direction`: The direction in which the room change is requested.
// ///
// /// ## Returns
// /// The new room enum corresponding to the direction.
// ///
// /// ## Permanently the store room has been removed for V0
// pub fn get_new_room(
//     player_stats: &ResMut<PlayerStats>,
//     direction: RoomDirectionEnum,
//     mission_modal_visibility: &mut ResMut<MissionModalVisible>,
//     mission_reports_modal_visibility: &mut ResMut<MissionReportsModalVisible>,
//     selected_recruit_for_mission: &mut ResMut<SelectedRecruitForMission>,
//     daily_events_modal_visibility: &mut ResMut<DailyEventsModalVisible>,
// ) -> Option<RoomEnum> {
//     // Close any open modals
//     mission_modal_visibility.0 = false;
//     mission_reports_modal_visibility.0 = false;
//     daily_events_modal_visibility.0 = false;
//     selected_recruit_for_mission.0 = None;

//     match player_stats.room {
//         RoomEnum::Office => match direction {
//             RoomDirectionEnum::Right => Some(RoomEnum::Barrack),
//             // RoomDirectionEnum::Left => Some(RoomEnum::Store),
//             RoomDirectionEnum::Left => None,
//             RoomDirectionEnum::Bottom => Some(RoomEnum::CommandRoom),
//             RoomDirectionEnum::Top => None,
//         },
//         RoomEnum::Barrack => match direction {
//             RoomDirectionEnum::Right => None,
//             RoomDirectionEnum::Left => Some(RoomEnum::Office),
//             RoomDirectionEnum::Bottom => None,
//             RoomDirectionEnum::Top => None,
//         },
//         RoomEnum::Store => match direction {
//             RoomDirectionEnum::Right => Some(RoomEnum::Office),
//             RoomDirectionEnum::Left => None,
//             RoomDirectionEnum::Bottom => None,
//             RoomDirectionEnum::Top => None,
//         },
//         RoomEnum::CommandRoom => match direction {
//             RoomDirectionEnum::Right => None,
//             RoomDirectionEnum::Left => None,
//             RoomDirectionEnum::Bottom => None,
//             RoomDirectionEnum::Top => Some(RoomEnum::Office),
//         },
//     }
// }

pub fn reset_modals_visibility(
    mission_modal_visibility: &mut ResMut<MissionModalVisible>,
    mission_reports_modal_visibility: &mut ResMut<MissionReportsModalVisible>,
    daily_events_modal_visibility: &mut ResMut<DailyEventsModalVisible>,
    selected_recruit_for_mission: &mut ResMut<SelectedRecruitForMission>,
    tuto_messages_modal_visibility: &mut ResMut<TutoMessagesModalVisible>,
) {
    mission_modal_visibility.0 = false;
    mission_reports_modal_visibility.0 = false;
    daily_events_modal_visibility.0 = false;
    selected_recruit_for_mission.0 = None;
    tuto_messages_modal_visibility.0 = false;
}

/// ## Calculates the victory percentage of a mission based on the global points
///
/// - If recruit points = ennemy points / 2 => 0%.
/// - If recruit points = ennemy points x 2 => 100%.
/// - If recruits points  = ennemy points => 50%.
///
/// ## Returns
/// The victory percentage of the mission.
pub fn get_victory_percentage(recruit_real_power: f32, enemy_real_power: f32) -> f32 {
    let loose_guaranteed = enemy_real_power / 2.;
    let victory_guaranteed = enemy_real_power * 2.;
    let percent_per_point_lower_range = 50.0 / loose_guaranteed;
    let percent_per_point_upper_range = 50.0 / enemy_real_power;

    if recruit_real_power <= loose_guaranteed {
        return 0.;
    } else if recruit_real_power > loose_guaranteed && recruit_real_power < enemy_real_power {
        return (recruit_real_power - loose_guaranteed) * percent_per_point_lower_range;
    } else if recruit_real_power == enemy_real_power {
        return 50.;
    } else if recruit_real_power > enemy_real_power && recruit_real_power < victory_guaranteed {
        return 100. - (victory_guaranteed - recruit_real_power) * percent_per_point_upper_range;
    } else {
        // recruit_global_power >= enemy_real_power * 2
        return 100.;
    }
}

/// Use the % of victory to randomly determine if the mission is a success or a failure
///
/// ## Parameters
/// - `percent_of_victory`: The % of victory of the mission (0 to 100)
///
/// It will generate a random number between 0 and 1 and compare it to the % of victory divided by 100
pub fn is_mission_success(percent_of_victory: f32) -> bool {
    let random_number: f32 = rand::random();
    return random_number <= percent_of_victory / 100.;
}

/// Get the xp earned on a mission based on the level of the ennemy
///
/// This function will can be upgraded to the recruit level, here some futur examples :
/// - recruit level < 5 ennemy level -> / 10 xp
/// - recruit level > 3 ennemy level -> x3 xp
pub fn get_xp_earned(level: u8) -> u32 {
    return level as u32 * 20;
}

#[allow(dead_code)]
pub fn format_ron_equipments_for_display(ron_data: &str) -> String {
    // Use a regex to format the RON output
    return ron_data
        .replace(":", ": ")
        .replace("),", "},")
        .replace(")", "}")
        .replace("Armors(", "  {")
        .replace("Armors(", "{")
        .replace("items:", "\"items\":")
        .replace("Weapon(", "  {")
        .replace("Weapons(", "  {")
        .replace("Weapons(", "{")
        .replace("Weapons(", "{");
}

/// Get the image atlas index of an item
///
/// Has to be updated each time the design will evolve
pub fn get_item_image_atlas_index(item: &ItemEnum) -> u16 {
    return match item {
        ItemEnum::Weapon(weapon) => weapon.image_atlas_index,
        ItemEnum::Armor(armor) => armor.image_atlas_index,
        ItemEnum::Scroll(scroll, _) => scroll.image_atlas_index,
    };
}

#[allow(dead_code)]
/// Get the tooltip description of an item
///
/// For now, only supports texts
pub fn get_item_tooltip_description(item: &ItemEnum) -> String {
    match item {
        ItemEnum::Weapon(weapon) => {
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

            return description;
        }
        ItemEnum::Armor(armor) => {
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

            return description;
        }
        ItemEnum::Scroll(scroll, quantity) => {
            let mut description = format!("{}\n\n{:?}", scroll.name, scroll.bonus);

            if let Some(attack) = scroll.attack {
                description.push_str(&format!("\n{}: {}", t!("attack"), attack));
            }

            if let Some(defense) = scroll.defense {
                description.push_str(&format!("\n{}: {}", t!("defense"), defense));
            }

            description.push_str(&format!("\n\n{}: {}", t!("quantity"), quantity));
            description.push_str(&format!("\n\n{}: {} G", t!("price"), scroll.price));

            return description;
        }
    };
}

/// Calculate the price range of an item based on its price
///
/// The item will can be sold more or less depending on merchant affinities
pub fn calculate_price_range(price: u16) -> (u16, u16) {
    let lower_range = (price as f32 * 0.95) as u16;
    let upper_range = (price as f32 * 1.05) as u16;

    return (lower_range, upper_range);
}

pub fn equip_recruit_inventory(
    selected_recruit_for_equipment: &mut ResMut<SelectedRecruitForEquipment>,
    item: &ItemEnum,
    player_stats: &mut ResMut<PlayerStats>,
) -> bool {
    match item {
        ItemEnum::Weapon(_weapon) => {
            // Add the weapon to the selected recruit inventory weapon slot
            // + Delete from the player_stat inventory

            let selected_recruit_for_equipment_inventory: RecruitInventory =
                selected_recruit_for_equipment.get_inventory();
            let selected_recruit_for_equipment_weapon =
                selected_recruit_for_equipment_inventory.weapon;
            let selected_recruit_for_equipment_id = selected_recruit_for_equipment.get_id();

            if let Some(recruit_id) = selected_recruit_for_equipment_id {
                // If the recruit already has a weapon, we put it back in the player inventory
                if let Some(weapon_already_equipped) = selected_recruit_for_equipment_weapon {
                    player_stats.add_item(ItemEnum::Weapon(weapon_already_equipped));
                }

                // In all cases we ->
                // Equip the weapon to the recruit
                player_stats.equip_item_to_recruit(recruit_id, item);

                // Update the selected recruit (must be synchronized with player_stats > recruits)
                selected_recruit_for_equipment.0 = player_stats.get_recruit_by_id(recruit_id);

                // Remove the item from the player inventory
                player_stats.remove_item(item);

                return true;
            }
            return false;
        }
        ItemEnum::Armor(_armor) => {
            let selected_recruit_for_equipment_inventory: RecruitInventory =
                selected_recruit_for_equipment.get_inventory();
            let selected_recruit_for_equipment_armor =
                selected_recruit_for_equipment_inventory.armor;
            let selected_recruit_for_equipment_id = selected_recruit_for_equipment.get_id();

            if selected_recruit_for_equipment_id.is_some() {
                if selected_recruit_for_equipment_armor.is_none() {
                    player_stats
                        .equip_item_to_recruit(selected_recruit_for_equipment_id.unwrap(), item);
                    selected_recruit_for_equipment.0 =
                        player_stats.get_recruit_by_id(selected_recruit_for_equipment_id.unwrap());
                    player_stats.remove_item(item);
                    return true;
                }

                if selected_recruit_for_equipment_armor.is_some() {
                    let selected_recruit_for_equipment_item =
                        ItemEnum::Armor(selected_recruit_for_equipment_armor.clone().unwrap());
                    player_stats.add_item(selected_recruit_for_equipment_item);
                    player_stats
                        .equip_item_to_recruit(selected_recruit_for_equipment_id.unwrap(), item);
                    selected_recruit_for_equipment.0 =
                        player_stats.get_recruit_by_id(selected_recruit_for_equipment_id.unwrap());
                    player_stats.remove_item(item);
                    return true;
                }
            }

            return false;
        }
        ItemEnum::Scroll(scroll, _quantity) => {
            let selected_recruit_for_equipment_inventory: RecruitInventory =
                selected_recruit_for_equipment.get_inventory();
            let selected_recruit_for_equipment_scrolls =
                selected_recruit_for_equipment_inventory.scrolls;

            if selected_recruit_for_equipment_scrolls.len() == 3 {
                return false;
            }

            let selected_recruit_for_equipment_id = selected_recruit_for_equipment.get_id();
            let scroll_id = scroll.id;

            if selected_recruit_for_equipment_id.is_some() {
                player_stats
                    .equip_item_to_recruit(selected_recruit_for_equipment_id.unwrap(), item);

                selected_recruit_for_equipment.0 =
                    player_stats.get_recruit_by_id(selected_recruit_for_equipment_id.unwrap());

                player_stats.remove_one_scroll_from_inventory(scroll_id);
                return true;
            }

            return false;
        }
    }
}

pub fn finish_mission(
    player_stats: &mut ResMut<PlayerStats>,
    mission_id: u16,
    missions: &mut Missions,
    percent_of_victory: f32,
    mission_reports: &mut ResMut<MissionReports>,
) {
    if let Some(recruit_id) = missions.get_recruit_send_id_by_mission_id(mission_id) {
        player_stats.update_state_of_recruit(recruit_id, RecruitStateEnum::WaitingReportSignature);

        let is_mission_sucess = is_mission_success(percent_of_victory);
        let mission_ennemy_level = missions.get_mission_enemmy_level_by_id(mission_id);
        if mission_ennemy_level.is_none() {
            return;
        }

        let mut new_mission_report = MissionReport {
            percent_of_victory: percent_of_victory as u32,
            recruit_id,
            mission_id,
            success: is_mission_sucess,
            experience_gained: None,
            golds_gained: None,
            mission_ids_to_unlock: vec![],
            loots: vec![],
        };

        if is_mission_sucess {
            let xp_earned = get_xp_earned(mission_ennemy_level.unwrap());
            new_mission_report.experience_gained = Some(xp_earned);

            let golds_earned = missions.get_golds_earned_by_mission_id(mission_id).unwrap() as i32;

            if let Some(recruit) = player_stats.get_recruit_by_id(recruit_id) {
                let gold_recruit_multiplicator = recruit
                    .recruit_inventory
                    .get_gold_multiplicator_from_scroll_bonus();

                new_mission_report.golds_gained =
                    Some((golds_earned as f32 * gold_recruit_multiplicator).round() as i32);

                if let Some(mission) = missions.get_mission_by_id(&mission_id) {
                    new_mission_report.calculate_loots(mission.loots.clone(), &recruit);
                }
            }
        }

        // Create a new mission_report
        mission_reports.add_mission_report(new_mission_report);
    }
}

pub fn sort_recruits_by_total_power(mut recruits: Vec<RecruitStats>) -> Vec<RecruitStats> {
    recruits.sort_by_key(|recruit| {
        std::cmp::Reverse((recruit.get_total_stats().0 + recruit.get_total_stats().1) as i32)
    });
    return recruits;
}

pub fn get_layout(texture_atlas_layout_enum: TextureAtlasLayoutEnum) -> TextureAtlasLayout {
    match texture_atlas_layout_enum {
        TextureAtlasLayoutEnum::Weapon => {
            return TextureAtlasLayout::from_grid(
                UVec2::new(400, 400),
                6,
                1,
                Some(UVec2::new(0, 0)),
                Some(UVec2::new(0, 0)),
            );
        }
        TextureAtlasLayoutEnum::Armor => {
            return TextureAtlasLayout::from_grid(
                UVec2::new(400, 400),
                6,
                1,
                Some(UVec2::new(0, 0)),
                Some(UVec2::new(0, 0)),
            )
        }
        TextureAtlasLayoutEnum::Scroll => {
            return TextureAtlasLayout::from_grid(
                UVec2::new(400, 400),
                9,
                1,
                Some(UVec2::new(0, 0)),
                Some(UVec2::new(0, 0)),
            )
        }
        TextureAtlasLayoutEnum::HudIcon => {
            return TextureAtlasLayout::from_grid(
                UVec2::new(500, 500),
                8,
                1,
                Some(UVec2::new(0, 0)),
                Some(UVec2::new(0, 0)),
            )
        }
        TextureAtlasLayoutEnum::Discussion => {
            return TextureAtlasLayout::from_grid(
                UVec2::new(800, 350),
                1,
                10,
                Some(UVec2::new(0, 0)),
                Some(UVec2::new(0, 0)),
            )
        }
        TextureAtlasLayoutEnum::SpontaneousApplication => {
            return TextureAtlasLayout::from_grid(
                UVec2::new(800, 350),
                1,
                2,
                Some(UVec2::new(0, 0)),
                Some(UVec2::new(0, 0)),
            )
        }
        TextureAtlasLayoutEnum::Loot(item) => match item {
            ItemLootEnum::Weapon(_) => {
                return TextureAtlasLayout::from_grid(
                    UVec2::new(400, 400),
                    6,
                    1,
                    Some(UVec2::new(0, 0)),
                    Some(UVec2::new(0, 0)),
                )
            }
            ItemLootEnum::Armor(_) => {
                return TextureAtlasLayout::from_grid(
                    UVec2::new(400, 400),
                    6,
                    1,
                    Some(UVec2::new(0, 0)),
                    Some(UVec2::new(0, 0)),
                )
            }
            ItemLootEnum::Scroll(_) => {
                return TextureAtlasLayout::from_grid(
                    UVec2::new(400, 400),
                    9,
                    1,
                    Some(UVec2::new(0, 0)),
                    Some(UVec2::new(0, 0)),
                )
            }
        },
        TextureAtlasLayoutEnum::Item(item) => match item {
            ItemEnum::Weapon(_) => {
                return TextureAtlasLayout::from_grid(
                    UVec2::new(400, 400),
                    6,
                    1,
                    Some(UVec2::new(0, 0)),
                    Some(UVec2::new(0, 0)),
                )
            }
            ItemEnum::Armor(_) => {
                return TextureAtlasLayout::from_grid(
                    UVec2::new(400, 400),
                    6,
                    1,
                    Some(UVec2::new(0, 0)),
                    Some(UVec2::new(0, 0)),
                )
            }
            ItemEnum::Scroll(_, _) => {
                return TextureAtlasLayout::from_grid(
                    UVec2::new(400, 400),
                    8,
                    1,
                    Some(UVec2::new(0, 0)),
                    Some(UVec2::new(0, 0)),
                )
            }
        },
        TextureAtlasLayoutEnum::Button => {
            return TextureAtlasLayout::from_grid(
                UVec2::new(16, 16),
                5,
                6,
                Some(UVec2::new(0, 0)),
                Some(UVec2::new(0, 0)),
            )
        }
        TextureAtlasLayoutEnum::Recruit => {
            return TextureAtlasLayout::from_grid(
                UVec2::new(200, 400),
                7,
                1,
                Some(UVec2::new(0, 0)),
                Some(UVec2::new(0, 0)),
            )
        }
        TextureAtlasLayoutEnum::Map => {
            return TextureAtlasLayout::from_grid(
                UVec2::new(270, 200),
                1,
                2,
                Some(UVec2::new(0, 0)),
                Some(UVec2::new(0, 0)),
            )
        }
        TextureAtlasLayoutEnum::MapType => {
            return TextureAtlasLayout::from_grid(
                UVec2::new(100, 100),
                4,
                1,
                Some(UVec2::new(0, 0)),
                Some(UVec2::new(0, 0)),
            )
        }
        TextureAtlasLayoutEnum::Ennemy => {
            return TextureAtlasLayout::from_grid(
                UVec2::new(200, 200),
                6,
                1,
                Some(UVec2::new(0, 0)),
                Some(UVec2::new(0, 0)),
            )
        }
        TextureAtlasLayoutEnum::SleepButton => {
            return TextureAtlasLayout::from_grid(
                UVec2::new(400, 400),
                3,
                1,
                Some(UVec2::new(0, 0)),
                Some(UVec2::new(0, 0)),
            )
        }
    }
}

/// ## Explaination of the calcul :
///
/// - a = b * (1 - (c / (c + b)))
/// - real_attack = attack * (1 - (defense_opponent / (defense_opponent + attack))).
///
/// If the recruit has no defense, so the attack will not be diluted.
///
/// If the recruit has way more in defense than the ennemy in attack, the result will follow a logarithmic curve
///
/// ## Example 1 :
/// - attack = 75
/// - defense_opponent = 24
/// - result will be -> 56.81
///
/// ## Example 2
/// - attack = 75
/// - defense_opponent = 68
/// - result will be -> 39.33
///
/// ## Example 3
/// - attack = 75
/// - defense_opponent = 4
/// - result will be -> 71.20
///
/// ## Example 4
/// - attack = 75
/// - defense_opponent = 141
/// - result will be -> 26.04
///
/// ## Example 5
/// - attack = 180
/// - defense_opponent = 180
/// - result will be -> 90 (50% of the attack)
///
/// ## Example 6
/// - attack = 180
/// - defense_opponent = 0
/// - result will be -> 180 (100% of the attack)
fn calculate_real_attack(attack: f32, defense: f32) -> f32 {
    return attack * (1. - (defense / (defense + attack)));
}

pub fn calculate_fight(recruit: &RecruitStats, ennemy: &Ennemy) -> f32 {
    let recruit_total_stats = recruit.get_total_stats();

    let recruit_real_attack =
        calculate_real_attack(recruit_total_stats.0 as f32, ennemy.defense as f32);

    let ennemy_real_attack =
        calculate_real_attack(ennemy.attack as f32, recruit_total_stats.1 as f32);

    let fight_percentage = get_victory_percentage(recruit_real_attack, ennemy_real_attack);

    return fight_percentage;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_victory_percentage() {
        assert_eq!(get_victory_percentage(5., 20.), 0.);
        assert_eq!(get_victory_percentage(10., 20.), 0.);
        assert_eq!(get_victory_percentage(20., 20.), 50.);
        assert_eq!(get_victory_percentage(25., 20.), 62.5);
        assert_eq!(get_victory_percentage(40., 20.), 100.);
        assert_eq!(get_victory_percentage(50., 20.), 100.);
    }

    #[test]
    fn test_calculate_real_attack() {
        assert_eq!(calculate_real_attack(75., 24.), 56.81818);
        assert_eq!(calculate_real_attack(75., 68.), 39.335663);
        assert_eq!(calculate_real_attack(75., 4.), 71.20253);
        assert_eq!(calculate_real_attack(75., 141.), 26.041666);
        assert_eq!(calculate_real_attack(180., 180.), 90.);
        assert_eq!(calculate_real_attack(180., 0.), 180.);
        assert_eq!(calculate_real_attack(112., 551.), 18.92006);
    }
}
