use bevy:: prelude::*;
use crate::{GameState, TypeDeath};
use crate::player::{DeadCount};

#[derive(Component)]
pub struct GameScore;

#[derive(Component)]
pub struct CounterPLugin;


pub fn game_score(mut commands: Commands, asset_server: Res<AssetServer>, deadcount: Res<DeadCount>, mut type_dead: ResMut<TypeDeath>){
    let mut type_of_dead;

    if type_dead.0 == 1 {
        type_of_dead = "Death by enemy";
    }
    else if type_dead.0 == 2 {
        type_of_dead = "Death by poisonous wall";
    }
    else {
        type_of_dead = "Death by timer";
    }


    let font_score = asset_server.load("FFFFORWA.TTF");


    let text_style_score = TextStyle {
        font: font_score,
        font_size: 30.0,
        color: Color::WHITE,
    };

    commands.spawn(NodeBundle{
        style: Style{
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_self: AlignSelf::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: BackgroundColor::from(Color::BLACK),
        ..default()
    }).insert(GameScore)
        .with_children(|commands|{
            commands.spawn(TextBundle{
                style: Style{
                    align_self: AlignSelf::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                text: Text::from_section("Game Over    Your score is:", text_style_score.clone()),
                ..default()
            });

        }).with_children(|commands|{
        commands.spawn(TextBundle{
            style: Style{
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            text: Text::from_section(deadcount.0.to_string(),  text_style_score.clone()),
            ..default()
        });

    }).with_children(|commands|{
        commands.spawn(TextBundle{
            style: Style{
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            text: Text::from_section(type_of_dead,  text_style_score.clone()),
            ..default()
        });

    });;
}


impl Plugin for CounterPLugin{
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::GameOver)
            .with_system(game_score));
    }
}