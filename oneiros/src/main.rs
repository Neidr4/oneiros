use std::{process::exit, thread::sleep, time::Duration};


fn main() {
    println!("Hello, world!");

    teleop_keyboard::start_teleoperation();
    let _ = control_raspberry_adapter::start_sending_to_io();

    ctrlc::set_handler(move || {
        control_raspberry_adapter::stop_sending_to_io();
        exit(1);
    }).expect("Error setting the Ctrl-C handler");

    let mut user_input: [f32; 3] = [0.0; 3];
    let mut control_output: [f32; 3] = [0.0; 3];
    loop {
        user_input = teleop_keyboard::get_user_input();
        // println!("user_input: {:?}", user_input);
        control_output = control_minimal::convert(user_input[0], user_input[1],  user_input[2]);
        control_output = control_rate_limiter::check_rate(&mut control_output);
        // println!("control_output: {:?}", control_output);
        control_raspberry_adapter::update_speed_value(control_output);
        sleep(Duration::from_millis(10));
    }
}
