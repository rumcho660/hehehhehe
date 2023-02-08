use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};

pub fn audio_game(audio: Res<Audio>,  asset_server: Res<AssetServer>){

    audio.play(asset_server.load("looperman-l-4278453-0321103-free-young-thug-synth-prod-jdabz.wav")).looped();
    audio.set_volume(0.1);
}
