use crate::{
    audio::play_sound::play_sound,
    enums::SoundEnum,
    structs::{
        equipments::ItemEnum, general_structs::UniqueId, player_stats::PlayerStats,
        recruits::SelectedRecruitForEquipment,
    },
    systems::systems_constants::HOVERED_BUTTON,
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
        (
            &Interaction,
            &mut BackgroundColor,
            &UniqueId,
            &mut BorderColor,
            &ItemEnum,
        ),
        // Changed<Interaction>,
        (Changed<Interaction>, With<Button>),
    >,
    _window: Single<&mut Window>,
    mut selected_recruit_for_equipment: ResMut<SelectedRecruitForEquipment>,
    mut player_stats: ResMut<PlayerStats>,
) {
    // let mut window = windows.single_mut();

    for (interaction, mut background_color, unique_id, mut border_color, item) in
        &mut interaction_query
    {
        if unique_id.0 == "item_in_inventory" {
            match *interaction {
                Interaction::Pressed => {
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
                Interaction::Hovered => {
                    // window.cursor.icon = CursorIcon::Pointer;
                    *background_color = HOVERED_BUTTON.into();
                    border_color.0 = Color::WHITE;
                }
                Interaction::None => {
                    // window.cursor.icon = CursorIcon::Default;
                    *background_color = BackgroundColor(WOOD_COLOR);
                    border_color.0 = Color::BLACK;
                }
            }
        }
    }
}
