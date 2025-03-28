use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::player::Player;
use crate::enemies::Enemy;
use crate::config::GameConfig;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player);
    }
}

fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    let window_width = window.width();
    let window_height = window.height();

    for mut transform in query.iter_mut() {
        let mut new_x = transform.translation.x;
        let mut new_y = transform.translation.y;

        if keyboard.pressed(KeyCode::KeyW) {
            new_y += GameConfig::PLAYER_SPEED * time.delta_seconds();
        }
        if keyboard.pressed(KeyCode::KeyS) {
            new_y -= GameConfig::PLAYER_SPEED * time.delta_seconds();
        }
        if keyboard.pressed(KeyCode::KeyD) {
            new_x += GameConfig::PLAYER_SPEED * time.delta_seconds();
        }
        if keyboard.pressed(KeyCode::KeyA) {
            new_x -= GameConfig::PLAYER_SPEED * time.delta_seconds();
        }

        let x_bound = (window_width - GameConfig::WALL_THICKNESS - GameConfig::PLAYER_SIZE) / 2.0;
        let y_bound = (window_height - GameConfig::WALL_THICKNESS - GameConfig::PLAYER_SIZE) / 2.0;

        new_x = new_x.clamp(-x_bound, x_bound);
        new_y = new_y.clamp(-y_bound, y_bound);
        transform.translation.x = new_x;
        transform.translation.y = new_y;
    }
}
