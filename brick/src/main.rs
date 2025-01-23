use std::{net::{Ipv4Addr, SocketAddrV4, TcpListener}, thread, time::Duration};

use ev3dev_lang_rust::{motors::TachoMotor, sensors::InfraredSensor, Ev3Result};

fn main() -> Ev3Result<()> {
    println!("init");
    let motor = TachoMotor::find().expect("Motor not found");
    let ir = InfraredSensor::find().expect("No IR sensor found");

    ir.set_mode_ir_prox().unwrap();

    motor.set_position(180).unwrap();
    motor.set_speed_sp(150).unwrap();
    
    thread::spawn(host);

    println!("Scanning");

    loop { 
        let rot = motor.get_position().unwrap();
        
        if rot.abs() >= 180 {
            motor.run_to_abs_pos(Some(-rot)).unwrap();
            thread::sleep(Duration::from_secs_f32(0.5));
        }
    }
}

fn host() {
    let server = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 3012)).unwrap();

    println!("Server started");

    for stream in server.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection from: {}", stream.peer_addr().unwrap());
            }
            
            Err(err) => println!("Connection failed: {}", err)
        }
    }
}