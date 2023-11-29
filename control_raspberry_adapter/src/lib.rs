use std::thread;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::error::Error;
use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};
use rppal::pwm::{Channel, Pwm, Polarity};

const PWM_FREQ_MIN: u32 = 1;
const GPIO_PWM: u8 = 23;
const GPIO_DIR0: u8 = 24;
const GPIO_DIR1: u8 = 25;
const GPIO_DIR2: u8 = 26;
static EXIT_EVENT: AtomicBool = AtomicBool::new(false);

// static RASP_ADAPT: RaspberryAdapter = RaspberryAdapter();
static RASP_ADAPT: RaspberryAdapter = RaspberryAdapter {
    exit_event: EXIT_EVENT,
    pwm_min_freq: PWM_FREQ_MIN,
    speed_desired: [0.0; 3],
    speed_current: [0.0; 3],

};

struct RaspberryAdapter {
    exit_event: bool,
    pwm_min_freq: u32,
    dir_pins: [u8; 3],
    gpio_pwm: u8,
    speed_desired: [f32; 3],
}


impl RaspberryAdapter {

    pub fn start_sending_to_io(&self, motor_pwms: &'static [f32; 3]) -> Result<(), Box<dyn Error>> {
        thread::spawn(move || {
            println!("Starting the PWM thread");
            EXIT_EVENT.store(false, Ordering::Relaxed);
            let _ = self.run_pwm();
            // println!("Leaving the PWM thread");
        });
        // TODO: Do the same for dir
        return Ok(());
    }

    // Consider getting this method outside and call to only one object
    pub fn update_speed_value(&self, motor_pwms: [f32; 3]) {
        // TODO: Make sure the threads are started before using this method
        // TODO: Verify that the values are legal
        // TODO: extract the minus for dir
        self.speed_desired = motor_pwms;
    }

    fn run_pwm(&self) -> Result<(), Box<dyn Error>>  {
        let pwm_0: Pwm = Pwm::new(Channel::Pwm0)?;
        let pwm_1: Pwm = Pwm::new(Channel::Pwm1)?;
        let mut pwm_2: OutputPin = Gpio::new()?.get(GPIO_PWM)?.into_output();
        
        loop{
            // TODO: add a check to see if the values have changed (this can be local)
            pwm_0.set_frequency((self.speed_desired[0] * PWM_FREQ_MIN as f32).into(), 0.5)?;
            pwm_1.set_frequency((self.speed_desired[1] * PWM_FREQ_MIN as f32).into(), 0.5)?;
            pwm_2.set_pwm_frequency((self.speed_desired[2] * PWM_FREQ_MIN as f32).into(), 0.5)?;
            if self.exit_event == false {break;}
            thread::sleep(Duration::from_millis(100));
        }
        pwm_0.disable();
        pwm_1.disable();
        pwm_2.clear_pwm();
        Ok(())
    }

    fn run_dir(&self) -> Result<(), Box<dyn Error>>  {
        let mut dir_0: OutputPin = Gpio::new()?.get(GPIO_DIR0)?.into_output();
        let mut dir_1: OutputPin = Gpio::new()?.get(GPIO_DIR1)?.into_output();
        let mut dir_2: OutputPin = Gpio::new()?.get(GPIO_DIR2)?.into_output();
        let mut dir_current: [bool; 3] = [true; 3];
        loop {
            // TODO: add a check to see if the values have changed (this can be local)
            let mut dir_desired = self.speed_desired.iter().map(|x| x.is_sign_positive()).collect();
            if dir_current == dir_desired {
                // TODO: iterate and assing to dir pin
                dir_0.set_low();
            }
        }

    }

    pub fn stop_sending_to_io(&self) {
        println!("Stopping sending to IOs");
        // self.exit_event.store(true, Ordering::Relaxed);
        self.exit_event = true;
    }
}

