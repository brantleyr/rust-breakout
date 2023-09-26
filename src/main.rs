use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

// Constants
const GRID_HEIGHT: f32 = 5.;
const GRID_WIDTH: f32 = 10.;
const GRID_CELL_SPACE: f32 = 5.;
const GRID_CELL_WIDTH: f32 = 80.;
const GRID_CELL_HEIGHT: f32 = 30.;
const GRID_CELL_TOP: f32 = 300.;
const GRID_CELL_LEFT: f32 = -350.;
const BACKGROUND_COLOR: Color = Color::rgb(0., 0., 0.25);
const LEFT_WALL: f32 = -400.;
const RIGHT_WALL: f32 = 465.;
const TOP_WALL: f32 = 325.;
const BOTTOM_WALL: f32 = -325.;
const WALL_COLOR: Color = Color::rgb(0., 0.75, 0.);
const LR_WALL_LENGTH: f32 = 650.;
const TB_WALL_LENGTH: f32 = 875.;
const TB_WALL_ADJUST: f32 = 32.5;
const WALL_SIZE: f32 = 10.;
const BALL_START_X: f32 = 32.5;
const BALL_START_Y: f32 = -270.;
const BALL_SIZE: f32 = 12.5;
const BALL_COLOR: Color = Color::PURPLE;
const PADDLE_START_X: f32 = 32.5;
const PADDLE_START_Y: f32 = -300.;
const PADDLE_HEIGHT: f32 = 20.;
const PADDLE_WIDTH: f32 = 125.;
const PADDLE_COLOR: Color = Color::ORANGE;
const PADDLE_SPEED: f32 = 10.0;
const LEFT_BOUND_PADDLE: f32 = LEFT_WALL + WALL_SIZE + (PADDLE_WIDTH / 2.);
const RIGHT_BOUND_PADDLE: f32 = RIGHT_WALL - WALL_SIZE - (PADDLE_WIDTH / 2.);

#[derive(Component)]
struct Paddle;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, move_paddle)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Draw walls
    // Left
    commands.spawn(
        SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                custom_size: Some(Vec2::new(WALL_SIZE, LR_WALL_LENGTH)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(LEFT_WALL, 0., 0.)),
            ..default()
        });
    // Right
    commands.spawn(
        SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                custom_size: Some(Vec2::new(WALL_SIZE, LR_WALL_LENGTH)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(RIGHT_WALL, 0., 0.)),
            ..default()
        });
    // Top
    commands.spawn(
        SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                custom_size: Some(Vec2::new(TB_WALL_LENGTH, WALL_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(TB_WALL_ADJUST, TOP_WALL, 0.)),
            ..default()
        });
    // Bottom
    commands.spawn(
        SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                custom_size: Some(Vec2::new(TB_WALL_LENGTH, WALL_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(TB_WALL_ADJUST, BOTTOM_WALL, 0.)),
            ..default()
        });

    // Draw Grid
    let mut i = 0.;
    while i < GRID_HEIGHT {
        let mut i2 = 0.;
        while i2 < GRID_WIDTH { 
            let grid_cell_top = GRID_CELL_TOP - (i*GRID_CELL_HEIGHT) - (i*GRID_CELL_SPACE);
            let grid_cell_left = GRID_CELL_LEFT + (i2*GRID_CELL_WIDTH) + (i2*GRID_CELL_SPACE);
            println!("{},{}", grid_cell_top, grid_cell_left);
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25+(i/10.), 0.75, 0.25+(i/10.)),
                    custom_size: Some(Vec2::new(GRID_CELL_WIDTH, GRID_CELL_HEIGHT)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(grid_cell_left, grid_cell_top, 0.)),
                ..default()
            });
            i2 = i2 + 1.;
        }
        i = i + 1.;
    }


    // Draw Ball
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(BALL_SIZE).into()).into(),
        material: materials.add(ColorMaterial::from(BALL_COLOR)),
        transform: Transform::from_translation(Vec3::new(BALL_START_X, BALL_START_Y, 0.)),
        ..default()
    });

    // Draw Paddle
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: PADDLE_COLOR,
            custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(PADDLE_START_X, PADDLE_START_Y, 0.)),
        ..default()
    },Paddle));
}

fn move_paddle(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Paddle>>,
) {
    let mut direction = 0.;
    if keyboard_input.pressed(KeyCode::Left) {
        direction = -1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        direction = 1.0;
    }
    let mut paddle_transform = query.single_mut();
    let new_paddle_position = paddle_transform.translation.x + (direction * PADDLE_SPEED);

    paddle_transform.translation.x = new_paddle_position.clamp(LEFT_BOUND_PADDLE, RIGHT_BOUND_PADDLE);
}
