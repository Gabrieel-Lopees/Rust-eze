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

#[derive(Resource, Default)]
pub struct PowerUpSpawnState {
    pub powerup_spawned: bool,
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
    mut powerup_spawn_state: ResMut<PowerUpSpawnState>,
) {
    let enemy_count = enemy_query.iter().filter(|enemy| enemy.room == current_room.id).count();
    let powerup_count = powerup_query.iter().filter(|powerup| powerup.powerup_type != PowerUpType::ExtraLife).count();

    if enemy_count == 0 && powerup_count == 0 && !powerup_spawn_state.powerup_spawned {
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
            PowerUpType::ExplosiveProjectile => Color::PURPLE,
            PowerUpType::RotatingCircle => Color::ORANGE,
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
        powerup_spawn_state.powerup_spawned = true;
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
    mut transform_queries: ParamSet<(
        Query<&mut Transform, With<Player>>,
        Query<(Entity, &Transform, &PowerUp), Without<Player>>,
    )>,
    mut lives_query: Query<&mut Lives>,
    mut player_powerup_state: ResMut<PlayerPowerUpState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut powerup_spawn_state: ResMut<PowerUpSpawnState>,
) {
    let player_pos = {
        let mut player_query = transform_queries.p0();
        // Tenta obter o jogador, se não encontrar, retorna
        let player_transform = if let Some(player_transform) = player_query.iter_mut().next() {
            player_transform
        } else {
            return; // Se não houver um jogador, saímos da função
        };
        player_transform.translation.xy()
    };

    let mut powerup_query = transform_queries.p1();
    for (powerup_entity, powerup_transform, powerup) in powerup_query.iter_mut() {
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
                    // Tenta obter o componente Lives, se não encontrar, retorna
                    let mut lives = if let Some(lives) = lives_query.iter_mut().next() {
                        lives
                    } else {
                        return; // Se não houver o componente Lives, saímos da função
                    };
                    if lives.count() < 5 {
                        lives.add_life();
                    }
                }
            }
            commands.entity(powerup_entity).despawn();
            powerup_spawn_state.powerup_spawned = false;
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
    mut transform_queries: ParamSet<(
        Query<(Entity, &mut Transform, &mut RotatingCircle)>,
        Query<&Transform, With<Player>>,
        Query<(Entity, &Transform), (With<Enemy>, Without<Player>)>,
    )>,
    time: Res<Time>,
) {
    let player_pos = {
        let player_query = transform_queries.p1();
        let player_transform = player_query.get_single().expect("Falha ao obter o transform do jogador!");
        player_transform.translation.xy()
    };

    let enemies: Vec<(Entity, Vec2)> = {
        let enemy_query = transform_queries.p2();
        enemy_query.iter().map(|(entity, transform)| (entity, transform.translation.xy())).collect()
    };

    let mut circle_query = transform_queries.p0();
    for (entity, mut transform, mut circle) in circle_query.iter_mut() {
        circle.angle += circle.speed * time.delta_seconds();
        let x = player_pos.x + circle.radius * circle.angle.cos();
        let y = player_pos.y + circle.radius * circle.angle.sin();
        transform.translation = Vec3::new(x, y, 0.0);

        for (enemy_entity, enemy_pos) in enemies.iter() {
            let distance = (Vec2::new(x, y) - *enemy_pos).length();
            if distance < 10.0 {
                commands.entity(*enemy_entity).despawn();
            }
        }

        circle.timer.tick(time.delta());
        if circle.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
