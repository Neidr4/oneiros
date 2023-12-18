use std::thread;
use std::{io, thread::sleep, time::Duration};
use inputbot::KeybdKey::*;

pub fn usage() {
    println!(" ------------------------- ");
    println!(" Call starting_teleop before doing anything");
    println!(" ------------------------- ");
    println!("Directions are \n
             Q W E\n
             A   D\n
             Z X C\n"
             );
println!("Rotations are \n
             J K"
             );
    println!(" ------------------------- ");
}

fn callback(key: String) {
    println!("{} key has been pressed", key);
}

fn teleoperate() {
    usage();

    let mut command: [f32; 3] = [0.0; 3];
    let mut command2 = vec![1.0, 2.0, 3.0];
    command[0] = 1.0;

    // TODO: Change the prints into an actual writing inside a vector
    WKey.bind(move || {
        println!("W key has been pressed");
        sleep(Duration::from_millis(100));
    });

    QKey.bind(move || {
        callback("Q".to_string());
        sleep(Duration::from_millis(100));
    });

    AKey.bind(move || {
        callback("A".to_string());
        sleep(Duration::from_millis(100));
    });

    SKey.bind(move || {
        println!("S key has been pressed");
        sleep(Duration::from_millis(100));
    });

    DKey.bind(move || {
        println!("D key has been pressed");
        sleep(Duration::from_millis(100));
    });

    inputbot::handle_input_events();
    // Blocking function. Nothing will work after
}

pub fn start_teleoperation() {
    thread::spawn(move || {
        println!("Starting the teleoperation keyboard thread");
        teleoperate();
        println!("Stopping the teleoperation keyboard thread");
    });
}
