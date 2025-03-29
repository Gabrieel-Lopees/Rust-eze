use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct LivesText;

#[derive(Component)]
pub struct GameOverText;

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Score>()
            .add_systems(Startup, setup_ui)
            .add_systems(Update, (update_score, update_lives_text));
    }
}

fn setup_ui(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();
    let window_width = window.width();
    let window_height = window.height();

    // Texto do título
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

    // Texto da pontuação
    commands.spawn((
        ScoreText,
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font_size: 30.0,
                    color: Color::YELLOW,
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            left: Val::Px(10.0),
            ..default()
        }),
    ));

    // Texto das vidas (corações)
    commands.spawn((
        LivesText,
        TextBundle::from_section(
            "❤️❤️❤️",
            TextStyle {
                font_size: 30.0,
                color: Color::RED,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(90.0),
            left: Val::Px(10.0),
            ..default()
        }),
    ));

    // Texto de Game Over (centralizado)
    commands.spawn((
        GameOverText,
        TextBundle::from_section(
            "Game Over\nPress Enter to Restart",
            TextStyle {
                font_size: 50.0,
                color: Color::RED,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(window_height / 2.0 - 50.0),
            left: Val::Px(window_width / 2.0 - 200.0),
            display: Display::None,
            ..default()
        }),
    ));
}

fn update_score(
    time: Res<Time>,
    mut score: ResMut<Score>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    score.value += (time.delta_seconds() * 10.0) as u32;
    for mut text in query.iter_mut() {
        text.sections[1].value = score.value.to_string();
    }
}

fn update_lives_text(
    lives_query: Query<&crate::lives::Lives>,
    mut text_query: Query<&mut Text, With<LivesText>>,
) {
    let lives = lives_query.single();
    let mut text = text_query.single_mut();
    text.sections[0].value = "❤️".repeat(lives.count() as usize);
}