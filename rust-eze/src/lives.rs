use bevy::prelude::*;

pub struct LivesPlugin;

impl Plugin for LivesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_lives);
    }
}

#[derive(Component)]
pub struct Lives(u32);

impl Lives {
    pub fn new(count: u32) -> Self {
        Lives(count)
    }

    pub fn lose_life(&mut self) -> bool {
        if self.0 > 0 {
            self.0 -= 1;
        }
        self.0 == 0
    }

    pub fn add_life(&mut self) {
        self.0 += 1;
    }

    pub fn count(&self) -> u32 {
        self.0
    }

    pub fn reset(&mut self) {
        self.0 = 3;
    }
}

fn setup_lives(
    mut commands: Commands,
) {
    commands.spawn(Lives::new(3));
}