use bevy::prelude::*;

use crate::systems::updates::init_rooms::ResetRoomTrigger;

#[allow(dead_code)]
pub fn room_store(my_assets: &Res<AssetServer>, commands: &mut Commands) {
    // let imager_handler: Handle<ImageNode> = my_assets.load("images/store.png");

    commands
        .spawn((
            Name::new("Store room"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ResetRoomTrigger,
        ))
        // ImageNode background node
        .with_children(|ui_container: &mut ChildBuilder| {
            ui_container.spawn((
                ImageNode {
                    image: my_assets.load("images/rooms/store/store.png"),
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                GlobalZIndex(-1),
            ));
        });
}
