use bevy:: prelude::*;
use bevy::sprite::collide_aabb::{collide};
use crate::player::{Health, Player, Velosity};
use crate::{GameState, SPRITE_ENEMY_SIZE, SPRITE_PLAYER_SIZE, TypeDeath};
use bevy::math::Vec3Swizzles;
use crate::player::Damage;

const SPEED_ENEMY: f32 = 200.0;

#[derive(Component)]
pub struct Enemy;


#[derive(Component)]
pub struct EnemyPlugin;


#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimerEnemy(pub Timer);




pub fn spawn_enemy_wave1(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>){
    let texture_handle = asset_server.load("Enemy_final.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 1, 5, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);



    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform{
                translation: Vec3::new(300.0, 100.0, 1.0),
                scale: Vec3::splat(2.5),
                ..default()
            },
            visibility: Visibility::VISIBLE,
            ..default()

        },
        AnimationTimerEnemy(Timer::from_seconds(0.1, TimerMode::Repeating)),
    )).insert(Enemy)
        .insert(Health{value: 5})
        .insert(Damage{value: 2})
        .insert(Velosity{x: 0.0, y: 0.0});


    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform{
                translation: Vec3::new(-300.0, 100.0, 1.0),
                scale: Vec3::splat(2.5),
                ..default()
            },
            visibility: Visibility::VISIBLE,
            ..default()

        },
        AnimationTimerEnemy(Timer::from_seconds(0.1, TimerMode::Repeating)),
    )).insert(Enemy)
        .insert(Health{value: 5})
        .insert(Damage{value: 2})
        .insert(Velosity{x: 0.0, y: 0.0});



    /*commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform{
                translation: Vec3::new(230.0, -200.0, 1.0),
                scale: Vec3::splat(2.5),
                ..default()
            },
            visibility: Visibility::VISIBLE,
            ..default()

        },
        AnimationTimerEnemy(Timer::from_seconds(0.1, TimerMode::Repeating)),
    )).insert(Enemy)
        .insert(Health{value: 5})
        .insert(Damage{value: 2})
        .insert(Velosity{x: 0.0, y: 0.0});


    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform{
                translation: Vec3::new(-200.0, -100.0, 1.0),
                scale: Vec3::splat(2.5),
                ..default()
            },
            visibility: Visibility::VISIBLE,
            ..default()

        },
        AnimationTimerEnemy(Timer::from_seconds(0.1, TimerMode::Repeating)),
    )).insert(Enemy)
        .insert(Health{value: 5})
        .insert(Damage{value: 2})
        .insert(Velosity{x: 0.0, y: 0.0});



    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform{
                translation: Vec3::new(200.0, 200.0, 1.0),
                scale: Vec3::splat(2.5),
                ..default()
            },
            visibility: Visibility::VISIBLE,
            ..default()

        },
        AnimationTimerEnemy(Timer::from_seconds(0.1, TimerMode::Repeating)),
    )).insert(Enemy)
        .insert(Health{value: 5})
        .insert(Damage{value: 2})
        .insert(Velosity{x: 0.0, y: 0.0});
       */
}


pub fn despawn_enemy(mut commands: Commands, mut query: Query<Entity, With<Enemy>>){
    for enemy in query.iter_mut(){
        commands.entity(enemy).despawn();
    }
}



pub fn move_enemy(mut query: Query<(&mut Velosity, &mut Transform), (With<Enemy>, Without<Player>)>,
                  player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
                  time: Res<Time>,
                  texture_atlases: Res<Assets<TextureAtlas>>,
                  mut query_animation: Query<(&mut AnimationTimerEnemy, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>) {

    for (mut velocity, mut enemy_pos) in query.iter_mut() {
        for player_pos in player_query.iter() {

            if enemy_pos.translation.x > player_pos.translation.x {
                velocity.x = -0.01;
            }

            if enemy_pos.translation.x < player_pos.translation.x {
                velocity.x = 0.01;
            }

            if enemy_pos.translation.y > player_pos.translation.y {
                velocity.y = -0.01;
            }

            if enemy_pos.translation.y < player_pos.translation.y {
                velocity.y = 0.01;
            }


            for (mut timer, mut sprite, texture_atlas_handle) in &mut query_animation {
                timer.tick(time.delta());
                if timer.just_finished() {
                    let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                    sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
                }
            }

            let mut translation = &mut enemy_pos.translation;


            translation.x += velocity.x * SPEED_ENEMY;
            translation.y += velocity.y * SPEED_ENEMY;

        }

    }
}



pub fn enemy_attack(mut commands: Commands, mut query_player: Query<(Entity, &mut Health, &Transform), With<Player>>, query_enemy: Query<(&Damage, &Transform), With<Enemy>>, mut app_state: ResMut<State<GameState>>, mut type_dead: ResMut<TypeDeath> ){

    for (player, mut health,  transform_player) in query_player.iter_mut(){
        let player_scale = Vec2::from(transform_player.scale.xy());

        for (damage, transform_enemy) in query_enemy.iter()  {
            let enemy_scale = Vec2::from(transform_enemy.scale.xy());

            let collide = collide(
                transform_player.translation,
                SPRITE_PLAYER_SIZE * player_scale,
                transform_enemy.translation,
                SPRITE_ENEMY_SIZE * enemy_scale,
            );


            if let Some(_) = collide{
                health.value -= damage.value;


                if health.value == 0{
                    type_dead.0 = 1;

                    commands.entity(player).despawn();
                    app_state.set(GameState::GameOver).expect("error in gameover in player.rs");
                }
            }
        }
    }
}



impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Room1)
            .with_system(spawn_enemy_wave1))
            .add_system_set(SystemSet::on_update(GameState::Room1)
                .with_system(move_enemy)
                .with_system(enemy_attack))
            .add_system_set(SystemSet::on_enter(GameState::Room2)
                .with_system(spawn_enemy_wave1))
            .add_system_set(SystemSet::on_update(GameState::Room2)
                .with_system(move_enemy)
                .with_system(enemy_attack))
            .add_system_set(SystemSet::on_enter(GameState::GameOver)
                .with_system(despawn_enemy));


    }
}