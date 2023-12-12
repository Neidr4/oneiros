use std::{io, thread::sleep, time::Duration};
use inputbot::KeybdKey::*;
use inputbot::MouseButton::*;

pub fn usage() {
    println!(" ------------------------- ");
    println!("Directions are \n
             Q W E\n
             A   D\n
             Z X C\n"
             );
println!("Rotations are \n
             J K"
             );
    println!(" ------------------------- ");
}

fn callback(key: String) {
    println!("{} key has been pressed", key);
}

// TODO: Add a capture of the inputs
pub fn teleoperate() {
    usage();
    // TODO: Add loop
    // TODO: Add timeout
    // TODO: Think of the way to take input of the constantly maintain key
    // let mut input = String::new();
    //
    // match io::stdin().read_line(&mut input) {
    //     Ok(n) => {
    //         // println!("{input}");
    //         match input.trim() {
    //             "z" => println!("z"),
    //             _ => println!("invalid"),
    //         }
    //     }
    //     Err(error) => println!("error: {error}"),
    // }
    //

    let mut command: [f32; 3] = [0.0; 3];
    let mut command2 = vec![1.0, 2.0, 3.0];
    command[0] = 1.0;

    WKey.bind(move || {
        println!("Z key has been pressed");
        sleep(Duration::from_millis(100));
    });

    QKey.bind(move || {
        callback("Q".to_string());
        sleep(Duration::from_millis(100));
    });

    AKey.bind(move || {
        callback("A".to_string());
        sleep(Duration::from_millis(100));
    });

    SKey.bind(move || {
        println!("A key has been pressed");
        sleep(Duration::from_millis(100));
    });

    DKey.bind(move || {
        println!("D key has been pressed");
        sleep(Duration::from_millis(100));
    });

    inputbot::handle_input_events();
}
