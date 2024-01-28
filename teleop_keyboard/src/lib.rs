use std::ops::AddAssign;
use std::sync::{Mutex, Arc};
use once_cell::sync::OnceCell;
use std::f32::consts::PI;
use std::thread;
use std::{thread::sleep, time::Duration};
use inputbot::KeybdKey::*;

static COMMAND_WRAPPER: OnceCell<Command> = OnceCell::new();
const SLEEP: u8 = 1;

struct Command {
    direction: Arc<Mutex<f32>>,
    speed_scalar: Arc<Mutex<f32>>,
    angle_scalar: Arc<Mutex<f32>>
}

impl Command {
    pub fn new() -> Self {
        Self {
            direction: Arc::new(Mutex::new(0.0)),
            speed_scalar: Arc::new(Mutex::new(0.0)),
            angle_scalar: Arc::new(Mutex::new(0.0))
        }
    }
}

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

fn teleoperate() {
    usage();

    QKey.bind(move || {
        match COMMAND_WRAPPER.get() {
            Some(x) => x.direction.lock().unwrap().clone_from(&(3.0*PI/4.0)),
            None => println!("The wrapper is not initialized"),
        }
        sleep(Duration::from_millis(SLEEP.into()));
    });

    WKey.bind(move || {
        match COMMAND_WRAPPER.get() {
            Some(x) => x.direction.lock().unwrap().clone_from(&(PI/2.0)),
            None => println!("The wrapper is not initialized"),
        }
        sleep(Duration::from_millis(SLEEP.into()));
    });

    EKey.bind(move || {
        match COMMAND_WRAPPER.get() {
            Some(x) => x.direction.lock().unwrap().clone_from(&(PI/4.0)),
            None => println!("The wrapper is not initialized"),
        }
        sleep(Duration::from_millis(SLEEP.into()));
    });

    AKey.bind(move || {
        match COMMAND_WRAPPER.get() {
            Some(x) => x.direction.lock().unwrap().clone_from(&PI),
            None => println!("The wrapper is not initialized"),
        }
        sleep(Duration::from_millis(SLEEP.into()));
    });

    DKey.bind(move || {
        match COMMAND_WRAPPER.get() {
            Some(x) => x.direction.lock().unwrap().clone_from(&0.0),
            None => println!("The wrapper is not initialized"),
        }
        sleep(Duration::from_millis(SLEEP.into()));
    });

    ZKey.bind(move || {
        match COMMAND_WRAPPER.get() {
            Some(x) => x.direction.lock().unwrap().clone_from(&(5.0*PI/4.0)),
            None => println!("The wrapper is not initialized"),
        }
        sleep(Duration::from_millis(SLEEP.into()));
    });

    XKey.bind(move || {
        match COMMAND_WRAPPER.get() {
            Some(x) => x.direction.lock().unwrap().clone_from(&(3.0*PI/2.0)),
            None => println!("The wrapper is not initialized"),
        }
        sleep(Duration::from_millis(SLEEP.into()));
    });

    CKey.bind(move || {
        match COMMAND_WRAPPER.get() {
            Some(x) => x.direction.lock().unwrap().clone_from(&(7.0*PI/4.0)),
            None => println!("The wrapper is not initialized"),
        }
        sleep(Duration::from_millis(SLEEP.into()));
    });

    JKey.bind(move || {
        match COMMAND_WRAPPER.get() {
            Some(x) => x.angle_scalar.lock().unwrap().add_assign(-0.1),
            None => println!("The wrapper is not initialized"),
        }
        sleep(Duration::from_millis(SLEEP.into()));
    });

    KKey.bind(move || {
        match COMMAND_WRAPPER.get() {
            Some(x) => x.angle_scalar.lock().unwrap().add_assign(0.1),
            None => println!("The wrapper is not initialized"),
        }
        sleep(Duration::from_millis(SLEEP.into()));
    });

    MKey.bind(move || {
        match COMMAND_WRAPPER.get() {
            Some(x) => x.speed_scalar.lock().unwrap().add_assign(-0.1),
            None => println!("The wrapper is not initialized"),
        }
        sleep(Duration::from_millis(SLEEP.into()));
    });

    IKey.bind(move || {
        match COMMAND_WRAPPER.get() {
            Some(x) => x.speed_scalar.lock().unwrap().add_assign(0.1),
            None => println!("The wrapper is not initialized"),
        }
        sleep(Duration::from_millis(SLEEP.into()));
    });

    SpaceKey.bind(move || {
        match COMMAND_WRAPPER.get() {
            Some(x) => {
                x.direction.lock().unwrap().clone_from(&0.0);
                x.speed_scalar.lock().unwrap().clone_from(&0.0);
                x.angle_scalar.lock().unwrap().clone_from(&0.0);
            }
            None => println!("The wrapper is not initialized"),
        }
    });

    EscapeKey.bind(move || {
        match COMMAND_WRAPPER.get() {
            Some(x) => {
                x.direction.lock().unwrap().clone_from(&0.0);
                x.speed_scalar.lock().unwrap().clone_from(&0.0);
                x.angle_scalar.lock().unwrap().clone_from(&0.0);
            }
            None => println!("The wrapper is not initialized"),
        }
    });

    inputbot::handle_input_events();
    // Blocking function. Nothing will work after
}

pub fn start_teleoperation() {
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
    let result: [f32; 3];
    match COMMAND_WRAPPER.get() {
        Some(x) => result = [x.direction.lock().unwrap().clone(),
                             x.speed_scalar.lock().unwrap().clone(),
                             x.angle_scalar.lock().unwrap().clone()
                             ],
        None => result = [0.0, 0.0, 0.0]
    }
    return result.clone()
}
