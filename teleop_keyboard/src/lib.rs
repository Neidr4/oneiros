use std::f32::consts::PI;
use std::sync::OnceLock;
use std::thread;
use std::{io, thread::sleep, time::Duration};
use inputbot::KeybdKey::*;

static COMMAND: OnceLock<[f32; 3]> = OnceLock::new();

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

    // TODO: Add the correct values in in functions
    // TODO: Add the ongoing speed
    // TODO: Add SPACE for stop
    // TODO: Add ESC for stop
    // TODO: Set the sleep duration as a constant
    WKey.bind(move || {
        println!("W key has been pressed");
        let _ = COMMAND.set([PI/2.0, 0.0, 0.0]);
        sleep(Duration::from_millis(100));
    });

    QKey.bind(move || {
        callback("Q".to_string());
        let _ = COMMAND.set([1.0, 0.0, 0.0]);
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

// TODO: create a stop function

// Get the user input in the form: [dir, speed_scalar, angle_scalar]
pub fn get_user_input() -> [f32; 3] {
    // TODO: Should it return NONE when not init? Force the user to take care of it ?
    match COMMAND.clone().into_inner() {
        Some(x) => return x,
        None => return [0.0, 0.0, 0.0]
    }
}
