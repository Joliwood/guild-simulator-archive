use crate::{
    structs::{
        general_structs::{TutoDoneModalVisible, TutoMessagesModalVisible},
        player_stats::{PlayerStats, TutoMessages},
    },
    ui::modals::{
        tuto_done_modal_folder::tuto_done_modal::CloseTutoDoneMessageTrigger,
        tuto_messages::tuto_message_modal::CloseTutoMessageTrigger,
    },
};
use bevy::prelude::*;

pub fn close_tuto_message(
    mut query: Query<&Interaction, (Changed<Interaction>, With<CloseTutoMessageTrigger>)>,
    mut tuto_message_modal_visibility: ResMut<TutoMessagesModalVisible>,
    mut tuto_messages: ResMut<TutoMessages>,
    mut player_stats: ResMut<PlayerStats>,
) {
    for interaction in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                tuto_message_modal_visibility.0 = false;
                tuto_messages.remove_first_tuto_message(&mut player_stats);
                if tuto_messages.get_first_tuto_message().is_some() {
                    tuto_message_modal_visibility.0 = true;
                }
            }
            Interaction::Hovered => {
                // window.cursor.icon = CursorIcon::Pointer;
            }
            Interaction::None => {
                // window.cursor.icon = CursorIcon::Default;
            }
        }
    }
}

pub fn close_tuto_done_message(
    mut query: Query<&Interaction, (Changed<Interaction>, With<CloseTutoDoneMessageTrigger>)>,
    mut player_stats: ResMut<PlayerStats>,
    mut tuto_done_modal_visibility: ResMut<TutoDoneModalVisible>,
) {
    for interaction in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                player_stats.tuto.is_tuto_completed = true;
                tuto_done_modal_visibility.0 = false;
            }
            Interaction::Hovered => {
                // window.cursor.icon = CursorIcon::Pointer;
            }
            Interaction::None => {
                // window.cursor.icon = CursorIcon::Default;
            }
        }
    }
}
