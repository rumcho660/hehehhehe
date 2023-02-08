mod menu;
mod timer;
mod sound;
mod player;
mod gamestory;
mod enemy;
mod rooms;
mod gameover_score;

use bevy:: prelude::*;
use bevy::window::close_on_esc;
use bevy_kira_audio::AudioPlugin;
use crate::enemy::EnemyPlugin;
use crate::gameover_score::CounterPLugin;
use crate::gamestory::GameStoryPlugin;
use crate::menu::MenusPlugin;
use crate::player::{DeadChangeRoom, DeadCount, PlayerPlugin, Position};
use crate::rooms::RoomsPlugin;
use crate::sound::audio_game;
use crate::timer::{TimerEndGame, TimerPlugin};

const WINDOW_HEIGHT:f32 = 700.0;
const WINDOW_WIDTH:f32 = 900.0;

pub const SPRITE_ENEMY_SIZE: Vec2 = Vec2::new(32.0, 32.0);
pub const SPRITE_PLAYER_SIZE: Vec2 = Vec2::new(32.0, 32.0);
pub const SPRITE_WALL_SIZE: Vec2 = Vec2::new(32.0, 32.0);
pub const SPRITE_SYRINGE_SIZE: Vec2 = Vec2::new(16.0, 16.0);



#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub enum GameState {
    GameStory,
    MainMenu,
    MainRoom,
    Room1,
    Room2,
    GameOver
}
#[derive(Resource)]
pub struct TypeDeath(pub i32);

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .insert_resource(TimerEndGame(Timer::from_seconds(11.0, TimerMode::Once)))
        .insert_resource(DeadCount(0))
        .insert_resource(DeadChangeRoom(0))
        .insert_resource(TypeDeath(0))
        .insert_resource(Position{x:0.0, y:0.0})
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            window: WindowDescriptor{
                title: "Dr. Covid".to_string(),
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                ..default()
            },
            ..default()
        })
            .set(ImagePlugin::default_nearest()))
        .add_state(GameState::GameStory)
        .add_startup_system(setup_camera)
        .add_plugin(AudioPlugin)
        .add_plugin(GameStoryPlugin)
        .add_plugin(MenusPlugin)
        .add_plugin(TimerPlugin)
        .add_plugin(RoomsPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(CounterPLugin)
        .add_startup_system(audio_game)
        .add_system(close_on_esc)
        .run();

}


