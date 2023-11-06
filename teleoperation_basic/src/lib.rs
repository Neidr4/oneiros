use std::io;

pub fn usage() {
    println!(" ------------------------- ");
    println!("Directions are \n
             A Z E\n
             Q   D\n
             W X C\n"
             );
println!("Rotations are \n
             J K"
             );
    println!(" ------------------------- ");
}

pub fn teleoperate() {
    usage();
    // TODO: Add loop
    // TODO: Add timeout
    // TODO: Think of the way to take input of the constantly maintain key
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            // println!("{input}");
            match input.trim() {
                "z" => println!("z"),
                _ => println!("invalid"),
            }
        }
        Err(error) => println!("error: {error}"),
    }
}
