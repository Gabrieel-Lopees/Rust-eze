use bevy::prelude::*;


// declaração:
pub mod player;
pub mod movement;
pub mod ui;

pub struct RustEzePlg;

impl Plugin for RustEzePlg {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPlugins)
            .add_plugins(player::PlayerPlugin)
            .add_plugins(movement::MovementPlugin)
            .add_plugins(ui::UiPlugin);
    }
}


