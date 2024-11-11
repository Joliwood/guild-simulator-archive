use crate::{
    enums::{ColorPaletteEnum, RecruitStateEnum},
    structs::{
        general_structs::UniqueId,
        recruits::{RecruitStats, SelectedRecruitForEquipment},
    },
};
use bevy::prelude::*;

/// Select the recruit when the button is pressed
pub fn select_recruit_for_equipment_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &UniqueId, &RecruitStats),
        Changed<Interaction>,
    >,
    mut windows: Query<&mut Window>,
    mut selected_recruit_for_equipment: ResMut<SelectedRecruitForEquipment>,
) {
    let _window = windows.single_mut();

    for (interaction, mut background_color, unique_id, recruit) in &mut interaction_query {
        let recruit_state = recruit.clone().state;
        if unique_id.0 == "recruit_button"
            && recruit_state != RecruitStateEnum::InMission
            && recruit_state != RecruitStateEnum::WaitingReportSignature
        {
            match *interaction {
                Interaction::Pressed => {
                    selected_recruit_for_equipment.0 = Some(recruit.clone());
                }
                Interaction::Hovered => {
                    // window.cursor.icon = CursorIcon::Pointer;
                    *background_color = BackgroundColor(ColorPaletteEnum::Brown.as_color());
                }
                Interaction::None => {
                    // window.cursor.icon = CursorIcon::Default;
                    *background_color = BackgroundColor(ColorPaletteEnum::Wood.as_color());
                }
            }
        }
    }
}
