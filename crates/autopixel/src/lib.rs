// 1. Pixelate the image
// 2. Reduce the number of colors with median-cut algorithm
// 3. Find rectangles of pixels that are the same color quad-tree decomposition
// 4. Quad-tree nodes are all squares, we optimize by combining adjacent squares into rectangles
// 5. Write p5.js code to draw the rectangles

mod error;
mod median_cut;
mod p5;
mod quadtree;
mod rectangle;

pub use error::{Error, Result};

use std::{
    collections::HashMap,
    io::{BufRead, Cursor, Read, Seek},
};

use image::{
    imageops::{resize, FilterType},
    ImageReader, Rgb, RgbImage,
};
use median_cut::median_cut;

use p5::create_program;
use quadtree::{decompose, optimize};
use rectangle::Rectangle;
use tracing::info_span;

pub type Color = Rgb<u8>;

pub fn encode_png(image: &RgbImage) -> Vec<u8> {
    let mut buffer = Cursor::new(Vec::new());
    image
        .write_to(&mut buffer, image::ImageFormat::Png)
        .expect("Failed to encode PNG");
    buffer.into_inner()
}

pub fn autopixel<R: Read + Seek + BufRead>(
    image_buffer: R,
    hash: u64,
    scale: usize,
    colors: usize,
) -> Result<(String, RgbImage)> {
    let image: RgbImage = ImageReader::new(image_buffer)
        .with_guessed_format()?
        .decode()?
        .to_rgb8();

    let _entered = info_span!("autopixel", hash = format!("{:#x}", hash), scale, colors).entered();

    let mut pixelated = pixelate(&image, scale);
    let color_counts = quantize(&mut pixelated, colors);

    let mut color_order = color_counts.into_iter().collect::<Vec<_>>();
    color_order.sort_unstable_by_key(|(_color, count)| *count);
    let color_order: Vec<Color> = color_order
        .into_iter()
        .map(|(color, _count)| color)
        .rev()
        .collect();

    let mut instructions = decompose(&pixelated, &color_order);
    instructions.iter_mut().for_each(|(_color, v)| optimize(v));

    let program = create_program(
        &color_order,
        &instructions,
        scale,
        pixelated.width() as usize,
        pixelated.height() as usize,
    );

    Ok((program, pixelated))
}

// Turns an RgbImage (Vec<u8>) into a Vec<Color>
pub(crate) fn into_colors(image: &RgbImage) -> Vec<Color> {
    image
        .as_raw()
        .chunks(3)
        .map(|chunk| Rgb([chunk[0], chunk[1], chunk[2]]))
        .collect()
}

fn pixelate(image: &RgbImage, pixel_size: usize) -> RgbImage {
    resize(
        image,
        image.width() / pixel_size as u32,
        image.height() / pixel_size as u32,
        FilterType::Nearest,
    )
}

fn quantize(image: &mut RgbImage, num_colors: usize) -> HashMap<Color, usize> {
    let colors: Vec<Color> = into_colors(image);
    let palette = median_cut(colors, num_colors);

    // Count the occurrences of each color in the image
    let mut color_counts = HashMap::new();

    image.pixels_mut().for_each(|pixel| {
        let quantized = palette.quantize(pixel);
        color_counts
            .entry(quantized)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        *pixel = quantized;
    });

    color_counts
}
