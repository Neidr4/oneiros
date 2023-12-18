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
        sleep(Duration::from_millis(100));
    }

}
