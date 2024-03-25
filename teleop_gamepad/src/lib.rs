use gilrs::{Gilrs, Button, Axis, Event};
use std::ops::AddAssign;
use std::sync::{Mutex, Arc};
use once_cell::sync::OnceCell;
use std::f32::consts::PI;
use std::thread;
use std::{thread::sleep, time::Duration};

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

// TODO: Add usage

fn teleoperate() {

    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
            println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    let mut active_gamepad = None;
    let mut left_z_value: f32 = 0.0;
    let mut left_z_counter: u64 = 0;
    let mut left_axis_x: f32 = 0.0;
    let mut left_axis_y: f32 = 0.0;
    let mut angle_scalar: f32 = 0.0;
    let mut speed_scalar: f32 = 0.0;

    loop {
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            // println!("{:?} New event from {}: {:?}", time, id, event);
            active_gamepad = Some(id);
        }

        if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {

            match gamepad.axis_data(Axis::LeftZ) {
                Some(x) => {
                    if left_z_counter != x.counter() {continue;}
                    left_z_value = x.value().clone();
                    left_z_counter = x.counter();
                },
                None => println!("The gamepad is not compatible or not properly initialized Z"),
            }

            // TODO: try gamepad.button_data and is_repeating
            if gamepad.is_pressed(Button::North) {
                angle_scalar.add_assign(1e-5);
            }

            if gamepad.is_pressed(Button::East) {
                angle_scalar.add_assign(-1e-5);
            }

            if gamepad.is_pressed(Button::West) {
                speed_scalar.add_assign(1e-5);
            }

            if gamepad.is_pressed(Button::South) {
                speed_scalar.add_assign(-1e-5);
            }


            match gamepad.axis_data(Axis::LeftStickX) {
                Some(x) => { left_axis_x = x.value().clone() },
                None => println!("The gamepad is not compatible or not properly initialized X"),
            }

            match gamepad.axis_data(Axis::LeftStickY) {
                Some(x) => { left_axis_y = x.value().clone() },
                None => println!("The gamepad is not compatible or not properly initialized Y"),
            }
            
            if left_z_value != 1.0 {
                speed_scalar = 0.0;
                angle_scalar = 0.0;
            }

            match COMMAND_WRAPPER.get() {
                Some(x) => {
                    x.speed_scalar.lock().unwrap().clone_from(&speed_scalar);
                    x.angle_scalar.lock().unwrap().clone_from(&angle_scalar);
                },
                None => println!("The wrapper is not initialized"),
            }
            let direction: f32 = left_axis_x.atan2(-left_axis_y);
            COMMAND_WRAPPER.get().expect("oops").direction.lock().unwrap().clone_from(&direction);
        }
    }
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
