use bevy::prelude::*;

pub fn play_sound(asset_server: &Res<AssetServer>, audio: &Res<Audio>, sound_effect_name: &str) {
    let sound_effect = asset_server.load(sound_effect_name);
    audio.play(sound_effect);
}

pub fn setup_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music = asset_server.load("audio/BackgroundMusicLoop.mp3");
    audio.play(music);
}
