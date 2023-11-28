use std::thread;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::error::Error;
use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};
use rppal::pwm::{Channel, Pwm, Polarity};

const PWM_FREQ_MIN: u32 = 1;
const GPIO_PWM: u8 = 23;
static EXIT_EVENT: AtomicBool = AtomicBool::new(false);

// static RASP_ADAPT: RaspberryAdapter = RaspberryAdapter();
static RASP_ADAPT: RaspberryAdapter = RaspberryAdapter {
    exit_event: true,
    pwm_0: Pwm::new(Channel::Pwm0),
    pwm_1: Pwm::new(Channel::Pwm1),
    pwm_2: Gpio::new()?.get(GPIO_PWM)?.into_output(),
    pwm_min_freq: PWM_FREQ_MIN,
    speed_desired: [0.0; 3],
    speed_current: [0.0; 3],

};

struct RaspberryAdapter {
    exit_event: bool,
    pwm_min_freq: u32,
    dir_pins: [u8; 3],
    gpio_pwm: u8,
    dir_desired: [f32; 3],
    dir_current: [f32; 3],
    speed_desired: [f32; 3],
    speed_current: [f32; 3],
}


impl RaspberryAdapter {

    // TODO: change this to setup
    pub fn start_sending_to_io(&self, motor_pwms: &'static [f32; 3]) -> Result<(), Box<dyn Error>> {
        thread::spawn(move || {
            println!("Starting the PWM thread");
            EXIT_EVENT.store(false, Ordering::Relaxed);
            let _ = self.run_();
            // println!("Leaving the PWM thread");
        });
        return Ok(());
    }

    pub fn update_speed_value(&self, motor_pwms: [f32; 3]) {
        // TODO: Verify that the values are legal
        // TODO: extract the minus for dir
        self.speed_desired = self.compute_dir();
    }

    fn run_pwm(&self) -> Result<(), Box<dyn Error>>  {
        // TODO: instanciate the pwms
        // let pwm_0 = Pwm::with_frequency((self.speed_desired[0] * PWM_FREQ_MIN as f32).into(), 0.5)?;
        // let pwm_1 = Pwm::with_frequency((self.speed_desired[1] * PWM_FREQ_MIN as f32).into(), 0.5)?;
        // let mut pwm_2 = Gpio::.into(), 0.5)?;
        loop{
            // TODO: update the PWM depending on pwm desired
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

    pub fn stop_sending_to_io(&self) {
        println!("Stopping sending to IOs");
        // self.exit_event.store(true, Ordering::Relaxed);
        self.exit_event = true;
    }
}

