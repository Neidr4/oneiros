use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    println!("About to start!");
    let _ = control_raspberry_adapter::start_sending_to_io();
    println!("About to update!");
    control_raspberry_adapter::update_speed_value([0.2; 3]);
    thread::sleep(Duration::from_secs(5));
    println!("About to update!");
    let motor_speeds: [f32; 3] = [1.0; 3];
    control_raspberry_adapter::update_speed_value(motor_speeds);
    thread::sleep(Duration::from_secs(5));
    control_raspberry_adapter::stop_sending_to_io();
}
