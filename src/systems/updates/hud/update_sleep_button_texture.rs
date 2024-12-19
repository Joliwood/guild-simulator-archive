use crate::{
    content::constants::{EVENING_TIME, MIDDAY_TIME},
    structs::general_structs::DayTime,
    ui::hud_folder::sleep_button::SleepButtonTrigger,
};
use bevy::prelude::*;

pub fn update_sleep_button_texture(
    day_time: Res<DayTime>,
    mut query: Query<(&mut ImageNode, &SleepButtonTrigger)>,
) {
    for (mut ui_image, _) in query.iter_mut() {
        let second_count = match day_time.second_count {
            0..MIDDAY_TIME => 0.,
            MIDDAY_TIME..EVENING_TIME => 1.,
            EVENING_TIME.. => 2.,
        };

        // Update the texture atlas index
        if let Some(texture_atlas) = ui_image.texture_atlas.as_mut() {
            texture_atlas.index = second_count as usize;
        }
    }
}
