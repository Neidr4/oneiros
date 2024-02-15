use std::{process::exit, thread::sleep, time::Duration};
use std::f32::consts::PI;

fn user_control() {
    let mut user_input: [f32; 3] = [0.0; 3];
    let mut control_output: [f32; 3] = [0.0; 3];
    loop {
        user_input = teleop_keyboard::get_user_input();
        // println!("user_input: {:?}", user_input);
        control_output = control_minimal::convert(user_input[0], user_input[1],  user_input[2]);
        // println!("control_output: {:?}", control_output);
        control_raspberry_adapter::update_speed_value(control_output);
        sleep(Duration::from_millis(10));
    }
}

fn shape() {
    let mut user_input: [f32; 3] = [0.0; 3];
    let mut control_output: [f32; 3] = [0.0; 3];
    // let commands: [f32; 4] = [0.0, PI/2.0, PI, 3.0*PI/2.0];
    let commands: [f32; 4] = [PI/8.0, 3.0*PI/4.0, 5.0*PI/4.0, 7.0*PI/4.0];
    let speed: f32 = 0.3;
    let sleep_time = Duration::from_secs(2);
    loop {
        for command in commands.iter() {
            control_output = control_minimal::convert(*command, speed, 0.0);
            control_raspberry_adapter::update_speed_value(control_output);
            sleep(sleep_time);
        }
    }
}

fn main() {
    println!("Hello, world!");

    teleop_keyboard::start_teleoperation();
    let _ = control_raspberry_adapter::start_sending_to_io();

    ctrlc::set_handler(move || {
        control_raspberry_adapter::stop_sending_to_io();
        exit(1);
    }).expect("Error setting the Ctrl-C handler");

    user_control();
    // shape();
}
