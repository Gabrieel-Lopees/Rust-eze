use bevy::prelude::*;
use rust_eze::RustEzePlugin;

fn main() {
    App:new()
        .add_plugins(RustEzePlugin)
        .run();
}
