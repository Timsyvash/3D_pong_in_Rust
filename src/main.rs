use bevy::prelude::*;

#[derive(Component)]
struct Ball {
    velocity: Vec3,
}

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct LivesText;

#[derive(Component)]
struct GameOverText;

#[derive(Component)]
struct PauseText;

#[derive(Resource)]
struct GameState {
    score: u32,
    lives: i32,
    is_game_over: bool,
    is_pause: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            score: 0,
            lives: 4,
            is_game_over: false,
            is_pause: false,
        }
    }
}

const PADDLE_SPEED: f32 = 12.0;
const BALL_SPEED: f32 = 10.0;
const ROOM_WIDTH: f32 = 20.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<GameState>()
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(Update, (move_paddle, move_ball, update_ui, restart_game, pause_game))
        .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 8.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 2000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 6.0, 0.0),
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(3.0, 0.5, 0.5)),
            material: materials.add(Color::rgb(0.2, 0.7, 0.2)),
            transform: Transform::from_xyz(0.0, 0.0, 5.0),
            ..default()
        },
        Paddle,
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(0.3).mesh().ico(4).unwrap()),
            material: materials.add(Color::rgb(0.9, 0.1, 0.1)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Ball {
            velocity: Vec3::new(0.6, 0.0, 0.8).normalize() * BALL_SPEED,
        },
    ));

    let wall_material = materials.add(Color::rgb(0.3, 0.3, 0.3));
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(0.2, 0.5, 12.0)),
        material: wall_material.clone(),
        transform: Transform::from_xyz(-ROOM_WIDTH / 2.0, 0.0, 0.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(0.2, 0.5, 12.0)),
        material: wall_material.clone(),
        transform: Transform::from_xyz(ROOM_WIDTH / 2.0, 0.0, 0.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(ROOM_WIDTH, 0.5, 0.2)),
        material: wall_material,
        transform: Transform::from_xyz(0.0, 0.0, -6.0),
        ..default()
    });
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Очки: 0",
                    TextStyle {
                        font_size: 40.0,
                        font: asset_server.load("fonts/FiorinaTitle-LightItalic.otf"),
                        color: Color::CYAN,
                        ..default()
                    },
                ),
                ScoreText,
            ));

            parent.spawn((
                TextBundle::from_section(
                    "Життя: 4",
                    TextStyle {
                        font_size: 40.0,
                        font: asset_server.load("fonts/FiorinaTitle-LightItalic.otf"),
                        color: Color::CYAN,
                        ..default()
                    },
                ),
                LivesText,
            ));
        });

    commands.spawn((
        TextBundle {
            visibility: Visibility::Hidden,
            text: Text::from_section(
                "Гра програна\nНатисніть ПРОБІЛ для перезапуску",
                TextStyle {
                    font_size: 40.0,
                    font: asset_server.load("fonts/FiorinaTitle-LightItalic.otf"),
                    color: Color::CYAN,
                    ..default()
                },
            ).with_justify(JustifyText::Center),
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(20.0),
                top: Val::Percent(40.0),
                ..default()
            },
            ..default()
        },
        GameOverText,
    ));

    commands.spawn((
        TextBundle {
            visibility: Visibility::Hidden,
            text: Text::from_section(
                "ПАУЗА\nНатисніть P для продовження",
                TextStyle {
                    font_size: 40.0,
                    font: asset_server.load("fonts/FiorinaTitle-LightItalic.otf"),
                    color: Color::CYAN,
                    ..default()
                },
            ).with_justify(JustifyText::Center),
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(30.0),
                top: Val::Percent(40.0),
                ..default()
            },
            ..default()
        },
        PauseText,
    ));
}

fn move_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    game_state: Res<GameState>,
    mut query: Query<&mut Transform, With<Paddle>>,
) {
    if game_state.is_game_over || game_state.is_pause {
        return;
    }

    for mut transform in &mut query {
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            direction += 1.0;
        }

        transform.translation.x += direction * PADDLE_SPEED * time.delta_seconds();

        let max_x = (ROOM_WIDTH / 2.0) - 1.6;
        transform.translation.x = transform.translation.x.clamp(-max_x, max_x);
    }
}

fn move_ball(
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
    paddle_query: Query<&Transform, (With<Paddle>, Without<Ball>)>,
) {
    if game_state.is_game_over || game_state.is_pause {
        return;
    }

    let paddle_transform = paddle_query.single();

    for (mut ball_transform, mut ball) in &mut ball_query {
        ball_transform.translation += ball.velocity * time.delta_seconds();

        let border_x = (ROOM_WIDTH / 2.0) - 0.4;
        if ball_transform.translation.x.abs() > border_x {
            ball.velocity.x = -ball.velocity.x;
            ball_transform.translation.x = ball_transform.translation.x.signum() * border_x;
        }

        if ball_transform.translation.z < -5.7 {
            ball.velocity.z = -ball.velocity.z;
            ball_transform.translation.z = -5.7;
        }

        let ball_pos = ball_transform.translation;
        let paddle_pos = paddle_transform.translation;

        if ball_pos.z >= paddle_pos.z - 0.4 && ball_pos.z <= paddle_pos.z + 0.4 {
            if ball_pos.x >= paddle_pos.x - 1.6 && ball_pos.x <= paddle_pos.x + 1.6 {
                if ball.velocity.z > 0.0 {
                    ball.velocity.z = -ball.velocity.z;
                    ball.velocity.x += (ball_pos.x - paddle_pos.x) * 2.0;
                    ball.velocity = ball.velocity.normalize() * BALL_SPEED;

                    game_state.score += 1;
                }
            }
        }

        if ball_transform.translation.z > 7.0 {
            game_state.lives -= 1;

            if game_state.lives <= 0 {
                game_state.is_game_over = true;
            } else {
                ball_transform.translation = Vec3::ZERO;
                ball.velocity = Vec3::new(0.5, 0.0, -0.8).normalize() * BALL_SPEED;
            }
        }
    }
}

fn update_ui(
    game_state: Res<GameState>,
    mut score_query: Query<&mut Text, (With<ScoreText>, Without<LivesText>)>,
    mut lives_query: Query<&mut Text, (With<LivesText>, Without<ScoreText>)>,
    mut game_over_query: Query<&mut Visibility, (With<GameOverText>, Without<PauseText>)>,
    mut pause_query: Query<&mut Visibility, (With<PauseText>, Without<GameOverText>)>, // Додаємо запит 👇
) {
    if game_state.is_changed() {
        if let Ok(mut text) = score_query.get_single_mut() {
            text.sections[0].value = format!("Очки: {}", game_state.score);
        }
        if let Ok(mut text) = lives_query.get_single_mut() {
            text.sections[0].value = format!("Життя: {}", game_state.lives);
        }
        if let Ok(mut visibility) = game_over_query.get_single_mut() {
            *visibility = if game_state.is_game_over { Visibility::Visible } else { Visibility::Hidden };
        }
        if let Ok(mut visibility) = pause_query.get_single_mut() {
            *visibility = if game_state.is_pause && !game_state.is_game_over {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

fn restart_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
) {
    if game_state.is_game_over && keyboard_input.just_pressed(KeyCode::Space) {
        game_state.score = 0;
        game_state.lives = 3;
        game_state.is_game_over = false;

        for (mut ball_transform, mut ball) in &mut ball_query {
            ball_transform.translation = Vec3::ZERO;
            ball.velocity = Vec3::new(0.5, 0.0, -0.8).normalize() * BALL_SPEED;
        }
    }
}

fn pause_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        game_state.is_pause = !game_state.is_pause;
    }
}