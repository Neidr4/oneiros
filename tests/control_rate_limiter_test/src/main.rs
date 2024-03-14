use std::{thread::sleep, time::Duration};

fn main() {
    println!("Hello, world!");

    let mut variable: [f32; 3] = [0.0; 3];
    for i in 0..100 {
        println!("------ i : {:?} --------", i);
        let y: f32 = i as f32 * 0.01;
        if i < 50 {
            variable.fill(0.5);
        } else if i > 50 && i < 100 {
            variable.fill(0.5 - y);
        }
        println!("variable asked   is {:?}", variable);
        control_rate_limiter::check_rate(&mut variable);
        println!("variable after   is {:?}", variable);
        sleep(Duration::from_millis(10));
    }
}
