use bevy::{ecs::query, prelude::*};
use bevy_rapier2d::prelude::*;
use rand::Rng;

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0)))
    .add_plugins(RapierDebugRenderPlugin::default()) // see colliders for testing
    .add_systems(Startup,setup)
    .add_systems(Update, (move_player_one, move_player_two))
    .insert_resource(MovementSettings{speed: 300.0})
    .run();
}

#[derive(Component)]
struct PlayerOne;

#[derive(Component)]
struct PlayerTwo;

#[derive(Component)]
struct Ball;

#[derive(Resource)]
struct MovementSettings{
    speed: f32
}

fn setup (mut commands: Commands,
mut mesh: ResMut<Assets<Mesh>>,
mut material: ResMut<Assets<ColorMaterial>>
){
    commands.spawn(Camera2d);

    commands.spawn(
        (Mesh2d(mesh.add(Rectangle::new(50.0, 80.0))),
        MeshMaterial2d(material.add(Color::srgb(0.5, 0.3, 0.9))),
        Transform::from_xyz(-500.0, 0.0, 0.0),
        RigidBody::KinematicPositionBased,
        Collider::cuboid(25.0, 40.0),
        PlayerOne,)
    );

    commands.spawn(
          (Mesh2d(mesh.add(Rectangle::new(50.0, 80.0))),
        MeshMaterial2d(material.add(Color::srgb(0.1, 0.7, 0.3))),
        Transform::from_xyz(500.0, 0.0, 0.0),
        RigidBody::KinematicPositionBased,
        Collider::cuboid(25.0, 40.0),
        PlayerTwo,)

    );

    let mut rng = rand::thread_rng(); //babys first THREAD!!!

    let direction_x = if rng.gen_bool(0.5) {1.0} else {-1.0};
    let direction = Vec2::new(direction_x, 0.3).normalize(); //give it a lil angle

    commands.spawn(
        Mesh2d(mesh.add(Circle::new(15.0))),
        MeshMaterial2d(material.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Dynamic,
        Collider::ball(15.0),
        Restitution::coefficient(1.0), //makes it go boing
        Velocity::linear(direction * 400.0),
        Friction::coefficient(1.0),
        Ball,
    );

    let wall_thiccness = 20.0;
    let wall_width = 1200.0;

    commands.spawn(
        Collider::cuboid(wall_width/ 2.0, wall_thiccness / 2.0),
        RigidBody::Fixed,
        Transform::from_xyz(0.0, 350.0, 0.0),
    );

     commands.spawn(
        Collider::cuboid(wall_width/ 2.0, wall_thiccness / 2.0),
        RigidBody::Fixed,
        Transform::from_xyz(0.0, -350.0, 0.0),
    );



}


fn move_player_one(
    key: Res<ButtonInput<KeyCode>>,
    settings: Res<MovementSettings>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<PlayerOne>>
){

    let mut direction = Vec3::ZERO;

    if key.pressed(KeyCode::KeyW){
        direction.y += 1.0;
    }

    if key.pressed(KeyCode::KeyS){
        direction.y -= 1.0;
    }

    for mut pos in &mut query {
        pos.translation += direction * settings.speed * time.delta_secs();

    }
}

fn move_player_two(
    key: Res<ButtonInput<KeyCode>>,
    settings: Res<MovementSettings>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<PlayerTwo>>
){

    let mut direction = Vec3::ZERO;

    if key.pressed(KeyCode::ArrowUp){
        direction.y += 1.0;
    }

    if key.pressed(KeyCode::ArrowDown){
        direction.y -= 1.0;
    }

    for mut pos in &mut query {
        pos.translation += direction * settings.speed * time.delta_secs();

    }
}