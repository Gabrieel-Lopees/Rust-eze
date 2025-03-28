use bevy::prelude::*;
use crate::player::Player;
use bevy::window::PrimaryWindow;
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player);
    }
}

fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {

    let speed = 300.0;
    let window = window_query.single();
    let window_width = window.width(); // corresponde a largura da janela
    let window_height = window.height(); // altura
    
    let player_size = 50.0; // tamanho do boneco
    let wall_thickness = 20.0; // gorssura da parede
    
    for mut transform in query.iter_mut() {
        let mut new_x = transform.translation.x;
        let mut new_y = transform.translation.y;

        // Rotina if para o movimetno 
        if keyboard.pressed(KeyCode::KeyW) {
            new_y += speed * time.delta_seconds();
        }
        if keyboard.pressed(KeyCode::KeyS) {
            new_y -= speed * time.delta_seconds();
        }
        if keyboard.pressed(KeyCode::KeyD) {
            new_x += speed * time.delta_seconds();
        }
        if keyboard.pressed(KeyCode::KeyA) {
            new_x -= speed * time.delta_seconds();
        }

        let x_bound = (window_width - wall_thickness - player_size) / 2.0;
        let y_bound = (window_height - wall_thickness - player_size) / 2.0;


        new_x = new_x.clamp(-x_bound, x_bound);
        new_y = new_y.clamp(-y_bound, y_bound);
        transform.translation.x = new_x;
        transform.translation.y = new_y;
    }
}



