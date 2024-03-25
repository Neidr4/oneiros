use once_cell::sync::OnceCell;
use std::sync::{Mutex, Arc};
use std::time::{Duration, Instant};

const ACCEL_RATE: f32 = 0.5;
const DECEL_RATE: f32 = -0.5;
static RATE_LIMITER: OnceCell<RateLimiter> = OnceCell::new();

struct RateLimiter {
    motors_speed_previous: Arc<Mutex<[f32; 3]>>,
    time_previous: Arc<Mutex<Instant>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            motors_speed_previous: Arc::new(Mutex::new([0.0; 3])),
            time_previous: Arc::new(Mutex::new(Instant::now()))
        }
    }
}

pub fn check_rate(motors_speed: &mut [f32; 3]) -> [f32; 3] {
    let now: Instant = Instant::now();
    let obj = RATE_LIMITER.get_or_init(|| RateLimiter::new());
    let sample_time: Duration = now.duration_since(obj.time_previous.lock().unwrap().clone());
    // Verifying the time is correct
    if Duration::is_zero(&sample_time) {
        println!("Rate limiter could not compute time properly. Returning 0.0 speed");
        obj.time_previous.lock().unwrap().clone_from(&mut Instant::now());
        obj.motors_speed_previous.lock().unwrap().clone_from(&[0.0; 3]);
        return *motors_speed;
    }
    // Compute the rate and limit it for each motor
    for (index, speed) in motors_speed.iter_mut().enumerate() {
        let previous_speed: f32 = *obj.motors_speed_previous.lock().unwrap()
                                      .get(index).expect("Could not grab previous value");
        let rate: f32 = (*speed - previous_speed) / sample_time.as_secs_f32();
        let rate_max: f32;
        if rate.is_sign_positive() {
            rate_max = ACCEL_RATE;
        } else {
            rate_max = DECEL_RATE;
        }
        if rate.abs() > rate_max.abs() {
            *speed = sample_time.as_secs_f32() * rate_max + previous_speed;
        }
    }
    // Saves the info for the next iteration
    obj.motors_speed_previous.lock().unwrap().clone_from(motors_speed);
    obj.time_previous.lock().unwrap().clone_from(&mut Instant::now());
    *motors_speed
}
