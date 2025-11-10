use std::net::UdpSocket;
use bevy::prelude::*;
use std::{thread, time::Duration};
use bevy::math::Vec2;
use rand::Rng;
use serde::{Serialize, Deserialize};
use bincode;
bevy::prelude::Query;


mod shared {
    pub mod GameState;
}
use shared::GameState::*;

// fn main(mut state: ResMut<GameState>,){
//     let socket = UdpSocket::bind("127.0.0.1:12346").unwrap();
//     socket.set_nonblocking(true).unwrap(); //if there is no data, tells program to keep going
    
    
//      let mut buf= [0u8; 512];                               

//     loop{

//         let res = socket.recv_from(&mut buf);
//         //println!("Res: {:?}", res);
        
//         if let Ok((amt, _)) = res {
//             println!("Bytes RECV: {:?}", &buf[..amt]);

//             if let Ok(current_gs) = bincode::deserialize::<GameState>(&buf[..amt]) {
                
//                 *state = current_gs;
//                 println!("[Network] Updated GameState '{:?}' from Server!", state);

//             }
//         }
//     }
    

fn update_from_network(mut state: ResMut<GameState>,
){  //make sure this works 

    let socket = UdpSocket::bind("127.0.0.1:12346").unwrap();
//     socket.set_nonblocking(true).unwrap(); //if there is no data, tells program to keep going
    
    
     let mut buf= [0u8; 512];                               

    loop{

        let res = socket.recv_from(&mut buf);
        //println!("Res: {:?}", res);
        
        if let Ok((amt, _)) = res {
            println!("Bytes RECV: {:?}", &buf[..amt]);

            if let Ok(current_gs) = bincode::deserialize::<GameState>(&buf[..amt]) {
                
                *state = current_gs;
                println!("[Network] Updated GameState '{:?}' from Server!", state);

                

            }
        }
    }

}


fn save_game_state(mut gs_elements: ParamSet<( //????? ex 4 WK7 
    Query <&Transform, With<Ball>>,
    Query <&Transform, With<PlayerOne>>,
    Query <&Transform, With<PlayerTwo>>,
)>) {
    for mut Transform in param_set.p0(){
                    let gamestate= GameState{
                         ball: p0.translation.truncate(),
                         paddle_one: p1.translation.truncate(),
                         paddle_two: p2.translation.truncate(),
                    };
                }

            
    for mut Transform in param_set.p1(){
                    let gamestate= GameState{
                         paddle_one: p1.translation.truncate(),
                    };
                }
    
    for mut Transform in param_set.p2(){
                    let gamestate= GameState{
                         paddle_two: p2.translation.truncate(),
                    };
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
