use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;


// Power ups:
//
mod powerups;
mod standard_projectile;  // Normal
mod fire_projectile;      // fogo
mod ice_projectile;       // gelo
mod electric_projectile;  // eletrico 
mod explosive_projectile; // explosivo
//

pub struct ProjectilesPlugin;

impl Plugin for ProjectilesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (spawn_projectiles, move_projectiles, check_projectile_collision, powerups::spawn_powerups));
    }
}

#[derive(Component)]
pub struct Projectile {
    direction: Vec2,
    speed: f32,
}

const PROJECTILE_SIZE: f32 = 5.0;
const PROJECTILE_SPEED: f32 = 400.0;

fn spawn_projectiles(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, With<crate::player::Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    let player_transform = player_query.single();
    let player_pos = player_transform.translation.xy();

    static mut LAST_SHOT: f32 = 0.0;
    let current_time = time.elapsed_seconds();
    if current_time - unsafe { LAST_SHOT } < 0.2 {
        return;
    }

    let mut direction = Vec2::ZERO;
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        direction = Vec2::new(0.0, 1.0);
    } else if keyboard.just_pressed(KeyCode::ArrowDown) {
        direction = Vec2::new(0.0, -1.0);
    } else if keyboard.just_pressed(KeyCode::ArrowLeft) {
        direction = Vec2::new(-1.0, 0.0);
    } else if keyboard.just_pressed(KeyCode::ArrowRight) {
        direction = Vec2::new(1.0, 0.0);
    }

    if direction != Vec2::ZERO {
        unsafe { LAST_SHOT = current_time; }
        commands.spawn((
            Projectile {
                direction,
                speed: PROJECTILE_SPEED,
            },
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(PROJECTILE_SIZE)).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_xyz(player_pos.x, player_pos.y, 0.0),
                ..default()
            },
        ));
    }
}

fn move_projectiles(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &mut Transform, &Projectile)>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    let window_width = window.width();
    let window_height = window.height();
    let x_bound = (window_width - crate::config::GameConfig::WALL_THICKNESS) / 2.0;
    let y_bound = (window_height - crate::config::GameConfig::WALL_THICKNESS) / 2.0;

    for (entity, mut transform, projectile) in projectile_query.iter_mut() {
        let dx = projectile.direction.x * projectile.speed * time.delta_seconds();
        let dy = projectile.direction.y * projectile.speed * time.delta_seconds();
        transform.translation.x += dx;
        transform.translation.y += dy;

        if transform.translation.x.abs() > x_bound || transform.translation.y.abs() > y_bound {
            commands.entity(entity).despawn();
        }
    }
}

fn check_projectile_collision(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<Projectile>>,
    enemy_query: Query<(Entity, &Transform), (With<crate::enemies::Enemy>, Without<crate::player::Player>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (projectile_entity, projectile_transform) in projectile_query.iter() {
        let projectile_pos = projectile_transform.translation.xy();

        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            let enemy_pos = enemy_transform.translation.xy();
            let distance = (projectile_pos - enemy_pos).length();

            if distance < crate::config::GameConfig::ENEMY_SIZE + PROJECTILE_SIZE {
                commands.entity(projectile_entity).despawn();
                commands.entity(enemy_entity).despawn();
                powerups::try_spawn_powerup(&mut commands, enemy_pos, &mut meshes, &mut materials);
                break;
            }
        }
    }
}
