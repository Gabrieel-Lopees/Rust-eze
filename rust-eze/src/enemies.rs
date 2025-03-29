use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::math::Vec3Swizzles;
use rand::Rng;
use crate::player::Player;
use crate::config::GameConfig;
use crate::rooms::{CurrentRoom, RoomId};

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_enemies)
            .add_systems(Update, move_enemies);
    }
}

#[derive(Component)]
pub struct Enemy {
    pub room: RoomId,
}

fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    current_room: Res<CurrentRoom>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.single();
    let window_width = window.width();
    let window_height = window.height();

    let x_bound = (window_width - GameConfig::WALL_THICKNESS - GameConfig::ENEMY_SIZE) / 2.0;
    let y_bound = (window_height - GameConfig::WALL_THICKNESS - GameConfig::ENEMY_SIZE) / 2.0;

    let mut rng = rand::thread_rng();
    for _ in 0..3 {
        let x = rng.gen_range(-x_bound..x_bound);
        let y = rng.gen_range(-y_bound..y_bound);

        commands.spawn((
            Enemy {
                room: current_room.id,
            },
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(GameConfig::ENEMY_SIZE)).into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
        ));
    }
}

fn move_enemies(
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let window = window_query.single();
    let window_width = window.width();
    let window_height = window.height();

    let x_bound = (window_width - GameConfig::WALL_THICKNESS - GameConfig::ENEMY_SIZE) / 2.0;
    let y_bound = (window_height - GameConfig::WALL_THICKNESS - GameConfig::ENEMY_SIZE) / 2.0;

    let player_transform = player_query.single();
    let player_pos = player_transform.translation.xy();

    for mut transform in enemy_query.iter_mut() {
        let enemy_pos = transform.translation.xy();
        let direction = (player_pos - enemy_pos).normalize_or_zero();

        let dx = direction.x * GameConfig::ENEMY_SPEED * time.delta_seconds();
        let dy = direction.y * GameConfig::ENEMY_SPEED * time.delta_seconds();

        let mut new_x = transform.translation.x + dx;
        let mut new_y = transform.translation.y + dy;

        new_x = new_x.clamp(-x_bound, x_bound);
        new_y = new_y.clamp(-y_bound, y_bound);

        transform.translation.x = new_x;
        transform.translation.y = new_y;
    }
}