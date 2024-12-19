use crate::{
    audio::play_sound::play_sound,
    enums::SoundEnum,
    structs::{
        equipments::ItemEnum, general_structs::ItemInInventoryTrigger, player_stats::PlayerStats,
        recruits::SelectedRecruitForEquipment,
    },
    ui::ui_constants::WOOD_COLOR,
    utils::equip_recruit_inventory,
};
use bevy::{
    prelude::*,
    // window::CursorGrabMode
};

pub fn select_item_in_inventory(
    mut commands: Commands,
    my_assets: Res<AssetServer>,
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor, &ItemInInventoryTrigger),
        (Changed<Interaction>, With<Button>),
    >,
    _window: Single<&mut Window>,
    mut selected_recruit_for_equipment: ResMut<SelectedRecruitForEquipment>,
    mut player_stats: ResMut<PlayerStats>,
) {
    // let mut window = windows.single_mut();

    for (interaction, mut border_color, item_in_inventory_trigger) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(item) = &item_in_inventory_trigger.0 {
                    // modal_visible.0 = false;
                    border_color.0 = WOOD_COLOR;
                    let is_recruit_equiped = equip_recruit_inventory(
                        &mut selected_recruit_for_equipment,
                        item,
                        &mut player_stats,
                    );
                    if is_recruit_equiped {
                        match item {
                            ItemEnum::Armor(_) => {
                                play_sound(&my_assets, &mut commands, SoundEnum::EquipArmor);
                            }
                            ItemEnum::Scroll(_, _) => {
                                play_sound(&my_assets, &mut commands, SoundEnum::EquipScroll);
                            }
                            ItemEnum::Weapon(_) => {
                                play_sound(&my_assets, &mut commands, SoundEnum::EquipWeapon);
                            }
                        }
                    }
                }
            }
            Interaction::Hovered => {
                // window.cursor.icon = CursorIcon::Pointer;
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                // window.cursor.icon = CursorIcon::Default;
                border_color.0 = Color::BLACK;
            }
        }
    }
}
