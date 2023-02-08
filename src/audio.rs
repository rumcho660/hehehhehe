use std::ptr::addr_of_mut;
use bevy:: prelude::*;
use bevy_kira_audio::{Audio, AudioControl};




pub(crate) fn audio_game(mut commands: Commands, audio: Res<Audio>,  asset_server: Res<AssetServer>){
    let audio1= asset_server.load("mixkit-arcade-retro-background-219.wav");

    audio.play(audio1);
}
