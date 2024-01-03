use std::f32::consts::PI;
use std::sync::OnceLock;
use std::thread;
use std::{io, thread::sleep, time::Duration};
use inputbot::KeybdKey::*;

static COMMAND: OnceLock<[f32; 3]> = OnceLock::new();
const SLEEP: u8 = 1;

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
    QKey.bind(move || {
        let _ = COMMAND.set([(3.0*PI/4.0), 0.0, 0.0]);
        sleep(Duration::from_millis(SLEEP.into()));
    });

    WKey.bind(move || {
        println!("W key has been pressed");
        let _ = COMMAND.set([PI/2.0, 0.0, 0.0]);
        sleep(Duration::from_millis(SLEEP.into()));
    });

    EKey.bind(move || {
        let _ = COMMAND.set([(3.0*PI/4.0), 0.0, 0.0]);
        sleep(Duration::from_millis(SLEEP.into()));
    });

    AKey.bind(move || {
        let _ = COMMAND.set([PI, 0.0, 0.0]);
        sleep(Duration::from_millis(SLEEP.into()));
    });

    DKey.bind(move || {
        let _ = COMMAND.set([0.0, 0.0, 0.0]);
        sleep(Duration::from_millis(SLEEP.into()));
    });

    ZKey.bind(move || {
        let _ = COMMAND.set([(5.0*PI/4.0), 0.0, 0.0]);
        sleep(Duration::from_millis(SLEEP.into()));
    });

    XKey.bind(move || {
        let _ = COMMAND.set([(3.0*PI/2.0), 0.0, 0.0]);
        sleep(Duration::from_millis(SLEEP.into()));
    });

    CKey.bind(move || {
        let _ = COMMAND.set([(7.0*PI/4.0), 0.0, 0.0]);
        sleep(Duration::from_millis(SLEEP.into()));
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
    let mut result: [f32; 3] = [0.0; 3];
    println!("{:?}", COMMAND.get().clone());
    match COMMAND.take() {
        Some(x) => result = x,
        None => result = [0.0, 0.0, 0.0]
    }
    // let _ = COMMAND.set([0.0, 0.0, 0.0]);
    let _ = COMMAND = OnceLock::new();
    return result
}
