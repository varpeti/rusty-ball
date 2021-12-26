use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

const GRAVITY: f32 = -500.;
const C_RADIUS: f32 = 12.;
const C_L_WIDTH: f32 = 2.;
const ACCELERATE: f32 = 50.;
const FRICTION: f32 = 0.80;

fn main() {
    App::build()
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(spawn_player.system()))
        .add_system(player_movement.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

struct Player;

struct Velocity(Vec2);

fn spawn_player(mut commands: Commands) {
    commands
        .spawn_bundle(
        GeometryBuilder::build_as(
            &shapes::Circle {
                radius: C_RADIUS,
                center: Vec2::new(0.0, 0.0),
            },
            ShapeColors::outlined(Color::GREEN, Color::BLACK),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(C_L_WIDTH),
            },
            Transform::default(),
        ))
        .insert(Player)
        .insert(Velocity(Vec2::new(0.,0.)))
    ;
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity, With<Player>)>,
    time: Res<Time>,
    windows: Res<Windows>,
) {
    for (mut transform, mut velocity, _) in query.iter_mut() {
        let vel = &mut velocity.0;
        let pos = &mut transform.translation;
        let window = windows.get_primary().unwrap();
        let wx = window.width()/2.0;
        let wy = window.height()/2.0;

        if keyboard_input.pressed(KeyCode::S) { vel.y -= ACCELERATE }
        if keyboard_input.pressed(KeyCode::W) { vel.y += ACCELERATE }
        if keyboard_input.pressed(KeyCode::A) { vel.x -= ACCELERATE }
        if keyboard_input.pressed(KeyCode::D) { vel.x += ACCELERATE }

        if      pos.x < -wx { vel.x *= -FRICTION; pos.x = -wx; }
        else if pos.x >  wx { vel.x *= -FRICTION; pos.x =  wx; }
        if      pos.y < -wy { vel.y *= -FRICTION; pos.y = -wy; }
        else if pos.y >  wy { vel.y *= -FRICTION; pos.y =  wy; }

        pos.x += vel.x * time.delta_seconds();
        pos.y += vel.y * time.delta_seconds();
        vel.y += GRAVITY * time.delta_seconds();
   }
}
