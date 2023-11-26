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

static RASP_ADAPT: RaspberryAdapter = RaspberryAdapter();

struct RaspberryAdapter {
    exit_event: bool,
    pwm_0: Pwm,
    pwm_1: Pwm,
    pwm_2: OutputPin,
    pwm_min_freq: u32,
    gpio_pwm: u8,
    speed_desired: [f32; 3],
    speed_current: [f32; 3],
}

impl RaspberryAdapter {
    fn new(&self) -> Self {
        Self {
            setup(),
            speed_current = [0.0; 3],
        }
    }

    pub fn start_sending_to_io(&self, motor_pwms: &'static [f32; 3]) -> Result<(), Box<dyn Error>> {
        // TODO: verify that the values are legal
        thread::spawn(move || {
            println!("Starting the PWM thread");
            EXIT_EVENT.store(false, Ordering::Relaxed);
            let _ = self.set_pwm();
            // println!("Leaving the PWM thread");
        });
        return Ok(());
    }

    fn setup(&self) -> Result<(), Box<dyn Error>> {
        self.pwm_0 = Pwm::with_frequency(Channel::Pwm0, 2.0, 0.25, Polarity::Normal, true)?;
        self.pwm_1 = Pwm::with_frequency(Channel::Pwm1, 2.0, 0.25, Polarity::Normal, true)?;
        self.pwm_2 = Gpio::new()?.get(GPIO_PWM)?.into_output();
        let _ = self.pwm_2.set_pwm_frequency(2.0, 0.25);
        self.speed_current = [0.0; 3];
        Ok(())
    }

    fn set_pwm(&self) -> Result<(), Box<dyn Error>>  {
        loop {
            self.pwm_0.set_frequency((self.speed_desired[0] * PWM_FREQ_MIN as f32).into(), 0.5)?;
            self.pwm_1.set_frequency((self.speed_desired[1] * PWM_FREQ_MIN as f32).into(), 0.5)?;
            self.pwm_2.set_pwm_frequency((self.speed_desired[2] * PWM_FREQ_MIN as f32).into(), 0.5)?;
            if self.exit_event == false {
                break;
            }
        }
        self.pwm_0.disable();
        self.pwm_1.disable();
        self.pwm_2.clear_pwm();
        Ok(())
    }

    fn run_pwm(&self) {
        loop {
            if EXIT_EVENT.load(Ordering::Relaxed) { break; }
            if *motor_speeds != pwms_previous {
                println!("{:?}", motor_speeds);
                self.pwm_0.set_frequency((motor_speeds[0] * PWM_FREQ_MIN as f32).into(), 0.5)?;
                self.pwm_1.set_frequency((motor_speeds[1] * PWM_FREQ_MIN as f32).into(), 0.5)?;
                self.pwm_2.set_pwm_frequency((motor_speeds[2] * PWM_FREQ_MIN as f32).into(), 0.5)?;
                self.pwms_previous = motor_speeds.clone();
            }
            thread::sleep(Duration::from_millis(100));
        }
    }

    pub fn stop_sending_to_io(&self) {
        println!("Stopping sending to IOs");
        // self.exit_event.store(true, Ordering::Relaxed);
        self.exit_event = true;
    }
}

pub fn update_speed(speeds: [f32; 3]) {

}
fn convert_to_pwm(motor_speeds: &[f32; 3]) -> Result<(), Box<dyn Error>> {
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


