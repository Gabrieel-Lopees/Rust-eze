use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_ui);
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn(
        TextBundle::from_section(
            "Rust-eze Game",
            TextStyle {
                font_size: 40.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
}
