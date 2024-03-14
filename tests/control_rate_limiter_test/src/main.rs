use std::time::{Duration, Instant};
use std::thread::sleep;

const TEST_TIME: Duration = Duration::new(7, 0);
const REFRESH_PERIOD: Duration = Duration::from_millis(10);

fn main() {
    println!("Hello, world!");

    let mut i : f32 = 0.0;
    let mut variable: [f32; 3] = [0.0; 3];
    let starting_time : Instant = Instant::now();
    loop {
        if starting_time.elapsed() > TEST_TIME {
            break;
        }

        println!("------ i : {:?} --------", i);
        i += 0.01;
        variable.fill(-0.5);
        // if i < 0.5 {
        //     variable.fill(0.5);
        // } else if i > 0.5 && i < 1.0 {
        //     variable.fill(0.5 - i);
        // }
        println!("variable asked   is {:?}", variable);
        control_rate_limiter::check_rate(&mut variable);
        println!("variable after   is {:?}", variable);
        sleep(REFRESH_PERIOD);
    }
}
