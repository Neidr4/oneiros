use std::f32::consts::PI;

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
                    motors_speed[i] = 1.0;
                }
            },
            None => println!("Incorrect number of values registered in motors_speed. Expected \'3\', got \'{}\'", i),
        }
    }
}

pub fn convert(direction: f32, speed_scalar: f32, angle_scalar: f32) -> (f32, f32, f32) {
    // Converts basic commands to motor ratios.
    // direction: f32; Direction of command relative to the robot in radians.
    // speed_scalar: f32; Scale between 0 (minimal speed) to 1 (maximale speed).
    // angle_scalar: f32; Scale between 0 (minimal angular speed) to 1 (maximale angular speed).
    // returns an array of the three motor all scalled to one
    let mut motors_speed: [f32; 3] = [0.0; 3];
    compute_angular(&mut motors_speed, angle_scalar);
    let adjusted_speed_scalar = compute_adjusted_scalar(&motors_speed, speed_scalar);
    compute_direction(&mut motors_speed, direction, adjusted_speed_scalar);
    check_overload(&mut motors_speed);
    (motors_speed[0], motors_speed[1], motors_speed[2])
}

// TODO: Add a convert for twist method (vector3 linear, vector3 angular)
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
