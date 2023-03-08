extern crate rand;
use rand::Rng;
use std::{net::UdpSocket, thread, time};

struct Thermometer {}

impl Thermometer {
    fn get_temperature(&self) -> f32 {
        let f1: f32 = rand::thread_rng().gen_range(19.0..22.0);
        f1
    }
}

fn main() {
    let thermometer = Thermometer {};
    let addr = "127.0.0.1:5505";
    let addr_to = "127.0.0.1:5507";
    let socket = UdpSocket::bind(addr).unwrap();
    println!("Started at {}", addr);

    loop {
        let temp = thermometer.get_temperature();
        println!("Temperature: {}", temp);
        socket.send_to(&temp.to_be_bytes(), addr_to).unwrap();
        thread::sleep(time::Duration::from_millis(500));
    }
}
