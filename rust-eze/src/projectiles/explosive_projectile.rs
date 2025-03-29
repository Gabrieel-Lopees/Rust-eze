use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

pub fn spawn(
    commands: &mut Commands,
    position: Vec2,
    direction: Vec2,
    speed: f32,
    explosive: bool,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    commands.spawn((
        super::Projectile {
            direction,
            speed,
            explosive,
        },
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(super::PROJECTILE_SIZE)).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_xyz(position.x, position.y, 0.0),
            ..default()
        },
    ));
}
