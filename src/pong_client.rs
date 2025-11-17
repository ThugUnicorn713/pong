use std::net::UdpSocket;
use bevy::prelude::*;
use std::{thread, time::Duration};
//use bevy::math::Vec2;
//use rand::Rng;
use serde::{Serialize, Deserialize};
use bincode;
use bevy::prelude::Query;

use pong::shared::GameState::*;
use pong::shared::Components::*;

// mod shared {

//     pub mod GameState;
// }
// use shared::GameState::*;

//  use crate::shared::GameState::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameState::default())
        .add_systems(Update, (update_from_network, save_game_state,))
        .run();
}
    

pub fn update_from_network(mut state: ResMut<GameState>,
){  //make sure this works 

    let socket = UdpSocket::bind("127.0.0.1:12346").unwrap(); //put this as a resource at some point
    socket.set_nonblocking(true).unwrap();

    let mut buf = [0u8; 512];

    if let Ok((amt, _)) = socket.recv_from(&mut buf) {
        if let Ok(current_gs) = bincode::deserialize::<GameState>(&buf[..amt]) {

            *state = current_gs;
            println!("[Network] Updated GameState: {:?}", state);
        }
    }
    

}


pub fn save_game_state(mut param_set: ParamSet<(
    Query <&mut Transform, With<Ball>>,
    Query <&mut Transform, With<PlayerOne>>,
    Query <&mut Transform, With<PlayerTwo>>,
)>,

    state: Res<GameState>,
){
    if let Ok(mut transform) = param_set.p0().single_mut(){

        transform.translation.x = state.ball.x;
        transform.translation.y = state.ball.y;

    }

    if let Ok(mut transform) = param_set.p1().single_mut(){

        transform.translation.x  = state.paddle_one.x;
        transform.translation.y = state.paddle_one.y;

    }
    
     if let Ok(mut transform) = param_set.p2().single_mut(){

        transform.translation.x = state.paddle_two.x;
        transform.translation.y = state.paddle_two.y;
    }
}
    
    
    //OG UDP loop
    
    //let target = "127.0.0.1:12345";
    
    // loop {
    //     let value = if rand::rng().random_bool(0.5) { 1 } else { -1 }; //random_bool randomly retunns true or false
    //     let msg = value.to_string();

    //     socket.send_to(msg.as_bytes(), target).unwrap();
    //     println!("Sent {}", msg);

    //     thread::sleep(Duration::from_millis(500));
    // }

