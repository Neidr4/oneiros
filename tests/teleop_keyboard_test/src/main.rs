use std::{thread::sleep, time::Duration};

fn main() {
    println!("Testing teleop_keyboard");

    teleop_keyboard::usage();

    // teleop_keyboard::teleoperate();
    teleop_keyboard::start_teleoperation();

    println!("After the teleop function");

    for _i in 0..100 {
        println!("Still listening for stuff here {:?} over {:?}", _i, 100);
        // Fetch the data
        let user_input: [f32; 3] = teleop_keyboard::get_user_input();
        println!("Value returned is {:?}", user_input);
        sleep(Duration::from_millis(100));
    }

}
