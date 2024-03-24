// use gilrs::{Gilrs, Button, Event};
use std::{thread::sleep, time::Duration};


fn main() {

    teleop_gamepad::start_teleoperation();

    for _i in 0..100 {

        println!("Still listening for stuff here {:?} over {:?}", _i, 100);
        // Fetch the data
        let user_input: [f32; 3] = teleop_gamepad::get_user_input();
        println!("Value returned is {:?}", user_input);
        sleep(Duration::from_millis(100));
    }
}


// fn main() {
//     println!("Hello, world!");
//     let mut gilrs = Gilrs::new().unwrap();
// 
//     for (_id, gamepad) in gilrs.gamepads() {
//             println!("{} is {:?}", gamepad.name(), gamepad.power_info());
//     }
// 
//     let mut active_gamepad = None;
// 
//     loop {
//         while let Some(Event { id, event, time }) = gilrs.next_event() {
//             println!("{:?} New event from {}: {:?}", time, id, event);
//             active_gamepad = Some(id);
//         }
// 
//         if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
//             if gamepad.is_pressed(Button::South) {
//                 println!("Button South is pressed (XBox - A, PS - X)");
//             }
//         }
//     }
// }
