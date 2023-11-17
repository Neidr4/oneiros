fn main() {
    println!("Hello, world!");

    // TODO: Not sure that this will work. It needs a reference to be assigned
    // which is not what's planned in the other packages
    let motor_speeds: &'static (f32, f32, f32) = &(1.0, 0.2, 0.8);
    control_raspberry_adapter::start_sending_to_io(&motor_speeds);
}
