use std::{fs, io};

use autopixel::autopixel;

#[tracing::instrument]
fn main() {
    let reader = io::Cursor::new(include_bytes!("../examples/monalisa.jpg").to_vec());
    let scale = 4;
    let colors = 16;

    let my_subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .with_max_level(tracing::Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(my_subscriber)
        .expect("setting default subscriber failed");

    let (program, image) = autopixel(reader, 0, scale, colors).expect("Failed to autopixel");

    image.save("output.png").expect("Failed to save image");
    fs::write("output.js", program).expect("Failed to save program");
}
