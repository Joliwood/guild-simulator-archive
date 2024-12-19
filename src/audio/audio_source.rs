// use crate::{enums::SoundEnum};
use crate::enums::SoundEnum;
use bevy::{audio::PlaybackMode, prelude::*};

pub fn audio_source(my_assets: Res<AssetServer>, mut commands: Commands) {
    // let audio = my_assets.load_sound(SoundEnum::SimpleHolidaysV3);
    let audio = my_assets.load(SoundEnum::get_path(&SoundEnum::SimpleHolidaysV3));
    commands.spawn((
        Name::new("Audio source"),
        AudioPlayer::<AudioSource>(audio),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            paused: true,
            speed: 1.0,
            ..default()
        },
    ));
}
