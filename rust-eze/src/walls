use bevy::prelude::*;
use bevy::window::PrimaryWindow;  // Importação necessária

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_walls);
    }
}

#[derive(Component)]
pub struct Wall;

fn spawn_walls(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {  // Adicione este parâmetro
    let wall_thickness = 20.0;
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
                custom_size: Some(Vec2::new(window_width, wall_thickness)),
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
                custom_size: Some(Vec2::new(window_width, wall_thickness)),
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
                custom_size: Some(Vec2::new(wall_thickness, window_height)),
                ..default()
            },
            ..

default()
        },
    ));

    // Parede direita
    commands.spawn((
        Wall,
        SpriteBundle {
            transform: Transform::from_xyz(window_width / 2.0, 0.0, 0.0),
            sprite: Sprite {
                color: Color::GRAY,
                custom_size: Some(Vec2::new(wall_thickness, window_height)),
                ..default()
            },
            ..default()
        },
    ));
}