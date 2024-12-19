use crate::ui::rooms::command_room::map_card::SelectMapTrigger;
use bevy::prelude::*;

/// Select the recruit when the button is pressed
pub fn select_map(
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor),
        (Changed<Interaction>, With<Button>, With<SelectMapTrigger>),
    >,
) {
    for (interaction, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {}
            Interaction::Hovered => {
                // window.cursor.icon = CursorIcon::Pointer;
                *border_color = BorderColor(Color::WHITE);
            }
            Interaction::None => {
                // window.cursor.icon = CursorIcon::Default;
                *border_color = BorderColor(Color::BLACK);
            }
        }
    }
}
