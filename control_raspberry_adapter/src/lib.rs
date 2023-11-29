use std::thread;
use std::sync::atomic::AtomicBool;
use std::error::Error;
use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};
use rppal::pwm::{Channel, Pwm};

const PWM_FREQ_MIN: u32 = 1;
const GPIO_PWM: u8 = 23;
const GPIO_DIR0: u8 = 24;
const GPIO_DIR1: u8 = 25;
const GPIO_DIR2: u8 = 26;
const THREAD_SLEEP: u8 = 100;
static EXIT_EVENT: AtomicBool = AtomicBool::new(false);


pub struct RaspberryAdapter {
    exit_event: bool,
    pwm_min_freq: u32,
    speed_desired: [f32; 3],
}


impl RaspberryAdapter {

    pub fn new() -> Self {
        Self {
            exit_event: true,
            pwm_min_freq: 1,
            speed_desired: [0.0; 3],
        }
    }

//
//https://users.rust-lang.org/t/how-to-use-self-while-spawning-a-thread-from-method/8282/4
//
    pub fn start_sending_to_io(&'static self) {
        thread::spawn(move || {
            println!("Starting the PWM thread");
            let _ = self.run_pwm();
        });
        thread::spawn(move || {
            println!("Starting the DIR thread");
            let _ = self.run_dir();
        });
    }

    // Consider getting this method outside and call to only one object
    pub fn update_speed_value(&mut self, motor_pwms: [f32; 3]) {
        // TODO: Make sure the threads are started before using this method
        // TODO: Verify that the values are legal
        self.speed_desired = motor_pwms;
    }

    fn run_pwm(&self) -> Result<(), Box<dyn Error>>  {
        let pwm_0: Pwm = Pwm::new(Channel::Pwm0)?;
        let pwm_1: Pwm = Pwm::new(Channel::Pwm1)?;
        let mut pwm_2: OutputPin = Gpio::new()?.get(GPIO_PWM)?.into_output();
        let mut speed_previous: [f32; 3] = [0.0; 3];
        loop{
            // Checking if anything has changed
            if self.exit_event == false {break;}
            thread::sleep(Duration::from_millis(THREAD_SLEEP.into()));
            if self.speed_desired == speed_previous { continue; }
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

    fn run_dir(&self) -> Result<(), Box<dyn Error>>  {
        let dir_0: OutputPin = Gpio::new()?.get(GPIO_DIR0)?.into_output();
        let dir_1: OutputPin = Gpio::new()?.get(GPIO_DIR1)?.into_output();
        let dir_2: OutputPin = Gpio::new()?.get(GPIO_DIR2)?.into_output();
        let mut list_dir: [OutputPin; 3] = [dir_0, dir_1, dir_2];
        let mut dir_current: [bool; 3] = [true; 3];
        loop {
            if self.exit_event == false {break;}
            for (index, &motor_speed) in self.speed_desired.iter().enumerate() {
                // Checking if anything has changed
                let state = motor_speed.is_sign_positive();
                if state == dir_current[index] {continue};
                dir_current[index] = !dir_current[index];
                // Setting the direction pin
                if state {
                    list_dir[index].set_high();
                } else {
                    list_dir[index].set_low();
                }
            }
            thread::sleep(Duration::from_millis(THREAD_SLEEP.into()));
        }
        println!("Quitting the DIR thread");
        Ok(())
    }

    pub fn stop_sending_to_io(&mut self) {
        println!("Stopping sending to IOs");
        // self.exit_event.store(true, Ordering::Relaxed);
        self.exit_event = true;
    }
}

