//! Quadtree decomposition of a quantized image

use std::collections::HashMap;

use image::RgbImage;

use crate::{into_colors, Color, Rectangle};

pub struct Decomposition {
    dims: Rectangle,
    pixels: Vec<Color>,
    bitmap: Vec<bool>,
}

impl Decomposition {
    fn new(image: &RgbImage) -> Self {
        let dims = Rectangle::new(0, 0, image.width() as usize, image.height() as usize);
        let pixels = into_colors(image);
        let bitmap = vec![false; pixels.len()];

        Self {
            dims,
            pixels,
            bitmap,
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.dims.width + x
    }

    fn update_bitmap(&mut self, rect: Rectangle, color: &Color) {
        debug_assert_eq!(rect, rect.crop(&self.dims).unwrap());

        for y in rect.y..rect.y + rect.height {
            let start = self.index(rect.x, y);
            let end = self.index(rect.x + rect.width, y);

            (start..end)
                .filter(|&idx| self.pixels[idx] == *color)
                .for_each(|idx| self.bitmap[idx] = true);
        }
    }

    // Checks
    // - All pixels in the region are not finalized
    // - There's at least 1 pixel in the region with the instruction's color
    fn check_instruction(&self, rect: &Rectangle, color: &Color) -> bool {
        let rect = match rect.crop(&self.dims) {
            Some(rect) => rect,
            None => return false,
        };

        let mut contains_color = false;
        for y in rect.y..rect.y + rect.height {
            let start = self.index(rect.x, y);
            let end = self.index(rect.x + rect.width, y);

            // If any pixel in the region is finalized - return false as we'd otherwise be clobbering
            // a valid pixel
            if self.bitmap[start..end].iter().any(|p| *p) {
                return false;
            }

            // Check that there's at least 1 pixel of
            if self.pixels[start..end].iter().any(|c| *c == *color) {
                contains_color = true;
            }
        }

        contains_color
    }

    fn crop(&self, rect: &Rectangle) -> Option<Rectangle> {
        rect.crop(&self.dims)
    }
}

pub fn decompose(image: &RgbImage, color_order: &[Color]) -> HashMap<Color, Vec<Rectangle>> {
    let mut instructions = HashMap::new();

    // Start by applying the most common color
    let square_size = image.width().max(image.height()).next_power_of_two() as usize;
    let square = Rectangle::from_square(0, 0, square_size);

    let mut decomposition = Decomposition::new(image);
    let first_color = color_order[0];
    decomposition.update_bitmap(decomposition.dims, &first_color);
    instructions.insert(first_color, vec![decomposition.dims]);

    // Now for the rest of the colors
    for color in color_order.iter().skip(1) {
        let mut instrs = vec![];
        let mut work = vec![square];

        while let Some(rect) = work.pop() {
            if decomposition.check_instruction(&rect, color) {
                let cropped = decomposition.crop(&rect).unwrap();
                instrs.push(cropped);
                decomposition.update_bitmap(cropped, color);
            } else if let Some(children) = rect.split() {
                work.extend(children)
            }
        }

        instructions.insert(*color, instrs);
    }

    instructions
}

/// Optimizes a program by merging adjacent rectangles
pub fn optimize(instructions: &mut Vec<Rectangle>) {
    if instructions.len() == 1 {
        return;
    }

    let mut next = Vec::with_capacity(instructions.len());

    loop {
        let starting_size = instructions.len();
        instructions.sort_unstable_by(|&a, &b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));

        next.push(instructions[0]);
        for rect in instructions.drain(..).skip(1) {
            let top = next.last_mut().unwrap();
            if let Some(merged) = top.merge(&rect) {
                *top = merged;
            } else {
                next.push(rect)
            }
        }

        next.sort_unstable_by(|&a, &b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
        instructions.push(next[0]);
        for rect in next.drain(..).skip(1) {
            let top = instructions.last_mut().unwrap();
            if let Some(merged) = top.merge(&rect) {
                *top = merged;
            } else {
                instructions.push(rect)
            }
        }

        if instructions.len() == starting_size {
            break;
        }
    }
}
