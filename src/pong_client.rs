use std::net::UdpSocket;
use std::{thread, time::Duration};
use rand::Rng;
use serde::{Serialize, Deserialize};
use bincode;

mod shared {
    pub mod GameState;
}
use shared::GameState::*;

fn main(){
    let socket = UdpSocket::bind("127.0.0.1:12346").unwrap();
    socket.set_nonblocking(true).unwrap(); //if there is no data, tells program to keep going
    
    
     let mut buf= [0u8; 512];                               

    loop{

        let res = socket.recv_from(&mut buf);
        //println!("Res: {:?}", res);
        
        if let Ok((amt, _)) = res {
            println!("Bytes RECV: {:?}", &buf[..amt]);

            if let Ok(current_gs) = bincode::deserialize::<GameState>(&buf[..amt]) {

                println!("Client received '{:?}' from Server!", current_gs);

            }
        }
    }
    
    
    
    
    
    
    //let target = "127.0.0.1:12345";

    
    
    
    
    
    // loop {
    //     let value = if rand::rng().random_bool(0.5) { 1 } else { -1 }; //random_bool randomly retunns true or false
    //     let msg = value.to_string();

    //     socket.send_to(msg.as_bytes(), target).unwrap();
    //     println!("Sent {}", msg);

    //     thread::sleep(Duration::from_millis(500));
    // }
}