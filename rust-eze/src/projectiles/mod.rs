use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::enemies::Enemy;

mod powerups;
mod standard_projectile;
mod fire_projectile;
mod ice_projectile;
mod electric_projectile;
mod explosive_projectile;

pub struct ProjectilesPlugin;

impl Plugin for ProjectilesPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<powerups::PlayerPowerUpState>()
            .add_systems(Update, (
                spawn_projectiles,
                move_projectiles,
                check_projectile_collision,
                powerups::spawn_powerups,
                powerups::collect_powerups,
                powerups::update_powerup_timers,
                powerups::update_rotating_circle,
            ));
    }
}

#[derive(Component)]
pub struct Projectile {
    direction: Vec2,
    speed: f32,
    explosive: bool,
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
    player_powerup_state: Res<powerups::PlayerPowerUpState>,
) {
    let player_transform = player_query.single();
    let player_pos = player_transform.translation.xy();

    static mut LAST_SHOT: f32 = 0.0;
    let current_time = time.elapsed_seconds();
    if current_time - unsafe { LAST_SHOT } < 0.2 {
        return;
    }

    let mut direction = Vec2::ZERO;
    let mut projectile_type = None;

    if keyboard.just_pressed(KeyCode::ArrowUp) {
        direction = Vec2::new(0.0, 1.0);
        projectile_type = Some("standard");
    } else if keyboard.just_pressed(KeyCode::ArrowDown) {
        direction = Vec2::new(0.0, -1.0);
        projectile_type = Some("fire");
    } else if keyboard.just_pressed(KeyCode::ArrowLeft) {
        direction = Vec2::new(-1.0, 0.0);
        projectile_type = Some("ice");
    } else if keyboard.just_pressed(KeyCode::ArrowRight) {
        direction = Vec2::new(1.0, 0.0);
        projectile_type = Some("electric");
    } else if keyboard.just_pressed(KeyCode::Space) {
        direction = Vec2::new(1.0, 0.0);
        projectile_type = Some("explosive");
    }

    if direction != Vec2::ZERO && projectile_type.is_some() {
        unsafe { LAST_SHOT = current_time; }

        let speed = if player_powerup_state.faster_projectile_timer.is_some() {
            PROJECTILE_SPEED * 1.25
        } else {
            PROJECTILE_SPEED
        };

        let explosive = player_powerup_state.explosive_projectile_timer.is_some();

        match projectile_type.unwrap() {
            "standard" => standard_projectile::spawn(&mut commands, player_pos, direction, speed, explosive, &mut meshes, &mut materials),
            "fire" => fire_projectile::spawn(&mut commands, player_pos, direction, speed, explosive, &mut meshes, &mut materials),
            "ice" => ice_projectile::spawn(&mut commands, player_pos, direction, speed, explosive, &mut meshes, &mut materials),
            "electric" => electric_projectile::spawn(&mut commands, player_pos, direction, speed, explosive, &mut meshes, &mut materials),
            "explosive" => explosive_projectile::spawn(&mut commands, player_pos, direction, speed, explosive, &mut meshes, &mut materials),
            _ => {}
        }
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
    mut transform_queries: ParamSet<(
        Query<(Entity, &Transform, &Projectile), Without<Enemy>>,
        Query<(Entity, &Transform), (With<Enemy>, Without<Projectile>)>,
    )>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Acessa a query dos inimigos e coleta os dados
    let enemies: Vec<(Entity, Vec2)> = {
        let enemy_query = transform_queries.p1();
        enemy_query.iter().map(|(entity, transform)| (entity, transform.translation.xy())).collect()
    };

    // Acessa a query dos projéteis
    let mut projectile_query = transform_queries.p0();
    for (projectile_entity, projectile_transform, projectile) in projectile_query.iter_mut() {
        let projectile_pos = projectile_transform.translation.xy();

        for (enemy_entity, enemy_pos) in enemies.iter() {
            let distance = (projectile_pos - *enemy_pos).length();

            if distance < crate::config::GameConfig::ENEMY_SIZE + PROJECTILE_SIZE {
                commands.entity(projectile_entity).despawn();
                if projectile.explosive {
                    for (other_enemy_entity, other_enemy_pos) in enemies.iter() {
                        let area_distance = (projectile_pos - *other_enemy_pos).length();
                        if area_distance < 50.0 {
                            commands.entity(*other_enemy_entity).despawn();
                        }
                    }
                } else {
                    commands.entity(*enemy_entity).despawn();
                }
                powerups::try_spawn_powerup(&mut commands, *enemy_pos, &mut meshes, &mut materials);
                break;
            }
        }
    }
}