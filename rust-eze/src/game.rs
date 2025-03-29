use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::sprite::MaterialMesh2dBundle;
use rand::Rng;
use crate::player::Player;
use crate::enemies::Enemy;
use crate::config::GameConfig;
use crate::lives::Lives;
use crate::ui::{GameOverText, Score};
use crate::rooms::CurrentRoom;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (check_collision, handle_game_over));
    }
}

pub fn check_collision(
    mut lives_query: Query<&mut Lives>,
    mut player_query: Query<(Entity, &mut Transform), (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<(Entity, &mut Transform, &Enemy), (With<Enemy>, Without<Player>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    current_room: Res<CurrentRoom>,
    mut game_over_text: Query<&mut Style, With<GameOverText>>,
) {
    let (_player_entity, mut player_transform) = player_query.single_mut();
    let player_pos = player_transform.translation.xy();

    for (_enemy_entity, enemy_transform, enemy) in enemy_query.iter() {
        if enemy.room != current_room.id {
            continue;
        }

        let enemy_pos = enemy_transform.translation.xy();
        let distance = (player_pos - enemy_pos).length();

        if distance < GameConfig::COLLISION_DISTANCE {
            let mut lives = lives_query.single_mut();
            let game_over = lives.lose_life();

            if game_over {
                let mut style = game_over_text.single_mut();
                style.display = Display::Flex;
            } else {
                player_transform.translation = Vec3::new(0.0, 0.0, 0.0);

                let window = window_query.single();
                let window_width = window.width();
                let window_height = window.height();
                let x_bound = (window_width - GameConfig::WALL_THICKNESS - GameConfig::ENEMY_SIZE) / 2.0;
                let y_bound = (window_height - GameConfig::WALL_THICKNESS - GameConfig::ENEMY_SIZE) / 2.0;
                let mut rng = rand::thread_rng();

                for (_, mut enemy_transform, enemy) in enemy_query.iter_mut() {
                    if enemy.room != current_room.id {
                        continue;
                    }
                    let mut new_x;
                    let mut new_y;
                    loop {
                        new_x = rng.gen_range(-x_bound..x_bound);
                        new_y = rng.gen_range(-y_bound..y_bound);
                        let distance_from_player = Vec2::new(new_x, new_y).distance(Vec2::new(0.0, 0.0));
                        if distance_from_player > 100.0 {
                            break;
                        }
                    }
                    enemy_transform.translation = Vec3::new(new_x, new_y, 0.0);
                }
            }

            break;
        }
    }
}

pub fn handle_game_over(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut lives_query: Query<&mut Lives>,
    player_query: Query<(Entity, &Transform), (With<Player>, Without<Enemy>)>,
    enemy_query: Query<(Entity, &Transform, &Enemy), (With<Enemy>, Without<Player>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    current_room: Res<CurrentRoom>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_over_text: Query<&mut Style, With<GameOverText>>,
    mut time: ResMut<Time<Virtual>>,
    mut score: ResMut<Score>,
) {
    let lives = lives_query.single();
    if lives.count() > 0 {
        time.unpause();
        return;
    }

    time.pause();

    if keyboard.just_pressed(KeyCode::Enter) {
        let mut style = game_over_text.single_mut();
        style.display = Display::None;

        let (player_entity, _) = player_query.single();
        commands.entity(player_entity).despawn();
        for (enemy, _, enemy_data) in enemy_query.iter() {
            if enemy_data.room == current_room.id {
                commands.entity(enemy).despawn();
            }
        }

        commands.spawn((
            Player,
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(GameConfig::PLAYER_SIZE)).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
        ));

        let window = window_query.single();
        let window_width = window.width();
        let window_height = window.height();

        let x_bound = (window_width - GameConfig::WALL_THICKNESS - GameConfig::ENEMY_SIZE) / 2.0;
        let y_bound = (window_height - GameConfig::WALL_THICKNESS - GameConfig::ENEMY_SIZE) / 2.0;

        let mut rng = rand::thread_rng();
        for _ in 0..3 {
            let mut new_x;
            let mut new_y;
            loop {
                new_x = rng.gen_range(-x_bound..x_bound);
                new_y = rng.gen_range(-y_bound..y_bound);
                let distance_from_player = Vec2::new(new_x, new_y).distance(Vec2::new(0.0, 0.0));
                if distance_from_player > 100.0 {
                    break;
                }
            }
            commands.spawn((
                Enemy {
                    room: current_room.id,
                },
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::new(GameConfig::ENEMY_SIZE)).into(),
                    material: materials.add(ColorMaterial::from(Color::GREEN)),
                    transform: Transform::from_xyz(new_x, new_y, 0.0),
                    ..default()
                },
            ));
        }

        let mut lives = lives_query.single_mut();
        lives.reset();
        score.value = 0;

        time.unpause();
    }
}
