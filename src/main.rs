use bevy_prototype_lyon::entity::ShapeBundle;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::{thread_rng, Rng};

const GRAVITY: f32 = -500.;
const C_RADIUS: f32 = 12.;
const C_L_WIDTH: f32 = 2.;
const ACCELERATE: f32 = 50.;
const FRICTION: f32 = 0.80;
const NUM_OF_BALLS: i8 = 6;
const INIT_VEL: f32 = 1000.;

fn main() {
    App::build()
        .add_startup_system(setup.system())
        .add_startup_stage("player", SystemStage::single(spawn_player.system()))
        .add_startup_stage("balls", SystemStage::single(spawn_balls.system()))
        .add_system(player_movement.system())
        .add_system(ball_movement.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    windows: Res<Windows>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let window = windows.get_primary().unwrap();
    commands.insert_resource(WinSize(Vec2::new(window.width()/2.,window.height()/2.)));
}

struct WinSize(Vec2);

struct Ball;
struct Velocity(Vec2);
struct Player;

fn create_ball(pos: Vec2, line_color: Color, fill_color: Color) -> ShapeBundle {
    let mut trans = Transform::default();
    trans.translation.x = pos.x;
    trans.translation.y = pos.y;

    return GeometryBuilder::build_as(
        &shapes::Circle {
            radius: C_RADIUS,
            center: Vec2::new(0.,0.),
        },
        ShapeColors::outlined(fill_color, line_color),
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(C_L_WIDTH),
        },
        trans,
    );
}

fn spawn_player(mut commands: Commands){
    commands
        .spawn_bundle(create_ball(Vec2::new(0.,0.), Color::BLACK, Color::GREEN))
        .insert(Ball)
        .insert(Player)
        .insert(Velocity(Vec2::new(0.,0.)))
        ;
}

fn spawn_balls(
    mut commands: Commands,
    win_size: Res<WinSize>,
) {
    let mut rng = thread_rng();
    let w = win_size.0;
    for _ in 0..NUM_OF_BALLS {
        commands
            .spawn_bundle(create_ball(
                Vec2::new(rng.gen_range(-w.x, w.x) as f32,rng.gen_range(-w.y, w.y)),
                Color::GRAY, Color::BLUE))
            .insert(Ball)
            .insert(Velocity(Vec2::new(rng.gen_range(-INIT_VEL, INIT_VEL),rng.gen_range(-INIT_VEL, INIT_VEL))))
        ;
    }
}

fn ball_movement(
    mut query: Query<(&mut Transform, &mut Velocity, With<Ball>)>,
    time: Res<Time>,
    win_size: Res<WinSize>,
) {
    for (mut transform, mut velocity, _) in query.iter_mut() {
        let vel = &mut velocity.0;
        let pos = &mut transform.translation;
        let w = win_size.0;

        if      pos.x < -w.x { vel.x *= -FRICTION; pos.x = -w.x; }
        else if pos.x >  w.x { vel.x *= -FRICTION; pos.x =  w.x; }
        if      pos.y < -w.y { vel.y *= -FRICTION; pos.y = -w.y; }
        else if pos.y >  w.y { vel.y *= -FRICTION; pos.y =  w.y; }

        pos.x += vel.x * time.delta_seconds();
        pos.y += vel.y * time.delta_seconds();
        vel.y += GRAVITY * time.delta_seconds();
    }
}


fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, With<Player>)>,
) {
    for (mut velocity, _) in query.iter_mut() {
        let vel = &mut velocity.0;
        if keyboard_input.pressed(KeyCode::S) { vel.y -= ACCELERATE }
        if keyboard_input.pressed(KeyCode::W) { vel.y += ACCELERATE }
        if keyboard_input.pressed(KeyCode::A) { vel.x -= ACCELERATE }
        if keyboard_input.pressed(KeyCode::D) { vel.x += ACCELERATE }
    }
}
