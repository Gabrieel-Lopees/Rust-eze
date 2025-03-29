use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use rand::Rng;
use crate::player::Player;
use crate::lives::Lives;
use crate::enemies::Enemy;

#[derive(Component)]
pub struct PowerUp {
    pub powerup_type: PowerUpType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerUpType {
    FasterProjectile,
    ExplosiveProjectile,
    RotatingCircle,
    ExtraLife,
}

#[derive(Component)]
pub struct RotatingCircle {
    pub angle: f32,
    pub speed: f32,
    pub radius: f32,
    pub timer: Timer,
}

#[derive(Resource, Default)]
pub struct PlayerPowerUpState {
    pub faster_projectile_timer: Option<Timer>,
    pub explosive_projectile_timer: Option<Timer>,
}

pub fn spawn_powerups(
    mut commands: Commands,
    powerup_query: Query<&PowerUp>,
    enemy_query: Query<&crate::enemies::Enemy>,
    current_room: Res<crate::rooms::CurrentRoom>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let enemy_count = enemy_query.iter().filter(|enemy| enemy.room == current_room.id).count();
    let powerup_count = powerup_query.iter().filter(|powerup| powerup.powerup_type != PowerUpType::ExtraLife).count();

    if enemy_count == 0 && powerup_count == 0 {
        let mut rng = rand::thread_rng();
        let powerup_type = match rng.gen_range(0..4) {
            0 => PowerUpType::FasterProjectile,
            1 => PowerUpType::ExplosiveProjectile,
            2 => PowerUpType::RotatingCircle,
            3 => PowerUpType::ExtraLife,
            _ => unreachable!(),
        };

        let color = match powerup_type {
            PowerUpType::FasterProjectile => Color::BLUE,
            PowerUpType::ExplosiveProjectile => Color::ORANGE,
            PowerUpType::RotatingCircle => Color::PURPLE,
            PowerUpType::ExtraLife => Color::RED,
        };

        commands.spawn((
            PowerUp { powerup_type },
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(10.0)).into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
        ));
    }
}

pub fn try_spawn_powerup(
    commands: &mut Commands,
    position: Vec2,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    let mut rng = rand::thread_rng();
    if rng.gen_range(0.0..1.0) < 0.2 {
        let powerup_type = PowerUpType::ExtraLife;
        let color = Color::RED;

        commands.spawn((
            PowerUp { powerup_type },
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(10.0)).into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_xyz(position.x, position.y, 0.0),
                ..default()
            },
        ));
    }
}

pub fn collect_powerups(
    mut commands: Commands,
    mut player_query: Query<&mut Transform, With<Player>>,
    powerup_query: Query<(Entity, &Transform, &PowerUp)>,
    mut lives_query: Query<&mut Lives>,
    mut player_powerup_state: ResMut<PlayerPowerUpState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player_transform = player_query.single_mut();
    let player_pos = player_transform.translation.xy();

    for (powerup_entity, powerup_transform, powerup) in powerup_query.iter() {
        let powerup_pos = powerup_transform.translation.xy();
        let distance = (player_pos - powerup_pos).length();

        if distance < 20.0 {
            match powerup.powerup_type {
                PowerUpType::FasterProjectile => {
                    player_powerup_state.faster_projectile_timer = Some(Timer::from_seconds(10.0, TimerMode::Once));
                }
                PowerUpType::ExplosiveProjectile => {
                    player_powerup_state.explosive_projectile_timer = Some(Timer::from_seconds(10.0, TimerMode::Once));
                }
                PowerUpType::RotatingCircle => {
                    commands.spawn((
                        RotatingCircle {
                            angle: 0.0,
                            speed: 2.0,
                            radius: 50.0,
                            timer: Timer::from_seconds(10.0, TimerMode::Once),
                        },
                        MaterialMesh2dBundle {
                            mesh: meshes.add(Circle::new(5.0)).into(),
                            material: materials.add(ColorMaterial::from(Color::PURPLE)),
                            transform: Transform::from_xyz(player_pos.x, player_pos.y, 0.0),
                            ..default()
                        },
                    ));
                }
                PowerUpType::ExtraLife => {
                    let mut lives = lives_query.single_mut();
                    if lives.count() < 5 {
                        lives.add_life();
                    }
                }
            }
            commands.entity(powerup_entity).despawn();
        }
    }
}

pub fn update_powerup_timers(
    mut player_powerup_state: ResMut<PlayerPowerUpState>,
    time: Res<Time>,
) {
    if let Some(ref mut timer) = player_powerup_state.faster_projectile_timer {
        timer.tick(time.delta());
        if timer.finished() {
            player_powerup_state.faster_projectile_timer = None;
        }
    }

    if let Some(ref mut timer) = player_powerup_state.explosive_projectile_timer {
        timer.tick(time.delta());
        if timer.finished() {
            player_powerup_state.explosive_projectile_timer = None;
        }
    }
}

pub fn update_rotating_circle(
    mut commands: Commands,
    mut circle_query: Query<(Entity, &mut Transform, &mut RotatingCircle)>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<(Entity, &Transform), (With<Enemy>, Without<Player>)>,
    time: Res<Time>,
) {
    let player_transform = player_query.single();
    let player_pos = player_transform.translation.xy();

    for (entity, mut transform, mut circle) in circle_query.iter_mut() {
        circle.angle += circle.speed * time.delta_seconds();
        let x = player_pos.x + circle.radius * circle.angle.cos();
        let y = player_pos.y + circle.radius * circle.angle.sin();
        transform.translation = Vec3::new(x, y, 0.0);

        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            let enemy_pos = enemy_transform.translation.xy();
            let distance = (Vec2::new(x, y) - enemy_pos).length();
            if distance < 10.0 {
                commands.entity(enemy_entity).despawn();
            }
        }

        circle.timer.tick(time.delta());
        if circle.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
