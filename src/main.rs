mod t;
mod menu;
mod audio;

use crate::t::{destroy_timer_el, timer_til_game_end, TimerEndGame};

use std::time::Duration;
use bevy:: prelude::*;
use bevy::app::AppExit;
use bevy::text::Text2dBundle;
use crate::menu::{GameState, MenusPlugin, setup_main_menu};
use bevy::window::close_on_esc;
use bevy_kira_audio::AudioPlugin;
use crate::audio::audio_game;
use crate::menu::GameState::MainMenu;


fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .insert_resource(TimerEndGame(Timer::from_seconds(10.0, TimerMode::Once)))
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            window: WindowDescriptor{
                title: "Dr. Covid".to_string(),
                ..default()
            },
            ..default()
        }))
        .add_plugin(AudioPlugin)
        .add_state(GameState::MainGame)
        .add_startup_system(setup)
        .add_system(close_on_esc)

        .add_system_set(
            SystemSet::on_update(GameState::MainGame)
                .with_system(audio_game)
                .with_system(timer_til_game_end)
        )
        .add_system_set(
            SystemSet::on_update(GameState::GameOver)
                .with_system(destroy_timer_el)
        )
        .run();
}


