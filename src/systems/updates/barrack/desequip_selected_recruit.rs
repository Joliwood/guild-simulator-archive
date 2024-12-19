use crate::{
    structs::{
        equipments::ItemEnum, general_structs::ItemInInventoryTrigger, player_stats::PlayerStats,
        recruits::SelectedRecruitForEquipment,
    },
    ui::rooms::barrack::recruit_overview_folder::recruit_infos_folder::{
        armor_button::DesequipArmorButtonTrigger, weapon_button::DesequipWeaponButtonTrigger,
    },
};
use bevy::prelude::*;

pub fn desequip_selected_recruit_armor(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BorderColor,
            &ItemInInventoryTrigger,
            &DesequipArmorButtonTrigger,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut player_stats: ResMut<PlayerStats>,
    mut selected_recruit: ResMut<SelectedRecruitForEquipment>,
) {
    for (interaction, _border_color, _item_trigger, desequip_armor_trigger) in
        interaction_query.iter_mut()
    {
        if *interaction == Interaction::Pressed {
            if let Some(armor) = &desequip_armor_trigger.0 {
                player_stats.add_item(ItemEnum::Armor(armor.clone()));
                player_stats.desequip_item_to_recruit(
                    selected_recruit.get_id().unwrap(),
                    &ItemEnum::Armor(armor.clone()),
                );
                selected_recruit.desequip_armor();
            }
        }
    }
}

pub fn desequip_selected_recruit_weapon(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BorderColor,
            &ItemInInventoryTrigger,
            &DesequipWeaponButtonTrigger,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut player_stats: ResMut<PlayerStats>,
    mut selected_recruit: ResMut<SelectedRecruitForEquipment>,
) {
    for (interaction, _border_color, _item_trigger, desequip_weapon_trigger) in
        interaction_query.iter_mut()
    {
        if *interaction == Interaction::Pressed {
            if let Some(weapon) = &desequip_weapon_trigger.0 {
                player_stats.add_item(ItemEnum::Weapon(weapon.clone()));
                player_stats.desequip_item_to_recruit(
                    selected_recruit.get_id().unwrap(),
                    &ItemEnum::Weapon(weapon.clone()),
                );
                selected_recruit.desequip_weapon();
            }
        }
    }
}
