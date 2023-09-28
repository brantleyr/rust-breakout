use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle,
};

// Constants
// Grid/Bricks
const GRID_HEIGHT: f32 = 5.;
const GRID_WIDTH: f32 = 10.;
const GRID_CELL_SPACE: f32 = 5.;
const GRID_CELL_WIDTH: f32 = 80.;
const GRID_CELL_HEIGHT: f32 = 30.;
const GRID_CELL_TOP: f32 = 300.;
const GRID_CELL_LEFT: f32 = -350.;
const BRICK_SIZE: Vec3 = Vec3::new(GRID_CELL_WIDTH, GRID_CELL_HEIGHT, 0.0);

// BG
const BACKGROUND_COLOR: Color = Color::rgb(0., 0., 0.25);

// Walls
const LEFT_WALL: f32 = -400.;
const RIGHT_WALL: f32 = 465.;
const TOP_WALL: f32 = 325.;
const BOTTOM_WALL: f32 = -325.;
const WALL_COLOR: Color = Color::rgb(0., 0.75, 0.);
const TB_WALL_ADJUST: f32 = 32.5;
const WALL_SIZE: f32 = 10.;

// Ball
const BALL_COLOR: Color = Color::PURPLE;
const BALL_SPEED: f32 = 400.0;
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const LEFT_WALL_SIZE: Vec3 = Vec3::new(10.0, 650.0, 0.0);
const RIGHT_WALL_SIZE: Vec3 = Vec3::new(10.0, 650.0, 0.0);
const TOP_WALL_SIZE: Vec3 = Vec3::new(875.0, 10.0, 0.0);
const BOTTOM_WALL_SIZE: Vec3 = Vec3::new(875.0, 10.0, 0.0);
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

// Paddle
const PADDLE_WIDTH: f32 = 125.;
const PADDLE_COLOR: Color = Color::ORANGE;
const PADDLE_SPEED: f32 = 10.0;
const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 20.0;
const LEFT_BOUND_PADDLE: f32 = LEFT_WALL + WALL_SIZE + (PADDLE_WIDTH / 2.);
const RIGHT_BOUND_PADDLE: f32 = RIGHT_WALL - WALL_SIZE - (PADDLE_WIDTH / 2.);

// Scoreboard
const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const SCOREBOARD_TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCOREBOARD_SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

// Info text
const INFO_VERTICAL_PADDING: Val = Val::Px(45.0);
const INFO_TEXT_PADDING: Val = Val::Px(8.0);
const INFO_FONT_SIZE: f32 = 18.5;
const INFO_TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);

// Start Game text and Overlay
const START_GAME_VERTICAL_PADDING: Val = Val::Px(300.0);
const START_GAME_LEFT_PADDING: Val = Val::Px(475.0);
const START_GAME_FONT_SIZE: f32 = 50.0;
const START_GAME_TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const START_OVERLAY_SIZE: Vec3 = Vec3::new(1500.0, 1500.0, 0.0);
const START_OVERLAY_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.95);

// Pause Game text and Overlay
const PAUSE_GAME_VERTICAL_PADDING: Val = Val::Px(300.0);
const PAUSE_GAME_LEFT_PADDING: Val = Val::Px(475.0);
const PAUSE_GAME_FONT_SIZE: f32 = 50.0;
const PAUSE_GAME_TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const PAUSE_OVERLAY_SIZE: Vec3 = Vec3::new(1500.0, 1500.0, 0.0);
const PAUSE_OVERLAY_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.95);

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Event, Default)]
struct ExplosionEvent;

#[derive(Resource)]
struct CollisionSound(Handle<AudioSource>);

#[derive(Resource)]
struct ExplosionSound(Handle<AudioSource>);

#[derive(Component)]
struct Brick;

#[derive(Component)]
struct StartGameOverlay;

#[derive(Component)]
struct PauseGameOverlay;

#[derive(Component)]
struct InfoText;

#[derive(Component)]
struct ScoreboardText;

// This resource tracks the game's score
#[derive(Resource)]
struct Scoreboard {
    score: usize,
}

