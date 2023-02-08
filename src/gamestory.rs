use bevy:: prelude::*;
use crate::GameState;


#[derive(Component)]
pub struct Story;


#[derive(Component)]
pub struct ContinueButton;


#[derive(Component)]
pub struct GameStoryPlugin;


pub fn continue_button_clicked(
    mut commands: Commands,
    interactions: Query<&Interaction, (With<ContinueButton>, Changed<Interaction>)>,
    menu_root: Query<Entity, With<Story>>,
    mut game_state: ResMut<State<GameState>>,
) {
    for interaction in &interactions {
        if matches!(interaction, Interaction::Clicked) {
            let root_entity = menu_root.single();
            commands.entity(root_entity).despawn_recursive();

            game_state.set(GameState::MainMenu).unwrap();
        }
    }
}


pub fn setup_game_story(mut commands: Commands, asset_server: Res<AssetServer>){
    let start_button1 = spawn_continue_button(&mut commands, &asset_server, "[CONTINUE]", Color::BLACK);
    commands.entity(start_button1).insert(ContinueButton);


    let font_menu = asset_server.load("FFFFORWA.TTF");


    let text_style_menu = TextStyle {
        font: font_menu,
        font_size: 15.0,
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
    }).insert(Story)
        .with_children(|commands|{
            commands.spawn(TextBundle{
                style: Style{
                    align_self: AlignSelf::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                text: Text::from_section("The COVID-19 pandemic, also known as the coronavirus pandemic,
                                               The virus was so deadly that it took almost half of the human population
                                               But hope was not lost. In United Kingdom there was a special hospital.
                                               The only doctor there was a man of many talents, but the most interesting was his immortality
                                               For that reason he was tasked to sure the whole kingdom
                                               Will he succeed...", text_style_menu.clone()),
                ..default()
            });

        }).add_child(start_button1);
}



pub fn spawn_continue_button(commands: &mut Commands, asset_server: &AssetServer, text: &str, color: Color) -> Entity {
    let font_menu_button = asset_server.load("FFFFORWA.TTF");
    let button_style = TextStyle {
        font: font_menu_button,
        font_size: 20.0,
        color: Color::WHITE
    };


    commands.spawn(ButtonBundle {
        style: Style {
            align_self: AlignSelf::Center,
            align_content: AlignContent::Center,
            size: Size::new(Val::Percent(20.0), Val::Percent(10.0)),
            ..default()
        },
        background_color: color.into(),

        ..default()
    }).with_children(|comands| {
        comands.spawn(TextBundle {
            style: Style{
                align_self: AlignSelf::Baseline,
                ..default()
            },
            text: Text::from_section(text, button_style.clone()),
            ..default()
        });
    })
        .id()
}


impl Plugin for GameStoryPlugin{
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::GameStory)
            .with_system(setup_game_story))
            .add_system_set(SystemSet::on_update(GameState::GameStory)
                .with_system(continue_button_clicked));
    }
}