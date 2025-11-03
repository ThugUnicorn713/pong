use bevy::prelude::*;
use avian2d::{prelude::*, PhysicsPluginsWithHooks};
//use bevy_ui::prelude::Text;
use bevy_text::Text2d; 
use std::net::UdpSocket;
use::std::{thread, time::Duration};


//after adding the UDP game is SUPPPERRRRR laggy, tried moving moving movement into the fixedUpdate didnt work

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(PhysicsPlugins::default())
    .add_systems(Startup,(setup, setup_network))
    .add_systems(Update, (move_player_one, move_player_two, player_score_check))
    .add_systems(FixedUpdate, handle_network_input)
    .insert_resource(MovementSettings{speed: 300.0})
    .insert_resource(Scores {player_one: 0, player_two: 0})
    .run();
}

const SCREEN_LIMIT_X: f32 = 600.0;

#[derive(Component)]
struct PlayerOne;

#[derive(Component)]
struct PlayerTwo;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct  PlayerOneScoreText;

#[derive(Component)]
struct  PlayerTwoScoreText;

#[derive(Resource)]
struct MovementSettings{
    speed: f32
}

#[derive(Resource)]
struct Scores{
    player_one: u32,
    player_two: u32,
}

#[derive(Resource)]
struct NetworkSocket {
    socket: UdpSocket,
    target: String,
}

fn setup_network(mut commands: Commands){
    let socket = UdpSocket::bind("127.0.0.1:12345").unwrap();
    socket.set_nonblocking(true).unwrap();

    commands.insert_resource(NetworkSocket {
        socket,
        target: "127.0.0.1:12346".to_string(),
    });
}

fn handle_network_input(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<PlayerTwo>>,
    socket_res: Option<Res<NetworkSocket>>, 
){
    
    if socket_res.is_none(){
        return;
    }

    let socket = &socket_res.unwrap().socket;
    let mut buf = [0u8; 8];

     if let Ok((amt, _src)) = socket.recv_from(&mut buf) {
        if let Ok(msg) = std::str::from_utf8(&buf[..amt]) {
            if let Ok(value) = msg.trim().parse::<i32>() {
                let mut dir = 0.0;
                if value == 1 {
                    dir = 1.0;
                } else if value == -1 {
                    dir = -1.0;
                }

                for mut transform in &mut query {
                    transform.translation.y += dir * 50.0 * time.delta_secs(); 
                }
            }
        }
    }

    thread::sleep(Duration::from_millis(50));

}

fn setup (mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
){
    commands.spawn(Camera2d);

     commands.spawn(
        (Mesh2d(mesh.add(Rectangle::new(50.0, 80.0))),
        MeshMaterial2d(material.add(Color::srgb(0.5, 0.3, 0.9))),
        RigidBody::Kinematic,
        Collider::rectangle(50.0, 80.0),
        Transform::from_xyz(-500.0, 0.0, 0.0),
        PlayerOne,)
    );

     commands.spawn(
          (Mesh2d(mesh.add(Rectangle::new(50.0, 80.0))),
        MeshMaterial2d(material.add(Color::srgb(0.1, 0.7, 0.3))),
        RigidBody::Kinematic,
        Collider::rectangle(50.0, 80.0),
        Transform::from_xyz(500.0, 0.0, 0.0),
        PlayerTwo,)

    );

     commands.spawn(
        (Mesh2d(mesh.add(Circle::new(15.0))),
        MeshMaterial2d(material.add(Color::WHITE)),
        RigidBody::Dynamic,
        Collider::circle(15.0),
        Transform::from_xyz(0.0, 0.0, 0.0),
        LinearVelocity(Vec2::new(200.0, 50.0)),
        GravityScale(0.0),
        Restitution::new(1.0),
        Ball,
        CollisionEventsEnabled,)
    )
    .observe(|trigger: Trigger<OnCollisionStart>,
        mut ball_query: Query<&mut LinearVelocity, With<Ball>>,
         player_query: Query<(), Or<(With<PlayerOne>, With<PlayerTwo>)>>,
         wall_query: Query<&Wall>|{

        let ball = trigger.target();
        let player = trigger.collider;
        let wall = trigger.collider;

        if player_query.contains(player){
            if let Ok(mut velocity) = ball_query.get_mut(ball){
                velocity.x = -velocity.x.abs() * 3.0;
            }
        }

        if wall_query.contains(wall){
            if let Ok(mut velocity) = ball_query.get_mut(ball){
                velocity.y = -velocity.y.abs() * 3.0;
            }
        }
    });

    commands.spawn(
        (Mesh2d(mesh.add(Rectangle::new(1000.0, 30.0))),
        MeshMaterial2d(material.add(Color::srgb(5.0,0.0,0.0))),
        RigidBody::Static,
        Collider::rectangle(1000.0, 30.0),
        Transform::from_xyz(0.0,350.0 , 0.0),
        Wall,)
);

     commands.spawn(
        (Mesh2d(mesh.add(Rectangle::new(1000.0, 30.0))),
        MeshMaterial2d(material.add(Color::srgb(5.0,0.0,0.0))),
        RigidBody::Static,
        Collider::rectangle(1000.0, 30.0),
        Transform::from_xyz(0.0,-350.0 , 0.0),
        Wall,)
    );


    commands.spawn((
        Text2d::new("SCORE"),
        Transform::from_xyz(0.0, 300.0, 0.0),
        ScoreText,
    ));

     commands.spawn((
        Text2d::new("0"),
        Transform::from_xyz(-50.0, 250.0, 0.0),
        PlayerOneScoreText,
    ));

     commands.spawn((
        Text2d::new("0"),
        Transform::from_xyz(50.0, 250.0, 0.0),
        PlayerTwoScoreText,
    ));



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

fn player_score_check(
    mut query: Query<(&mut Transform, &mut LinearVelocity), With<Ball>>,
      mut score_texts: ParamSet<(
        Query<&mut Text2d, With<PlayerOneScoreText>>,  //tells bevy they both borrow Text2d but not at the same time
        Query<&mut Text2d, With<PlayerTwoScoreText>>,
    )>,
   // mut score_query_one: Query<&mut Text2d, With<PlayerOneScoreText>>,  //bevy freaked out about this,
    //mut score_query_two: Query<&mut Text2d, With<PlayerTwoScoreText>>,   //something to do with query mismatch?
    mut scores: ResMut<Scores>,
){
    for (mut transform, mut velocity) in &mut query{

        if transform.translation.x > SCREEN_LIMIT_X {
            
            scores.player_one += 1;
            
            if let Ok(mut text) = score_texts.p0().single_mut(){
                text.0 = scores.player_one.to_string();
            }

            transform.translation = Vec3::ZERO;
            velocity.x = -velocity.x.abs();
            velocity.y = 50.0;
        }

        
        if transform.translation.x < -SCREEN_LIMIT_X {
            
            scores.player_two += 1;
            
            if let Ok(mut text) = score_texts.p1().single_mut(){
                text.0 = scores.player_two.to_string();
            }

            transform.translation = Vec3::ZERO;
            velocity.x = velocity.x.abs();
            velocity.y = 50.0;
        }
    }

}

