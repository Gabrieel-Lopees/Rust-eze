use bevy::prelude::*;

#[derive(Component)]
pub struct Lives {
    count: u32,
}

impl Lives {
    pub fn new() -> Self {
        Lives { count: 3 }
    }

    pub fn lose_life(&mut self) -> bool {
        if self.count > 0 {
            self.count -= 1;
            self.count == 0  // Retorna true se as vidas acabaram
        } else {
            true
        }
    }

    pub fn reset(&mut self) {
        self.count = 3;
    }

    pub fn count(&self) -> u32 {
        self.count
    }
}

pub struct LivesPlugin;

impl Plugin for LivesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_lives);
    }
}

fn setup_lives(mut commands: Commands) {
    commands.spawn(Lives::new());
}
