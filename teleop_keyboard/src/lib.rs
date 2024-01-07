use std::sync::{Mutex, Arc};
use once_cell::sync::OnceCell;
use std::f32::consts::PI;
use std::thread;
use std::{io, thread::sleep, time::Duration};
use inputbot::KeybdKey::*;

static COMMAND_WRAPPER: OnceCell<Command> = OnceCell::new();
const SLEEP: u8 = 1;

struct Command {
    command_wrapper: Arc<Mutex<[f32; 3]>>,
}

impl Command {
    pub fn new() -> Self {
        Self {
            command_wrapper: Arc::new(Mutex::new([0.0; 3])),
        }
    }
}

// TODO: Add impl for new

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
        match COMMAND_WRAPPER.get() {
            Some(x) => x.command_wrapper.lock().unwrap().clone_from(&[(3.0*PI/4.0), 0.0, 0.0]),
            None => println!("The wrapper is not initialized"),
        }
        sleep(Duration::from_millis(SLEEP.into()));
    });

    WKey.bind(move || {
        println!("W key has been pressed");
        match COMMAND_WRAPPER.get() {
            Some(x) => x.command_wrapper.lock().unwrap().clone_from(&[PI/2.0, 0.0, 0.0]),
            None => println!("The wrapper is not initialized"),
        }
        sleep(Duration::from_millis(SLEEP.into()));
    });

    EKey.bind(move || {
        match COMMAND_WRAPPER.get() {
            Some(x) => x.command_wrapper.lock().unwrap().clone_from(&[(3.0*PI/4.0), 0.0, 0.0]),
            None => println!("The wrapper is not initialized"),
        }
        sleep(Duration::from_millis(SLEEP.into()));
    });

    AKey.bind(move || {
        match COMMAND_WRAPPER.get() {
            Some(x) => x.command_wrapper.lock().unwrap().clone_from(&[PI, 0.0, 0.0]),
            None => println!("The wrapper is not initialized"),
        }
        sleep(Duration::from_millis(SLEEP.into()));
    });

    DKey.bind(move || {
        match COMMAND_WRAPPER.get() {
            Some(x) => x.command_wrapper.lock().unwrap().clone_from(&[0.0, 0.0, 0.0]),
            None => println!("The wrapper is not initialized"),
        }
        sleep(Duration::from_millis(SLEEP.into()));
    });

    ZKey.bind(move || {
        match COMMAND_WRAPPER.get() {
            Some(x) => x.command_wrapper.lock().unwrap().clone_from(&[(5.0*PI/4.0), 0.0, 0.0]),
            None => println!("The wrapper is not initialized"),
        }
        sleep(Duration::from_millis(SLEEP.into()));
    });

    XKey.bind(move || {
        match COMMAND_WRAPPER.get() {
            Some(x) => x.command_wrapper.lock().unwrap().clone_from(&[(3.0*PI/2.0), 0.0, 0.0]),
            None => println!("The wrapper is not initialized"),
        }
        sleep(Duration::from_millis(SLEEP.into()));
    });

    CKey.bind(move || {
        match COMMAND_WRAPPER.get() {
            Some(x) => x.command_wrapper.lock().unwrap().clone_from(&[(7.0*PI/4.0), 0.0, 0.0]),
            None => println!("The wrapper is not initialized"),
        }
        sleep(Duration::from_millis(SLEEP.into()));
    });

    inputbot::handle_input_events();
    // Blocking function. Nothing will work after
}

pub fn start_teleoperation() {
    //  TODO: Add init of OnceCell here
    COMMAND_WRAPPER.get_or_init(|| Command::new() );
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
    match COMMAND_WRAPPER.get() {
        Some(x) => result = x.command_wrapper.lock().unwrap().clone(),
        None => result = [0.0, 0.0, 0.0]
    }
    // let _ = COMMAND.set([0.0, 0.0, 0.0]);
    return result
}
