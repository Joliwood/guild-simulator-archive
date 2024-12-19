use crate::{
    structs::general_structs::TutoMessagesModalVisible,
    ui::hud_folder::mayor_notification_toast::NotificationToastTrigger,
};
use bevy::prelude::*;

pub fn open_tuto_message(
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor, &NotificationToastTrigger),
        Changed<Interaction>,
    >,
    mut tuto_message_modal_visibility: ResMut<TutoMessagesModalVisible>,
) {
    for (interaction, mut border_color, _) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                tuto_message_modal_visibility.0 = true;
            }
            Interaction::Hovered => {
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                border_color.0 = Color::BLACK;
            }
        }
    }
}
