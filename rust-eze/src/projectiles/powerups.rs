use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use rand::Rng;

#[derive(Component)]
pub struct PowerUp;

pub fn spawn_powerups(
    powerup_query: Query<&PowerUp>,
) {
    if powerup_query.iter().count() >= 3 {
        return;
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
        commands.spawn((
            PowerUp,
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(10.0)).into(),
                material: materials.add(ColorMaterial::from(Color::YELLOW)),
                transform: Transform::from_xyz(position.x, position.y, 0.0),
                ..default()
            },
        ));
    }
}