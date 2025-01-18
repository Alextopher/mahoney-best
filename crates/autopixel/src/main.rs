use std::{fs, io};

use autopixel::autopixel;

fn main() {
    let reader = io::Cursor::new(include_bytes!("../examples/monalisa.jpg").to_vec());
    let scale = 4;
    let colors = 16;

    let (program, image) = autopixel(reader, scale, colors).expect("Failed to autopixel");

    image.save("output.png").expect("Failed to save image");
    fs::write("output.js", program).expect("Failed to save program");
}
