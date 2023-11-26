use std::thread;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::error::Error;
use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};
use rppal::pwm::{Channel, Pwm, Polarity};

// TODO: Add a function start sending in a thread
const PWM_FREQ_MIN: u32 = 1;
const GPIO_PWM: u8 = 23;
static EXIT_EVENT: AtomicBool = AtomicBool::new(false);

pub fn setup_io() {
    // Setup the io
    // Setup the motor parameters steps and ticks
}

fn is_setup_io() -> bool {
    let mut result: bool = true;
    if result {
        result = true;
    } else {
        result = false;
    }
    return result
}

fn convert_to_pwm(motor_speeds: &[f32; 3]) -> Result<(), Box<dyn Error>> {
    let pwm_0 = Pwm::with_frequency(Channel::Pwm0, 2.0, 0.25, Polarity::Normal, true)?;
    let pwm_1 = Pwm::with_frequency(Channel::Pwm1, 2.0, 0.25, Polarity::Normal, true)?;
    let mut pwm_2 = Gpio::new()?.get(GPIO_PWM)?.into_output();
    let _ = pwm_2.set_pwm_frequency(2.0, 0.25);
    let mut pwms_previous: [f32; 3] = [0.0; 3];
    loop {
        if EXIT_EVENT.load(Ordering::Relaxed) { break; }
        if *motor_speeds != pwms_previous {
            println!("{:?}", motor_speeds);
            pwm_0.set_frequency((motor_speeds[0] * PWM_FREQ_MIN as f32).into(), 0.5)?;
            pwm_1.set_frequency((motor_speeds[1] * PWM_FREQ_MIN as f32).into(), 0.5)?;
            pwm_2.set_pwm_frequency((motor_speeds[2] * PWM_FREQ_MIN as f32).into(), 0.5)?;
            pwms_previous = motor_speeds.clone();
        }
        thread::sleep(Duration::from_millis(100));
    }
    return Ok(());
}

pub fn start_sending_to_io(motor_pwms: &'static [f32; 3]) -> Result<(), Box<dyn Error>> {
    if is_setup_io() != true {
        println!("I/O have not been setup\n Please setup I/O before converting.");
        return Ok(());
    }
    // TODO: verify that the values are legal
    thread::spawn(move || {
        println!("Starting the PWM thread");
        EXIT_EVENT.store(false, Ordering::Relaxed);
        let _ = convert_to_pwm(&motor_pwms);
        // println!("Leaving the PWM thread");
    });
    return Ok(());
}

pub fn stop_sending_to_io() {
    println!("Stopping sending to IOs");
    EXIT_EVENT.store(true, Ordering::Relaxed);
}
