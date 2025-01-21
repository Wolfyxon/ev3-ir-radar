use std::time::{Duration, SystemTime, UNIX_EPOCH};

use ev3dev_lang_rust::{motors::TachoMotor, sensors::InfraredSensor, Ev3Result};

fn main() -> Ev3Result<()> {
    println!("init");
    let motor = TachoMotor::find().expect("Motor not found");
    let ir = InfraredSensor::find().expect("No IR sensor found");

    ir.set_mode_ir_prox().unwrap();

    motor.set_position(180).unwrap();
    motor.set_speed_sp(5).unwrap();

    loop {
        let rot = motor.get_position().unwrap();
        
        if rot.abs() <= 180 {
            motor.run_to_abs_pos(Some(-rot)).unwrap();
        }
    }
}
