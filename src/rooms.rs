use bevy:: prelude::*;
use bevy::math::Vec3Swizzles;
use bevy::sprite::collide_aabb::collide;
use crate::{GameState, SPRITE_PLAYER_SIZE, SPRITE_WALL_SIZE, TypeDeath, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::player::{Health, Player};


#[derive(Component)]
pub struct RoomsPlugin;

#[derive(Component)]
pub struct Room1;

#[derive(Component)]
pub struct Room2;


#[derive(Component)]
pub struct Wall;


#[derive(Component)]
pub struct Bed;


#[derive(Component)]
pub struct Something;


pub fn spawn_main_room(mut commands: Commands, asset_surver: Res<AssetServer>){
    let block = asset_surver.load("Block.png");


    let mut x1 = 0.0;
    let mut x2 = 0.0;
    let mut x3 = 0.0;
    let mut x4 = 0.0;
    while x1 < WINDOW_WIDTH/2.0 {
        commands.spawn(
            SpriteBundle {
                texture: block.clone(),
                transform: Transform{
                    translation: Vec3::new(x1, -WINDOW_HEIGHT/2.0 + 32.0, 1.0),
                    scale: Vec3::splat(2.0),
                    ..default()
                },
                ..default()
            }
        ).insert(Wall);
        x1 += 64.0;

        commands.spawn(
            SpriteBundle {
                texture: block.clone(),
                transform: Transform{
                    translation: Vec3::new(x2, -WINDOW_HEIGHT/2.0 + 32.0, 1.0),
                    scale: Vec3::splat(2.0),
                    ..default()
                },
                ..default()
            }
        ).insert(Wall);
        x2 -= 64.0;

        commands.spawn(
            SpriteBundle {
                texture: block.clone(),
                transform: Transform{
                    translation: Vec3::new(x3, WINDOW_HEIGHT/2.0 - 32.0, 1.0),
                    scale: Vec3::splat(2.0),
                    ..default()
                },
                ..default()
            }
        ).insert(Wall);
        x3 += 64.0;


        commands.spawn(
            SpriteBundle {
                texture: block.clone(),
                transform: Transform{
                    translation: Vec3::new(x4, WINDOW_HEIGHT/2.0 - 32.0, 1.0),
                    scale: Vec3::splat(2.0),
                    ..default()
                },
                ..default()
            }
        ).insert(Wall);
        x4 -= 64.0;
    }



    let mut y1 = 0.0;
    let mut y2 = 0.0;
    let mut y3 = 0.0;
    let mut y4 = 0.0;
    while y1 < WINDOW_HEIGHT/2.0{
        commands.spawn(
            SpriteBundle {
                texture: block.clone(),
                transform: Transform{
                    translation: Vec3::new(-WINDOW_WIDTH/2.0 + 32.0, y1, 0.0),
                    scale: Vec3::splat(2.0),
                    ..default()
                },
                ..default()
            }
        ).insert(Wall);

        y1 += 64.0;

        commands.spawn(
            SpriteBundle {
                texture: block.clone(),
                transform: Transform{
                    translation: Vec3::new(-WINDOW_WIDTH/2.0 + 32.0, y2, 0.0),
                    scale: Vec3::splat(2.0),
                    ..default()
                },
                ..default()
            }
        ).insert(Wall);

        y2 -= 64.0;

        commands.spawn(
            SpriteBundle {
                texture: block.clone(),
                transform: Transform{
                    translation: Vec3::new(WINDOW_WIDTH/2.0 - 32.0, y3, 0.0),
                    scale: Vec3::splat(2.0),
                    ..default()
                },
                ..default()
            }
        ).insert(Wall);

        y3 += 64.0;


        commands.spawn(
            SpriteBundle {
                texture: block.clone(),
                transform: Transform{
                    translation: Vec3::new(WINDOW_WIDTH/2.0 - 32.0, y4, 0.0),
                    scale: Vec3::splat(2.0),
                    ..default()
                },
                ..default()
            }
        ).insert(Wall);

        y4 -= 64.0;

    }

}


pub fn hitting_wall(mut app_state: ResMut<State<GameState>>, mut commands: Commands, mut query_player: Query<(Entity, &mut Transform, &mut Health), (With<Player>, Without<Wall>)>, query_wall: Query<(&Transform), (With<Wall>, Without<Player>)>,  mut type_dead: ResMut<TypeDeath> ){

    for (entity, mut transform_player, mut health) in query_player.iter_mut(){
        let player_scale = Vec2::from(transform_player.scale.xy());

        for  transform_wall in query_wall.iter()  {
            let wall_scale = Vec2::from(transform_wall.scale.xy());

            let collide = collide(
                transform_player.translation,
                SPRITE_PLAYER_SIZE * player_scale,
                transform_wall.translation,
                SPRITE_WALL_SIZE * wall_scale,
            );


            if let Some(_) = collide{
                type_dead.0 = 2;

                health.value -= 4;
                commands.entity(entity).despawn();
                app_state.set(GameState::GameOver);
            }
        }
    }
}


pub fn despawn_main_room(mut commands: Commands, query: Query< Entity, With<Wall>>){
    for walls in query.iter(){
        commands.entity(walls).despawn();
    }
}


pub fn spawn_room1(mut commands: Commands, asset_surver: Res<AssetServer>){



    let room1 = asset_surver.load("Room1.png");

    commands.spawn(
        SpriteBundle {
            texture: room1.clone(),
            transform: Transform{
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::splat(15.0),
                ..default()
            },
            ..default()
        }
    ).insert(Room1);

}


pub fn despawn_room1(mut commands: Commands, query: Query< Entity, With<Room1>>){
    for room1 in query.iter(){
        commands.entity(room1).despawn();
    }
}


pub fn spawn_room2(mut commands: Commands, asset_surver: Res<AssetServer>){



    let room2 = asset_surver.load("Room2.png");

    commands.spawn(
        SpriteBundle {
            texture: room2.clone(),
            transform: Transform::from_scale(Vec3::splat(26.0)),
            visibility: Visibility::VISIBLE,
            ..default()
        }
    ).insert(Room2);

}


pub fn despawn_room2(mut commands: Commands, query: Query< Entity, With<Room1>>){
    for room2 in query.iter(){
        commands.entity(room2).despawn();
    }
}


impl Plugin for RoomsPlugin{
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainRoom)
            .with_system(spawn_main_room))
            .add_system_set(SystemSet::on_update(GameState::MainRoom)
                .with_system(hitting_wall))
            .add_system_set(SystemSet::on_enter(GameState::Room1)
                .with_system(despawn_main_room)
                .with_system(spawn_room1))
            .add_system_set(SystemSet::on_enter(GameState::Room2)
                .with_system(despawn_room1)
                .with_system(spawn_room2))
            .add_system_set(SystemSet::on_enter(GameState::GameOver)
                .with_system(despawn_room2));
    }
}