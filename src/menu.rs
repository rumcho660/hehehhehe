use bevy::app::AppExit;
use bevy::prelude::*;
use crate::GameState;


#[derive(Component)]
pub struct GameStory;

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct MainGame;

#[derive(Component)]
pub struct GameOver;

#[derive(Component)]
pub struct StartButton;


#[derive(Component)]
pub struct QuitButton;




#[derive(Component)]
pub struct MenusPlugin;


pub(crate) fn start_button_clicked(
    mut commands: Commands,
    interactions: Query<&Interaction, (With<StartButton>, Changed<Interaction>)>,
    menu_root: Query<Entity, With<MainMenu>>,
    mut game_state: ResMut<State<GameState>>,
) {
    for interaction in &interactions {
        if matches!(interaction, Interaction::Clicked) {
            let root_entity = menu_root.single();
            commands.entity(root_entity).despawn_recursive();

            game_state.set(GameState::MainRoom).unwrap();
        }
    }
}

pub(crate) fn quit_button_clicked(
    interactions: Query<&Interaction, (With<QuitButton>, Changed<Interaction>)>,
    mut exit: EventWriter<AppExit>,
) {
    for interaction in &interactions {
        if matches!(interaction, Interaction::Clicked) {
            exit.send(AppExit);
        }
    }
}



pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>){
    let start_button1 = spawn_button(&mut commands, &asset_server, "Start", Color::BLACK);
    commands.entity(start_button1).insert(StartButton);

    let start_button2 = spawn_button(&mut commands, &asset_server, "Quit", Color::BLACK);
    commands.entity(start_button2).insert(QuitButton);


    let font_menu = asset_server.load("FFFFORWA.TTF");


    let text_style_menu = TextStyle {
        font: font_menu,
        font_size: 50.0,
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
    }).insert(MainMenu)
        .with_children(|commands|{
            commands.spawn(TextBundle{
                style: Style{
                    align_self: AlignSelf::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                text: Text::from_section("Dr.Covid", text_style_menu.clone()),
                ..default()
            });

        }).add_child(start_button1)
        .add_child(start_button2);
}






pub fn spawn_button(commands: &mut Commands, asset_server: &AssetServer, text: &str, color: Color) -> Entity {
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
            flex_direction: FlexDirection::RowReverse,
            size: Size::new(Val::Percent(20.0), Val::Percent(20.0)),
            ..default()
        },
        background_color: color.into(),

        ..default()
    }).with_children(|comands| {
        comands.spawn(TextBundle {
            style: Style{
                align_self: AlignSelf::Center,
                margin: UiRect::all(Val::Percent(3.0)),
                ..default()
            },
            text: Text::from_section(text, button_style.clone()),
            ..default()
        });
    })
        .id()
}


impl Plugin for MenusPlugin{
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu)
            .with_system(setup_menu))
            .add_system_set(SystemSet::on_update(GameState::MainMenu)
                .with_system(start_button_clicked)
                .with_system(quit_button_clicked));
    }
}

