use std::time::Duration;
use bevy:: prelude::*;
use bevy::app::AppExit;
use crate::{GameState, TypeDeath};

#[derive(Resource)]
pub struct TimerEndGame (pub Timer);

#[derive(Component)]
pub struct TimerItem;

#[derive(Component)]
pub struct TimerPlugin;

pub fn timer_til_game_end(mut type_dead: ResMut<TypeDeath>, mut timer_end: ResMut<TimerEndGame>, mut _exit: EventWriter<AppExit>, mut commands: Commands, asset_server: Res<AssetServer>, mut app_state: ResMut<State<GameState>>){



    let font1 = asset_server.load("ARCADECLASSIC.TTF");

    let text_style = TextStyle {
        font: font1,
        font_size: 20.0,
        color: Color::RED,
    };


    timer_end.0.tick(Duration::from_secs_f32(0.02));



    if timer_end.0.elapsed_secs() == 1.0{
        commands.spawn(Text2dBundle {
            text: Text::from_section("**********", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);
    }

    else if timer_end.0.elapsed_secs() == 2.0{

        commands.spawn(Text2dBundle {
            text: Text::from_section("********************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);
    }

    else if timer_end.0.elapsed_secs() == 3.0{

        commands.spawn(Text2dBundle {
            text: Text::from_section("******************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);
    }

    else if timer_end.0.elapsed_secs() == 4.0{
        commands.spawn(Text2dBundle {
            text: Text::from_section("****************************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);
    }

    else if timer_end.0.elapsed_secs() == 5.0{
        commands.spawn(Text2dBundle {
            text: Text::from_section("**************************************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);

    }


    else if timer_end.0.elapsed_secs() == 6.0{
        commands.spawn(Text2dBundle {
            text: Text::from_section("************************************************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);

    }

    else if timer_end.0.elapsed_secs() == 7.0{
        commands.spawn(Text2dBundle {
            text: Text::from_section("**********************************************************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);

    }

    else if timer_end.0.elapsed_secs() == 8.0{
        commands.spawn(Text2dBundle {
            text: Text::from_section("********************************************************************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);

    }

    else if timer_end.0.elapsed_secs() == 9.0{
        commands.spawn(Text2dBundle {
            text: Text::from_section("******************************************************************************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);

    }

    else if timer_end.0.elapsed_secs() == 10.0{
        commands.spawn(Text2dBundle {
            text: Text::from_section("*************************************************************************************************************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);




    }
    else if timer_end.0.elapsed_secs() == 11.0{
        type_dead.0 = 3;
        app_state.set(GameState::GameOver).expect("Problem with pushing GameOver state");
    }


}

pub fn destroy_timer_el(mut commands: Commands, query: Query<Entity, With<TimerItem>>){
    for timer in query.iter() {
        commands.entity(timer).despawn_recursive();
    }
}


impl Plugin for TimerPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::GameOver)
                .with_system(destroy_timer_el))
            .add_system_set(SystemSet::on_update(GameState::Room1)
                .with_system(timer_til_game_end))
            .add_system_set(SystemSet::on_update(GameState::Room2)
                .with_system(timer_til_game_end));
    }
}

