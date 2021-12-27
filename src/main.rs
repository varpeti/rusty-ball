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
        .add_event::<BallCollision>()
        .add_system(player_movement.system())
        .add_system(ball_movement.system())
        .add_system(ball_collision_detection.system())
        .add_system(ball_collision.system())
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
    mut query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    time: Res<Time>,
    win_size: Res<WinSize>,
) {
    for (mut transform, mut velocity) in query.iter_mut() {
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
    mut query: Query<&mut Velocity, With<Player>>,
) {
    for mut velocity in query.iter_mut() {
        let vel = &mut velocity.0;
        if keyboard_input.pressed(KeyCode::S) { vel.y -= ACCELERATE }
        if keyboard_input.pressed(KeyCode::W) { vel.y += ACCELERATE }
        if keyboard_input.pressed(KeyCode::A) { vel.x -= ACCELERATE }
        if keyboard_input.pressed(KeyCode::D) { vel.x += ACCELERATE }
    }
}

#[allow(unused)]
struct BallCollision {
    ent_a: Entity, pos_a: Vec2,
    ent_b: Entity, pos_b: Vec2,
    is_collided: bool, angle: f32,
}
impl BallCollision {
    pub fn new(ent_a: Entity, pos_a: Vec2, radius_a: f32, ent_b: Entity, pos_b: Vec2, radius_b: f32) -> BallCollision {
        let dx = pos_b.x - pos_a.x;
        let dy = pos_b.y - pos_a.y;
        let ar = radius_a + radius_b;
        let is_collided = (dx*dx + dy*dy) < (ar*ar);

        let mut angle = 0.;
        if is_collided {
            angle = dx.atan2(dy);
        }

        BallCollision {
            ent_a: ent_a, pos_a: pos_a,
            ent_b: ent_b, pos_b: pos_b,
            is_collided: is_collided, angle: angle,
        }
    }
}


fn ball_collision_detection(
    queries: QuerySet<(Query<(Entity, &Transform), With<Ball>>,
                           Query<(Entity, &Transform), With<Ball>>)>,
    mut ev_collision: EventWriter<BallCollision>,
) {

    for (e0, t0) in queries.q0().iter() {
        for (e1, t1) in queries.q1().iter() {
            if e0 == e1 {continue;}

            let bc = BallCollision::new(
                e0, t0.translation.truncate(), C_RADIUS,
                e1, t1.translation.truncate(), C_RADIUS
            );
            if bc.is_collided {
                ev_collision.send(bc);
            }
        }
    }
}

fn ball_collision(
    mut ev_collision: EventReader<BallCollision>,
    mut query: Query<(Entity, &mut Velocity), With<Ball>>,
) {
    for ev in ev_collision.iter() {
        let dist_a: f32;
        let dist_b: f32;
        if let Ok((_, velocity)) = query.get_mut(ev.ent_a) {
            dist_a = velocity.0.length();
        } else {return};
        if let Ok((_, mut velocity)) = query.get_mut(ev.ent_b) {
            dist_b = velocity.0.length();
            velocity.0.x = ev.angle.sin() * (dist_a + dist_b) * 0.5;
            velocity.0.y = ev.angle.cos() * (dist_a + dist_b) * 0.5;
        }
    }
}
