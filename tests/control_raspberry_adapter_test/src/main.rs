use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    println!("About to start!");
    let _ = control_raspberry_adapter::start_sending_to_io();

    println!("About to update!");
    control_raspberry_adapter::update_speed_value([1.0, 0.5, 0.1]);
    thread::sleep(Duration::from_secs(5));

    println!("About to update!");
    control_raspberry_adapter::update_speed_value([-0.1, -0.5, -1.0]);
    thread::sleep(Duration::from_secs(5));

    println!("About to update!");
    control_raspberry_adapter::update_speed_value([1.0, 0.5, 0.1]);
    thread::sleep(Duration::from_secs(5));

    control_raspberry_adapter::stop_sending_to_io();
}
