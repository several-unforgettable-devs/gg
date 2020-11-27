
use bevy::prelude::*;

pub fn setup_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {

    println!("Hello!");
    let music = asset_server.load("audio/BackgroundMusicLoop.mp3");
    audio.play(music);
    println!("Goodbye!");
}
