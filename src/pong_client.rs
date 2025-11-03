use std::net::UdpSocket;
use std::{thread, time::Duration};
use rand::Rng;

fn main(){
    let socket = UdpSocket::bind("127.0.0.1:12346").unwrap();
    socket.set_nonblocking(true).unwrap(); //if there is no data, tells program to keep going
    let target = "127.0.0.1:12345";

    loop {
        let value = if rand::thread_rng().gen_bool(0.5) { 1 } else { -1 }; //gen_bool randomly retunns true or false
        let msg = value.to_string();

        socket.send_to(msg.as_bytes(), target).unwrap();
        println!("Sent {}", msg);

        thread::sleep(Duration::from_millis(500));
    }
}