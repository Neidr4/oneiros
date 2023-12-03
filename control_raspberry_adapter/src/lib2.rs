use std::thread;
use std::sync::{Mutex, Arc};
use std::sync::atomic::AtomicBool;
use std::error::Error;
use std::time::Duration;
use std::sync::atomic::Ordering;

use rppal::gpio::{Gpio, OutputPin};
use rppal::pwm::{Channel, Pwm};

const PWM_FREQ_MIN: u32 = 1;
const GPIO_PWM: u8 = 23;
const GPIO_DIR0: u8 = 24;
const GPIO_DIR1: u8 = 25;
const GPIO_DIR2: u8 = 26;
const THREAD_SLEEP: u8 = 100;
static EXIT_EVENT: AtomicBool = AtomicBool::new(false);
static MOTOR_SPEEDS: &'static [f32; 3] = &[0.0; 3];

//
//https://users.rust-lang.org/t/how-to-use-self-while-spawning-a-thread-from-method/8282/4
//
pub fn start_sending_to_io3() -> Result<(), Box<dyn Error>> {
    thread::spawn(move || {
        println!("Starting the PWM thread");
        let _ = run_pwm();
    });
    // thread::spawn(move || {
    //     println!("Starting the DIR thread");
    //     let _ = self.run_dir();
    // });
    Ok(())
}

// Consider getting this method outside and call to only one object
pub fn update_speed_value(motor_pwms: [f32; 3]) {
    // TODO: Make sure the threads are started before using this method
    // TODO: Verify that the values are legal
    MOTOR_SPEEDS = motor_pwm;

}

    // fn run_dir(&self) -> Result<(), Box<dyn Error>>  {
    //     let dir_0: OutputPin = Gpio::new()?.get(GPIO_DIR0)?.into_output();
    //     let dir_1: OutputPin = Gpio::new()?.get(GPIO_DIR1)?.into_output();
    //     let dir_2: OutputPin = Gpio::new()?.get(GPIO_DIR2)?.into_output();
    //     let mut list_dir: [OutputPin; 3] = [dir_0, dir_1, dir_2];
    //     let mut dir_current: [bool; 3] = [true; 3];
    //     loop {
    //         if self.exit_event == false {break;}
    //         for (index, &motor_speed) in self.speed_desired.iter().enumerate() {
    //             // Checking if anything has changed
    //             let state = motor_speed.is_sign_positive();
    //             if state == dir_current[index] {continue};
    //             dir_current[index] = !dir_current[index];
    //             // Setting the direction pin
    //             if state {
    //                 list_dir[index].set_high();
    //             } else {
    //                 list_dir[index].set_low();
    //             }
    //         }
    //         thread::sleep(Duration::from_millis(THREAD_SLEEP.into()));
    //     }
    //     println!("Quitting the DIR thread");
    //     Ok(())
    // }

pub fn stop_sending_to_io(&mut self) {
    println!("Stopping sending to IOs");
    self.exit_event.store(true, Ordering::Relaxed);
}

fn run_pwm() -> Result<(), Box<dyn Error>>  {
    let pwm_0: Pwm = Pwm::new(Channel::Pwm0)?;
    let pwm_1: Pwm = Pwm::new(Channel::Pwm1)?;
    let mut pwm_2: OutputPin = Gpio::new()?.get(GPIO_PWM)?.into_output();
    let mut speed_previous: [f32; 3] = [0.0; 3];
    let _ = pwm_0.enable();
    let _ = pwm_1.enable();
    loop{
        // Checking if anything has changed
        if EXIT_EVENT == true {break;}
        thread::sleep(Duration::from_millis(THREAD_SLEEP.into()));
        if MOTOR_SPEEDS == speed_previous { continue; }
        speed_previous = self.speed_desired.clone();
        // Setting the frequency
        pwm_0.set_frequency((self.speed_desired[0] * PWM_FREQ_MIN as f32).into(), 0.5)?;
        pwm_1.set_frequency((self.speed_desired[1] * PWM_FREQ_MIN as f32).into(), 0.5)?;
        pwm_2.set_pwm_frequency((self.speed_desired[2] * PWM_FREQ_MIN as f32).into(), 0.5)?;
    }
    println!("Disabling the PWMs");
    let _ = pwm_0.disable();
    let _ = pwm_1.disable();
    let _ = pwm_2.clear_pwm();
    Ok(())
}
