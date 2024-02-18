use once_cell::sync::OnceCell;
use std::sync::{Mutex, Arc};
use std::time::{Duration};
use std::time::SystemTime;

const ACCEL_RATE: f32 = 0.01;
const DECEL_RATE: f32 = -0.01;
const SAMPLE_TIME: f32 = 0.01;
static RATE_LIMITER: OnceCell<RateLimiter> = OnceCell::new();

struct RateLimiter {
    motors_speed_previous: Arc<Mutex<[f32; 3]>>,
    time_previous: Arc<Mutex<SystemTime>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            motors_speed_previous: Arc::new(Mutex::new([0.0; 3])),
            time_previous: Arc::new(Mutex::new(SystemTime::now()))
        }
    }
}

pub fn check_rate(motors_speed: &mut [f32; 3]) -> [f32; 3] {
    let now: SystemTime = SystemTime::now();
    let obj = RATE_LIMITER.get_or_init(|| RateLimiter::new());
    let sample_time: Duration = now.duration_since(obj.time_previous.lock().unwrap().clone())
                                                   .expect("Could not compute time difference");
    for (index, speed) in motors_speed.iter_mut().enumerate() {
        let previous_speed: f32 = *obj.motors_speed_previous.lock().unwrap()
                                      .get(index).expect("Could not grab previous value");
        let rate: f32 = (*speed - previous_speed) / sample_time.as_secs_f32();

        let mut rate_max: f32 = 0.0;
        if rate.is_sign_positive() {
            rate_max = ACCEL_RATE;
        } else {
            rate_max = DECEL_RATE;
        }
        if rate > rate_max.abs() {
            *speed = sample_time.as_secs_f32() * rate_max + previous_speed;
        }
    }
    obj.motors_speed_previous.lock().unwrap().clone_from(motors_speed);
    return *motors_speed
}
