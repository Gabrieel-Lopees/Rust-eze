use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::config::GameConfig;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_walls);
    }
}

#[derive(Component)]
pub struct Wall;

fn spawn_walls(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();
    let window_width = window.width();
    let window_height = window.height();

    // Parede superior
    commands.spawn((
        Wall,
        SpriteBundle {
            transform: Transform::from_xyz(0.0, window_height / 2.0, 0.0),
            sprite: Sprite {
                color: Color::GRAY,
                custom_size: Some(Vec2::new(window_width, GameConfig::WALL_THICKNESS)),
                ..default()
            },
            ..default()
        },
    ));

    // Parede inferior
    commands.spawn((
        Wall,
        SpriteBundle {
            transform: Transform::from_xyz(0.0, -window_height / 2.0, 0.0),
            sprite: Sprite {
                color: Color::GRAY,
                custom_size: Some(Vec2::new(window_width, GameConfig::WALL_THICKNESS)),
                ..default()
            },
            ..default()
        },
    ));

    // Parede esquerda
    commands.spawn((
        Wall,
        SpriteBundle {
            transform: Transform::from_xyz(-window_width / 2.0, 0.0, 0.0),
            sprite: Sprite {
                color: Color::GRAY,
                custom_size: Some(Vec2::new(GameConfig::WALL_THICKNESS, window_height)),
                ..default()
            },
            ..default()
        },
    ));

    // Parede direita
    commands.spawn((
        Wall,
        SpriteBundle {
            transform: Transform::from_xyz(window_width / 2.0, 0.0, 0.0),
            sprite: Sprite {
                color: Color::GRAY,
                custom_size: Some(Vec2::new(GameConfig::WALL_THICKNESS, window_height)),
                ..default()
            },
            ..default()
        },
    ));
}
