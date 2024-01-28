use std::thread;
use std::sync::{Mutex, Arc};
use std::sync::atomic::AtomicBool;
use std::error::Error;
use std::time::Duration;
use std::sync::atomic::Ordering;
use once_cell::sync::OnceCell;

use rppal::gpio::{Gpio, OutputPin};
use rppal::pwm::{Channel, Pwm};

const PWM_FREQ_MIN: u32 = 41;
const GPIO_PWM: u8 = 6;
const GPIO_DIR0: u8 = 17;
const GPIO_DIR1: u8 = 27;
const GPIO_DIR2: u8 = 22;
const THREAD_SLEEP: u8 = 10;
static EXIT_EVENT: AtomicBool = AtomicBool::new(false);

static RASPBERRY_ADAPTER: OnceCell<RaspberryAdapter> = OnceCell::new();

struct RaspberryAdapter {
    motor_speeds: Arc<Mutex<[f32; 3]>>,
}

impl RaspberryAdapter {
    pub fn new() -> Self {
        Self {
            motor_speeds: Arc::new(Mutex::new([0.0; 3])),
        }
    }
}

pub fn start_sending_to_io() -> Result<(), Box<dyn Error>> {
    RASPBERRY_ADAPTER.get_or_init(|| RaspberryAdapter::new());
    thread::spawn(move || {
        println!("Starting the PWM thread");
        let _ = run_pwm();
        println!("Stopping the PWM thread");
    });
    thread::spawn(move || {
        println!("Starting the DIR thread");
        let _ = run_dir();
        println!("Stopping the DIR thread");
    });
    Ok(())
}

// Consider getting this method outside and call to only one object
pub fn update_speed_value(mut motor_pwms: [f32; 3]) {
    // TODO: Make sure the threads are started before using this method
    for pwm in motor_pwms.iter_mut() {
        if (*pwm).abs() > 1.0 {
            println!("Saturating to 1.0.");
            if pwm.is_sign_negative() {
                *pwm = -1.0;
            } else {
                *pwm = 1.0;
            }
        }
        if pwm.abs() < 0.05 {
            *pwm = 0.0;
        }
    }
    match RASPBERRY_ADAPTER.get() {
        Some(x) => x.motor_speeds.lock().unwrap().clone_from(&motor_pwms),
        None => println!("Please start sending IOs first"),
    }
}

pub fn stop_sending_to_io() {
    // TODO: -Add Catch interrupt to call stop
    // Go to rppal gpio_blinked_signals example to handle signal interrupt
    println!("Stopping sending to IOs");
    EXIT_EVENT.store(true, Ordering::Relaxed);
    // TODO: Find something better for this. Wait the EXIT to be back to true?
    thread::sleep(Duration::from_secs(1));
}

fn run_dir() -> Result<(), Box<dyn Error>>  {
    let dir_0: OutputPin = Gpio::new()?.get(GPIO_DIR0)?.into_output();
    let dir_1: OutputPin = Gpio::new()?.get(GPIO_DIR1)?.into_output();
    let dir_2: OutputPin = Gpio::new()?.get(GPIO_DIR2)?.into_output();
    let mut list_dir: [OutputPin; 3] = [dir_0, dir_1, dir_2];
    let mut dir_current: [bool; 3] = [true; 3];
    let mut motor_speeds: [f32; 3] = [0.0; 3];
    loop {
        if EXIT_EVENT.load(Ordering::Relaxed) == true {break;}
        match RASPBERRY_ADAPTER.get() {
            Some(x) => x.motor_speeds.lock().unwrap().clone_into(&mut motor_speeds),
            None => {println!("Please start sending IOs first"); continue;}
        }
        for (index, &motor_speed) in motor_speeds.iter().enumerate() {
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
    Ok(())
}

fn apply_pwm(value: f32, pwm_channel: &Pwm) -> Result<(), Box<dyn Error>> {
        // Setting the frequency
        if value == 0.0 {
            _ = pwm_channel.disable();
        } else if !pwm_channel.is_enabled()? {
            _ = pwm_channel.enable();
        } else {
            pwm_channel.set_frequency((value * PWM_FREQ_MIN as f32).abs().into(), 0.5)?;
        }
        return Ok(())
}

fn run_pwm() -> Result<(), Box<dyn Error>>  {
    // TODO: Make try again with with_frequency method to check pinout
    let pwm_0: Pwm = Pwm::new(Channel::Pwm0)?;
    let pwm_1: Pwm = Pwm::new(Channel::Pwm1)?;
    let mut pwm_2: OutputPin = Gpio::new()?.get(GPIO_PWM)?.into_output();
    let mut speed_previous: [f32; 3] = [0.0; 3];
    let mut motor_speeds: [f32; 3] = [0.0; 3];
    loop{
        // Checking if anything has changed
        if EXIT_EVENT.load(Ordering::Relaxed) == true {break;}
        thread::sleep(Duration::from_millis(THREAD_SLEEP.into()));
        match RASPBERRY_ADAPTER.get() {
            Some(x) => x.motor_speeds.lock().unwrap().clone_into(&mut motor_speeds),
            None => {println!("Please start sending IOs first"); continue;}
        }
        if motor_speeds == speed_previous { continue; }
        speed_previous = motor_speeds.clone();
        _ = apply_pwm(motor_speeds[0], &pwm_0);
        _ = apply_pwm(motor_speeds[1], &pwm_1);
        pwm_2.set_pwm_frequency((motor_speeds[2] * PWM_FREQ_MIN as f32).abs().into(), 0.5)?;
        // println!("motor_speeds pwm: {:?}", motor_speeds);
    }
    println!("Disabling the PWMs");
    let _ = pwm_0.disable();
    let _ = pwm_1.disable();
    let _ = pwm_2.clear_pwm();
    Ok(())
}
