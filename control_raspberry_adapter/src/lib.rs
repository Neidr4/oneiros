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

fn convert_to_pwm(motor_speeds: (f32, f32, f32)) {
    println!("Converting to PWM");
}

pub fn send_to_io(motor_pwms: (f32, f32, f32)) {
    if is_setup_io() != true {
        println!("I/O have not been setup\n Please setup I/O before converting.");
        return
    }
    convert_to_pwm(motor_pwms);
    println!("Sending to IO");
}
