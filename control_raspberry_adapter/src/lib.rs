use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};
use rppal::pwm::{Channel, Pwm};

// TODO: Add a function start sending in a thread
// static PWM0 = Pwm::new(Channel::Pwm0)?;
// static PWM1 = Pwm::new(Channel::Pwm1)?;
const PWM_FREQ_MIN: u32 = 400;

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

fn convert_to_pwm(motor_speeds: &(f32, f32, f32)) {
    println!("Converting to PWM");
    println!("{:?}", motor_speeds);
    loop {
        println!("{:?}", motor_speeds);
    }
}

pub fn start_sending_to_io(motor_pwms: &(f32, f32, f32)) {

    // let mut led = Gpio::new()?.get(23)?.into_output();
    if is_setup_io() != true {
        println!("I/O have not been setup\n Please setup I/O before converting.");
        return
    }
    let handle = thread::spawn(move || {
        println!("Sending to IO");
        convert_to_pwm(motor_pwms);
        thread::sleep(Duration::from_millis(1));
    });
    handle.join().unwrap();
}
