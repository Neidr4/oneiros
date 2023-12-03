use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    // TODO: Not sure that this will work. It needs a reference to be assigned
    // which is not what's planned in the other packages
    // let motor_speeds: 'static+[f32; 3] = [1.0, 0.2, 0.8];
    // let motor_speeds: [f32; 3] = [1.0, 0.2, 0.8];
    // println!("About to create!");
    // let mut rasp_controller = control_raspberry_adapter::RaspberryAdapter::new();
    // println!("About to start!");
    // let _ = rasp_controller.start_sending_to_io2();
    // println!("About to update!");
    // rasp_controller.update_speed_value([1.0; 3]);
    // thread::sleep(Duration::from_secs(5));
    // rasp_controller.update_speed_value(motor_speeds);
    // thread::sleep(Duration::from_secs(5));
    // rasp_controller.stop_sending_to_io();
    println!("About to start!");
    let _ = control_raspberry_adapter::start_sending_to_io3();
    println!("About to update!");
    control_raspberry_adapter::update_speed_value([0.2; 3]);
    thread::sleep(Duration::from_secs(5));
    println!("About to update!");
    let motor_speeds: [f32; 3] = [1.0; 3];
    control_raspberry_adapter::update_speed_value(motor_speeds);
    thread::sleep(Duration::from_secs(5));
    control_raspberry_adapter::stop_sending_to_io();
}

// use std::error::Error;
// use std::thread;
// use std::time::Duration;
// 
// use rppal::pwm::{Channel, Polarity, Pwm};
// 
// fn main() -> Result<(), Box<dyn Error>> {
//     // Enable PWM channel 0 (BCM GPIO 18, physical pin 12) at 2 Hz with a 25% duty cycle.
//     let pwm = Pwm::with_frequency(Channel::Pwm0, 2.0, 0.25, Polarity::Normal, true)?;
// 
//     // Sleep for 2 seconds while the LED blinks.
//     thread::sleep(Duration::from_secs(2));
// 
//     // Reconfigure the PWM channel for an 8 Hz frequency, 50% duty cycle.
//     pwm.set_frequency(8.0, 0.5)?;
// 
//     thread::sleep(Duration::from_secs(3));
// 
//     Ok(())
// 
//     // // When the pwm variable goes out of scope, the PWM channel is automatically disabled.
//     // // You can manually disable the channel by calling the Pwm::disable() method. }
//     // use rppal::gpio::Gpio;
// 
//     // let gpio = Gpio::new()?;
//     // let mut pin = gpio.get(16)?.into_output();
// 
//     // pin.set_high();
//     // thread::sleep(Duration::from_secs(1));
//     // pin.set_low();
// 
//     // Ok(())
// }
