use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::enemies::Enemy;

// Importa módulos relacionados aos power-ups e tipos de projéteis
pub mod powerups;
mod standard_projectile;
mod fire_projectile;
mod ice_projectile;
mod electric_projectile;
mod explosive_projectile;

pub struct ProjectilesPlugin;

impl Plugin for ProjectilesPlugin {
    fn build(&self, app: &mut App) {
        app
            // Inicializa os recursos de power-ups
            .init_resource::<powerups::PlayerPowerUpState>()
            .init_resource::<powerups::PowerUpSpawnState>()
            // Adiciona os sistemas responsáveis pelos projéteis e power-ups
            .add_systems(Update, (
                spawn_projectiles,       // Sistema de spawn de projéteis
                move_projectiles,        // Sistema de movimentação de projéteis
                check_projectile_collision, // Sistema de detecção de colisão
                powerups::spawn_powerups,
                powerups::collect_powerups,
                powerups::update_powerup_timers,
                powerups::update_rotating_circle,
                powerups::reset_powerup_spawn_state, // Sistema de reset de spawn de power-ups
            ));
    }
}

// Componente que representa um projétil
#[derive(Component)]
pub struct Projectile {
    direction: Vec2,
    speed: f32,
    explosive: bool,
}

// Constantes definindo tamanho e velocidade padrão dos projéteis
const PROJECTILE_SIZE: f32 = 4.0;
const PROJECTILE_SPEED: f32 = 400.0;

// Função para spawnar projéteis
fn spawn_projectiles(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, With<crate::player::Player>>, // Obtém a posição do jogador
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    player_powerup_state: Res<powerups::PlayerPowerUpState>,
) {
    let player_transform = player_query.single(); // <-- PODE CAUSAR PANIC SE NÃO HOUVER UM PLAYER
    let player_pos = player_transform.translation.xy();

    static mut LAST_SHOT: f32 = 0.0;
    let current_time = time.elapsed_seconds();
    if current_time - unsafe { LAST_SHOT } < 0.2 {
        return; // Limita a taxa de disparo dos projéteis
    }

    let mut direction = Vec2::ZERO;
    let mut projectile_type = None;

    // Determina a direção e tipo do projétil baseado nas teclas pressionadas
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

        // Spawna projéteis de acordo com o tipo selecionado
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

// Função para mover projéteis
fn move_projectiles(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &mut Transform, &Projectile)>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single(); // <-- PODE CAUSAR PANIC SE NÃO HOUVER UMA JANELA
    let window_width = window.width();
    let window_height = window.height();
    let x_bound = (window_width - crate::config::GameConfig::WALL_THICKNESS) / 2.0;
    let y_bound = (window_height - crate::config::GameConfig::WALL_THICKNESS) / 2.0;

    for (entity, mut transform, projectile) in projectile_query.iter_mut() {
        transform.translation += Vec3::new(
            projectile.direction.x * projectile.speed * time.delta_seconds(),
            projectile.direction.y * projectile.speed * time.delta_seconds(),
            0.0,
        );

        if transform.translation.x.abs() > x_bound || transform.translation.y.abs() > y_bound {
            commands.entity(entity).despawn(); // Remove projéteis que saem da tela
        }
    }
}

// Verifica colisão entre projéteis e inimigos
fn check_projectile_collision(
    mut commands: Commands,
    mut transform_queries: ParamSet<(
        Query<(Entity, &Transform, &Projectile), Without<Enemy>>,
        Query<(Entity, &Transform), (With<Enemy>, Without<Projectile>)>,
    )>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    let enemies: Vec<(Entity, Vec2)> = transform_queries.p1()
        .iter()
        .map(|(entity, transform)| (entity, transform.translation.xy()))
        .collect();

    for (projectile_entity, projectile_transform, projectile) in transform_queries.p0().iter_mut() {
        let projectile_pos = projectile_transform.translation.xy();
        for (enemy_entity, enemy_pos) in &enemies {
            let distance = (projectile_pos - *enemy_pos).length();

            if distance < crate::config::GameConfig::ENEMY_SIZE + PROJECTILE_SIZE {
                commands.entity(projectile_entity).despawn();
                commands.entity(*enemy_entity).despawn();
                break;
            }
        }
    }
}
