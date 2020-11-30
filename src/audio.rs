use bevy::prelude::*;

use crate::cooldown::*;

pub fn play_sound(asset_server: &Res<AssetServer>, audio: &Res<Audio>, sound_effect_name: &str) {
    let sound_effect = asset_server.load(sound_effect_name);
    audio.play(sound_effect);
}


pub const SOUNDTRACK_DURATION: f64 = 0.8;
pub fn play_soundtrack(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    time: Res<Time>,
    mut soundtrack_cooldown: Local<Cooldown>,
) {
    if soundtrack_cooldown.over(&time) {
        let music = asset_server.load("audio/Asteroid_Game_Soundtrack.mp3");
        audio.play(music);
        soundtrack_cooldown.reset(&time, SOUNDTRACK_DURATION);
    }
}
