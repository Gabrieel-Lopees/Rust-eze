use bevy::prelude::*;
use crate::player::Player;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, move_player);
    }
}

fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: QUery<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let speed = 300.0;
    for mut transform in query.iter_mut() {
        if keyboard.pressed(Keycode::KeyW) {
            transform.translation.y += speed * time.delta_seconds();
        }
        if keyboard.pressed(KeyCode::KeyS) {
            transform.translation.y -= speed * time.delta_seconds();
        }
        if keyboard.pressed(KeyCode::KeyD) {
            transform.translation.x += speed * time.delta_seconds();
        }
        if keyboard.pressed(KeyCode::KeyA) {
            transform.translation.x -= speed * time.delta_seconds();
        }
    } 
}
