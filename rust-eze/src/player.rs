use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::config::GameConfig;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
     commands.spawn(Camera2dBundle::default());


    commands.spawn((
        Player,
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(GameConfig::PLAYER_SIZE)).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));
}
