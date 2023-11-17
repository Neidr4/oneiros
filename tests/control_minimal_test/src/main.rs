use std::f32::consts::PI;

fn main() {
    println!("Hello, world!");

    // let variable_0: usize = controller_omniwheel_3::add(199, 1);
    // println!("{}", variable_0);

    // let variable_1: (f32, f32, f32) = controller_omniwheel_3::convert(PI/2.0, 1.0, 0.0);
    // println!("variable_1: {:?}", variable_1);

    let variable_2: [f32; 3] = control_minimal::convert(PI/2.0, 0.9, 0.0);
    println!("variable_2: {:?}", variable_2);
}
