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
const BALL_SPEED: f32 = 200.0;
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

#[derive(Component)]
struct Brick;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_event::<CollisionEvent>()
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                check_for_collisions,
                apply_velocity.before(check_for_collisions),
                move_paddle
                    .before(check_for_collisions)
                    .after(apply_velocity),
            ),
        )
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
                Collider
            ));
            i2 += 1.;
        }
        i += 1.;
    }

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

    paddle_transform.translation.x =
        new_paddle_position.clamp(LEFT_BOUND_PADDLE, RIGHT_BOUND_PADDLE);
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<FixedTime>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.period.as_secs_f32();
        transform.translation.y += velocity.y * time_step.period.as_secs_f32();
    }
}

fn check_for_collisions(
    mut commands: Commands,
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(Entity, &Transform, Option<&Brick>), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {

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
                //scoreboard.score += 1;
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
