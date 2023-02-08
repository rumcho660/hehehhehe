use std::time::Duration;
use bevy::app::AppExit;
use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::text::Text2dSize;
use crate::menu::GameState;

#[derive(Resource)]
pub struct TimerEndGame (pub Timer);

#[derive(Component)]
pub struct TimerItem;


pub(crate) fn timer_til_game_end(mut timer_end: ResMut<TimerEndGame>, mut _exit: EventWriter<AppExit>, mut commands: Commands, asset_server: Res<AssetServer>, mut app_state: ResMut<State<GameState>>) {

    let font1 = asset_server.load("ARCADECLASSIC.TTF");
    let font2 = asset_server.load("FFFFORWA.TTF");

    let text_style = TextStyle {
        font: font1,
        font_size: 20.0,
        color: Color::RED,
    };

    let text_style_over = TextStyle {
        font: font2,
        font_size: 90.0,
        color: Color::BLACK,
    };



    timer_end.0.tick(Duration::from_secs_f32(0.01));

    if timer_end.0.elapsed_secs() == 1.0{
        let mut _one = commands.spawn(TextBundle {
            text: Text::from_section("**********", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);

    }

    else if timer_end.0.elapsed_secs() == 2.0{

        let mut _two = commands.spawn(TextBundle {
            text: Text::from_section("********************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);
    }

    else if timer_end.0.elapsed_secs() == 3.0{

        let mut _three = commands.spawn(TextBundle {
            text: Text::from_section("******************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);
    }

    else if timer_end.0.elapsed_secs() == 4.0{
        let mut _four = commands.spawn(TextBundle {
            text: Text::from_section("****************************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);
    }

    else if timer_end.0.elapsed_secs() == 5.0{
        let mut _five = commands.spawn(TextBundle {
            text: Text::from_section("**************************************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);

    }


    else if timer_end.0.elapsed_secs() == 6.0{
        let mut _6 = commands.spawn(TextBundle {
            text: Text::from_section("************************************************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);

    }

    else if timer_end.0.elapsed_secs() == 7.0{
        let mut _seven = commands.spawn(TextBundle {
            text: Text::from_section("**********************************************************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);

    }

    else if timer_end.0.elapsed_secs() == 8.0{
        let mut  _eight = commands.spawn(TextBundle {
            text: Text::from_section("********************************************************************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);

    }

    else if timer_end.0.elapsed_secs() == 9.0{
        let mut _nine = commands.spawn(TextBundle {
            text: Text::from_section("******************************************************************************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);

    }

    else if timer_end.0.elapsed_secs() == 10.0{
        let mut _ten = commands.spawn(TextBundle {
            text: Text::from_section("***************************************************************************************************************Game Over", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);
        app_state.push(GameState::GameOver).expect("TODO: panic message");
    }

    else if timer_end.0.elapsed_secs() == 11.0{
        app_state.push(GameState::GameOver).expect("TODO: panic message");
        commands.spawn(Text2dBundle {
            text: Text::from_section("Time ran out\
            Try agan by restarting the game", text_style_over.clone()),
            transform: Transform{
                translation: Default::default(),
                rotation: Default::default(),
                scale: Default::default()
            },

            text_2d_size: Text2dSize{
                size: Vec2{ x: 30.0, y: 30.0 }
            },
            ..default()
        });



    }


}




pub(crate) fn destroy_timer_el(mut commands: Commands, query: Query<Entity, With<TimerItem>>){
    for timer in query.iter() {
        commands.entity(timer).despawn_recursive();
    }
}