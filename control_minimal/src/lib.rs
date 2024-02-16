use std::f32::consts::PI;
use std::sync::{Mutex, Arc};
use std::time::Duration;
use once_cell::sync::OnceCell;

const ACCEL_RATE: f32 = 0.01;
const DECEL_RATE: f32 = 0.01;
const SAMPLE_TIME: f32 = 0.01;
static CONTROLLER: OnceCell<Controller> = OnceCell::new();

struct Controller {
    motor_speeds: Arc<Mutex<[f32; 3]>>,
    angles: [f32; 3],
}

impl Controller {
    pub fn new() -> Self {
        Self {
            motor_speeds: Arc::new(Mutex::new([0.0; 3])),
            angles: [(1.0*PI/3.0), PI, (-1.0*PI/3.0)],
        }
    }
}

struct Geometries {
    // The motor is 1.8deg per step. Which means full rotation is 200 steps.
    // Let's say software of rasp PWM is 400Hz which converts to a pulse (step) every 2.5ms.
    // If you multiply that by 200, you get a full rotation in 500ms.
    // NoW if your wheel is 120mm diameter, that means the circonference is 120*PI = 377mm
    // Thus you get 754mm/sec == 0,754m/s == 2,71 km/h.
    // And that's assuming that assuming the torque will follow...
    angles: [f32; 3], // 60, 180, 300
}

// TODO: Implement a strategy to choose between linear, angular or mixed control.
// static STRATEGY: u8 = 0;
// static LINEAR: u8 = 1;
// static ANGULAR: u8 = 2;
// static MIXED: u8 = 3;

static GEOMETRIES: Geometries = Geometries{
    angles: [(1.0*PI/3.0), PI, (-1.0*PI/3.0)]
};

fn compute_direction(motors_speed: &mut [f32; 3], direction: f32, scalar: f32) {
    // Compute the scalar of each motor. It will modify the array given.
    // motors_speed: [f32; 3]. The value of the speed of all motors.
    // direction: f32. The direction of the command referencing the robot.
    // scalar: f32. The speed or force of the command.
    for i in 0..GEOMETRIES.angles.len() {
        match GEOMETRIES.angles.get(i) {
            Some(theta) => {
                motors_speed[i] += scalar * (theta - direction).sin();
            },
            None => println!("Incorrect number of values registered in GEOMETRIES.angles. Expected \'3\', got \'{}\'", i),
        }
    }
}

fn compute_angular(motors_speed: &mut [f32; 3], angle_scalar: f32) {
    motors_speed.fill(angle_scalar);
}

fn compute_adjusted_scalar(motors_speed: &[f32; 3], speed_scalar: f32) -> f32 {
    // let value: f32 = *motors_speed.into_iter().reduce(f32::max).unwrap();
    let remaining_power: f32 = 1.0 - (motors_speed.iter().copied().fold(f32::NAN, f32::max)).abs();
    return remaining_power * speed_scalar;

}

fn check_overload(motors_speed: &mut [f32; 3]) {
    for i in 0..motors_speed.len() {
        match motors_speed.get(i) {
            Some(speed) => {
                if (*speed).abs() > 1.0 {
                    println!("Motor {}, has been assign speed \'{}\' greater than 1.0! Saturating to 1.0.", i, *speed);
                    if speed.is_sign_negative() {
                        motors_speed[i] = -1.0;
                    } else {
                        motors_speed[i] = 1.0;
                    }
                }
            },
            None => println!("Incorrect number of values registered in motors_speed. Expected \'3\', got \'{}\'", i),
        }
    }
}

fn check_rate(motors_speed: &mut [f32; 3]) {
    println!("check rate");
    let mut old_speeds: [f32; 3] = [0.0; 3];
    match CONTROLLER.get() {
        // Some(x) => old_speeds.clone_from(&x.motor_speeds.lock().unwrap()),
        Some(x) => {old_speeds.copy_from_slice(&x.motor_speeds.lock().unwrap().clone()); println!("old_speed: {:?}", old_speeds);},
        None => println!("Please start sending IOs first")
    }
    for (index, speed) in motors_speed.iter_mut().enumerate() {
        let rate: f32 = (*speed - old_speeds[index]) / SAMPLE_TIME;
        let mut rate_max: f32 = 0.0;
        if rate.is_sign_positive() {
            rate_max = ACCEL_RATE;
        } else {
            rate_max = DECEL_RATE
        }
        if rate > rate_max.abs() {
            *speed = SAMPLE_TIME * rate_max + old_speeds[index];
        }
    }
    match CONTROLLER.get() {
        Some(x) => {x.motor_speeds.lock().unwrap().clone_into(&mut motors_speed.clone()); println!("CONTROLLER : {:?}", motors_speed);},
        None => println!("Please start sending IOs first")
    }
    println!("old_speed: {:?}", old_speeds);
}

pub fn convert(direction: f32, speed_scalar: f32, angle_scalar: f32) -> [f32; 3] {
    // Converts basic commands to motor ratios.
    // direction: f32; Direction of command relative to the robot in radians.
    // speed_scalar: f32; Scale between 0 (minimal speed) to 1 (maximale speed).
    // angle_scalar: f32; Scale between 0 (minimal angular speed) to 1 (maximale angular speed).
    // returns an array of the three motor all scalled to one
    let mut motors_speed: [f32; 3] = [0.0; 3];
    CONTROLLER.get_or_init(|| Controller::new());
    compute_angular(&mut motors_speed, angle_scalar);
    let adjusted_speed_scalar = compute_adjusted_scalar(&motors_speed, speed_scalar);
    compute_direction(&mut motors_speed, direction, adjusted_speed_scalar);
    check_rate(&mut motors_speed);
    println!("before: {:?}", motors_speed);
    check_overload(&mut motors_speed);
    println!("after : {:?}", motors_speed);
    return [motors_speed[0], motors_speed[1] ,motors_speed[2]]
}

// TODO: Add a convert for twist method (vector3 linear, vector3 angular)