// Game State
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    InGame,
    Paused,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(OnEnter(GameState::Paused), check_for_state)
        .add_systems(OnEnter(GameState::InGame), check_for_state)
        .add_event::<CollisionEvent>()
        .add_event::<ExplosionEvent>()
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_systems(
            FixedUpdate,
            (
                apply_velocity.before(check_for_collisions),
                move_paddle
                    .before(check_for_collisions)
                    .after(apply_velocity),
                check_for_collisions,
                play_collision_sound.after(check_for_collisions),
                play_explosion_sound.after(check_for_collisions),
            ),
        )
        .add_systems(Update, (update_scoreboard, bevy::window::close_on_esc))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    game_state: Res<State<GameState>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Draw walls
    // Left
    let left_x = LEFT_WALL;
    let left_y = 0.0;
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(left_x, left_y, 0.0),
                scale: LEFT_WALL_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: WALL_COLOR,
                ..default()
            },
            ..default()
        },
        Collider,
    ));
    // Right
    let right_x = RIGHT_WALL;
    let right_y = 0.0;
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(right_x, right_y, 0.0),
                scale: RIGHT_WALL_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: WALL_COLOR,
                ..default()
            },
            ..default()
        },
        Collider,
    ));
    // Top
    let top_x = 0.0 + TB_WALL_ADJUST;
    let top_y = TOP_WALL;
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(top_x, top_y, 0.0),
                scale: TOP_WALL_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: WALL_COLOR,
                ..default()
            },
            ..default()
        },
        Collider,
    ));
    // Bottom
    let bottom_x = 0.0 + TB_WALL_ADJUST;
    let bottom_y = BOTTOM_WALL;
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(bottom_x, bottom_y, 0.0),
                scale: BOTTOM_WALL_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: WALL_COLOR,
                ..default()
            },
            ..default()
        },
        Collider,
    ));

    // Draw Grid
    let mut i = 0.;
    while i < GRID_HEIGHT {
        let mut i2 = 0.;
        while i2 < GRID_WIDTH {
            let grid_cell_top = GRID_CELL_TOP - (i * GRID_CELL_HEIGHT) - (i * GRID_CELL_SPACE);
            let grid_cell_left = GRID_CELL_LEFT + (i2 * GRID_CELL_WIDTH) + (i2 * GRID_CELL_SPACE);
            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(grid_cell_left, grid_cell_top, 0.0),
                        scale: BRICK_SIZE,
                        ..default()
                    },
                    sprite: Sprite {
                        color: Color::rgb(0.25 + (i / 10.), 0.75, 0.25 + (i / 10.)),
                        ..default()
                    },
                    ..default()
                },
                Brick,
                Collider,
            ));
            i2 += 1.;
        }
        i += 1.;
    }

    // Load collision sounds
    let ball_collision_sound = asset_server.load("sounds/breakout_collision.ogg");
    commands.insert_resource(CollisionSound(ball_collision_sound));

    let brick_explosion_sound = asset_server.load("sounds/breakout_brick_explosion.ogg");
    commands.insert_resource(ExplosionSound(brick_explosion_sound));

    // Draw Paddle
    let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, paddle_y, 0.0),
                scale: PADDLE_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        },
        Paddle,
        Collider,
    ));

    // Draw Ball
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(BALL_COLOR)),
            transform: Transform::from_translation(BALL_STARTING_POSITION).with_scale(BALL_SIZE),
            ..default()
        },
        Ball,
        Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED),
    ));

    // Draw Info text
    commands.spawn((
        TextBundle::from_section(
            "Keys:\nLeft/Right arrow to move\nEnter to Pause\nEscape to Quit.",
            TextStyle {
                font_size: INFO_FONT_SIZE,
                color: INFO_TEXT_COLOR,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: INFO_VERTICAL_PADDING,
            left: INFO_TEXT_PADDING,
            ..default()
        }),
        InfoText,
    ));

    // Draw Scoreboard
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: SCOREBOARD_TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCOREBOARD_SCORE_COLOR,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: SCOREBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        }),
        ScoreboardText,
    ));

    // Draw "ENTER to start" if Game has not yet been started
    if game_state.get() == &GameState::Paused {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    scale: START_OVERLAY_SIZE,
                    ..default()
                },
                sprite: Sprite {
                    color: START_OVERLAY_COLOR,
                    ..default()
                },
                ..default()
            },
            StartGameOverlay,
        ));
        commands.spawn((
            TextBundle::from_section(
                "ENTER to start",
                TextStyle {
                    font_size: START_GAME_FONT_SIZE,
                    color: START_GAME_TEXT_COLOR,
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: START_GAME_VERTICAL_PADDING,
                left: START_GAME_LEFT_PADDING,
                ..default()
            }),
            StartGameOverlay,
        ));
    }
}

fn move_paddle(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Paddle>>,
    game_state: Res<State<GameState>>,
) {
    if game_state.get() == &GameState::InGame {
        let mut direction = 0.;
        if keyboard_input.pressed(KeyCode::Left) {
            direction = -1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction = 1.0;
        }
        let mut paddle_transform = query.single_mut();
        let new_paddle_position = paddle_transform.translation.x + (direction * PADDLE_SPEED);

        paddle_transform.translation.x =
            new_paddle_position.clamp(LEFT_BOUND_PADDLE, RIGHT_BOUND_PADDLE);
    }
}

fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity)>,
    time_step: Res<FixedTime>,
    game_state: Res<State<GameState>>,
) {
    if game_state.get() == &GameState::InGame {
        for (mut transform, velocity) in &mut query {
            transform.translation.x += velocity.x * time_step.period.as_secs_f32();
            transform.translation.y += velocity.y * time_step.period.as_secs_f32();
        }
    }
}

fn update_scoreboard(
    scoreboard: Res<Scoreboard>,
    mut query: Query<&mut Text, With<ScoreboardText>>,
) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}

fn check_for_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    start_query: Query<Entity, With<StartGameOverlay>>,
    pause_query: Query<Entity, With<PauseGameOverlay>>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    match game_state.get() {
        GameState::InGame => {
            if keyboard_input.just_released(KeyCode::Return) {
                for start_ent in &start_query {
                    commands.entity(start_ent).despawn();
                }
                next_state.set(GameState::Paused)
            }
        }
        GameState::Paused => {
            if keyboard_input.just_released(KeyCode::Return) {
                for pause_ent in &pause_query {
                    commands.entity(pause_ent).despawn();
                }
                next_state.set(GameState::InGame)
            } else {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(0.0, 0.0, 1.0),
                            scale: PAUSE_OVERLAY_SIZE,
                            ..default()
                        },
                        sprite: Sprite {
                            color: PAUSE_OVERLAY_COLOR,
                            ..default()
                        },
                        ..default()
                    },
                    PauseGameOverlay,
                ));
                commands.spawn((
                    TextBundle::from_section(
                        "ENTER to Resume",
                        TextStyle {
                            font_size: PAUSE_GAME_FONT_SIZE,
                            color: PAUSE_GAME_TEXT_COLOR,
                            ..default()
                        },
                    )
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        top: PAUSE_GAME_VERTICAL_PADDING,
                        left: PAUSE_GAME_LEFT_PADDING,
                        ..default()
                    }),
                    PauseGameOverlay,
                ));
                next_state.set(GameState::Paused)
            }
        }
    }
}

fn check_for_collisions(
    mut commands: Commands,
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    mut scoreboard: ResMut<Scoreboard>,
    collider_query: Query<(Entity, &Transform, Option<&Brick>), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
    mut explosion_events: EventWriter<ExplosionEvent>,
    game_state: Res<State<GameState>>,
) {
    if game_state.get() == &GameState::InGame {
        let (mut ball_velocity, ball_transform) = ball_query.single_mut();
        let ball_size = ball_transform.scale.truncate();

        // check collision with walls
        for (collider_entity, transform, maybe_brick) in &collider_query {
            let collision = collide(
                ball_transform.translation,
                ball_size,
                transform.translation,
                transform.scale.truncate(),
            );
            if let Some(collision) = collision {
                // Sends a collision event so that other systems can react to the collision
                collision_events.send_default();

                // Bricks should be despawned and increment the scoreboard on collision
                if maybe_brick.is_some() {
                    explosion_events.send_default();
                    scoreboard.score += 1;
                    commands.entity(collider_entity).despawn();
                }

                // reflect the ball when it collides
                let mut reflect_x = false;
                let mut reflect_y = false;

                // only reflect if the ball's velocity is going in the opposite direction of the
                // collision
                match collision {
                    Collision::Left => reflect_x = ball_velocity.x > 0.0,
                    Collision::Right => reflect_x = ball_velocity.x < 0.0,
                    Collision::Top => reflect_y = ball_velocity.y < 0.0,
                    Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                    Collision::Inside => { /* do nothing */ }
                }

                // reflect velocity on the x-axis if we hit something on the x-axis
                if reflect_x {
                    ball_velocity.x = -ball_velocity.x;
                }

                // reflect velocity on the y-axis if we hit something on the y-axis
                if reflect_y {
                    ball_velocity.y = -ball_velocity.y;
                }
            }
        }
    }
}

fn play_collision_sound(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    sound: Res<CollisionSound>,
) {
    // Play a sound once per frame if a collision occurred.
    if !collision_events.is_empty() {
        // This prevents events staying active on the next frame.
        collision_events.clear();
        commands.spawn(AudioBundle {
            source: sound.0.clone(),
            // auto-despawn the entity when playback finishes
            settings: PlaybackSettings::DESPAWN,
        });
    }
}

fn play_explosion_sound(
    mut commands: Commands,
    mut explosion_events: EventReader<ExplosionEvent>,
    sound: Res<ExplosionSound>,
) {
    // Play a sound once per frame if a explosion occurred.
    if !explosion_events.is_empty() {
        // This prevents events staying active on the next frame.
        explosion_events.clear();
        commands.spawn(AudioBundle {
            source: sound.0.clone(),
            // auto-despawn the entity when playback finishes
            settings: PlaybackSettings::DESPAWN,
        });
    }
}
