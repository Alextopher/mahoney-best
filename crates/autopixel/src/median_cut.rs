//! Implements the median cut algorithm for color quantization.

use std::{
    cmp::{max, min},
    collections::{BinaryHeap, HashSet},
    ops::RangeInclusive,
};

use image::Rgb;

use crate::Color;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ColorRange {
    reds: RangeInclusive<u8>,
    greens: RangeInclusive<u8>,
    blues: RangeInclusive<u8>,
}

impl ColorRange {
    fn from_colors(colors: &[Color]) -> Self {
        // Single pass algorithm to find the min and max of each color channel
        let (mut min_red, mut max_red) = (u8::MAX, u8::MIN);
        let (mut min_green, mut max_green) = (u8::MAX, u8::MIN);
        let (mut min_blue, mut max_blue) = (u8::MAX, u8::MIN);

        for color in colors {
            min_red = min(min_red, color[0]);
            max_red = max(max_red, color[0]);
            min_green = min(min_green, color[1]);
            max_green = max(max_green, color[1]);
            min_blue = min(min_blue, color[2]);
            max_blue = max(max_blue, color[2]);
        }

        Self {
            reds: min_red..=max_red,
            greens: min_green..=max_green,
            blues: min_blue..=max_blue,
        }
    }

    fn contains(&self, color: &Color) -> bool {
        self.reds.contains(&color[0])
            && self.greens.contains(&color[1])
            && self.blues.contains(&color[2])
    }

    /// Checks if this space fully contains another space.
    fn contains_space(&self, other: &Self) -> bool {
        self.reds.start() <= other.reds.start()
            && self.reds.end() >= other.reds.end()
            && self.greens.start() <= other.greens.start()
            && self.greens.end() >= other.greens.end()
            && self.blues.start() <= other.blues.start()
            && self.blues.end() >= other.blues.end()
    }

    fn range(&self) -> u32 {
        let red_range = self.reds.end() - self.reds.start();
        let green_range = self.greens.end() - self.greens.start();
        let blue_range = self.blues.end() - self.blues.start();

        red_range as u32 * green_range as u32 * blue_range as u32
    }

    pub fn median(&self) -> Color {
        fn median_u8(range: &RangeInclusive<u8>) -> u8 {
            let start = *range.start();
            let end = *range.end();
            ((start ^ end) >> 1) + (start & end)
        }

        Rgb([
            median_u8(&self.reds),
            median_u8(&self.greens),
            median_u8(&self.blues),
        ])
    }
}

impl PartialOrd for ColorRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ColorRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.range().cmp(&other.range())
    }
}

/// A [`Bucket`] is a mutable slice of colors that can be split into two other buckets by partitioning
/// the colors along the axis with the largest range.
#[derive(Debug)]
struct Bucket<'a> {
    colors: &'a mut [Color],
    space: ColorRange,
}

impl<'a> Bucket<'a> {
    fn new(colors: &'a mut [Color]) -> Self {
        let space = ColorRange::from_colors(colors);
        Self { colors, space }
    }

    /// The core of median cut algorithm. Splits the bucket into two buckets along the axis with the
    /// largest range.
    ///
    /// This method uses a quick-select algorithm to find the median color along the axis, while partitioning at the same time.
    fn split(self) -> (Self, Self) {
        let colors = self.colors;
        let ColorRange {
            reds,
            greens,
            blues,
        } = self.space;

        let (left, _median, right) = if reds.len() >= greens.len() && reds.len() >= blues.len() {
            colors.select_nth_unstable_by_key(colors.len() / 2, |color| color[0])
        } else if greens.len() >= reds.len() && greens.len() >= blues.len() {
            colors.select_nth_unstable_by_key(colors.len() / 2, |color| color[1])
        } else {
            colors.select_nth_unstable_by_key(colors.len() / 2, |color| color[2])
        };

        return (Self::new(left), Self::new(right));
    }
}

impl PartialEq for Bucket<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.space == other.space
    }
}

impl Eq for Bucket<'_> {}

impl PartialOrd for Bucket<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Bucket<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.space.cmp(&other.space)
    }
}

/// Binary tree datastructure for median cut algorithm
#[derive(Debug)]
pub struct Quantizer {
    nodes: Vec<QuantizerNode>,
}

#[derive(Debug)]
struct QuantizerNode {
    space: ColorRange,
    children: Option<(usize, usize)>,
}

impl Quantizer {
    fn new(space: ColorRange, num_colors: usize) -> Self {
        let mut nodes = Vec::with_capacity(2 * num_colors - 1);
        nodes.push(QuantizerNode {
            space,
            children: None,
        });

        Self { nodes }
    }

    fn split(&mut self, node: usize, left: ColorRange, right: ColorRange) -> (usize, usize) {
        debug_assert!(
            self.nodes[node].space.contains_space(&left),
            "{:?} {:?}",
            self.nodes[node].space,
            left
        );
        debug_assert!(
            self.nodes[node].space.contains_space(&right),
            "{:?} {:?}",
            self.nodes[node].space,
            right
        );

        // Add left & right nodes
        let left_idx = self.nodes.len();
        self.nodes.push(QuantizerNode {
            space: left,
            children: None,
        });

        let right_idx = self.nodes.len();
        self.nodes.push(QuantizerNode {
            space: right,
            children: None,
        });

        let node = &mut self.nodes[node];
        node.children = Some((left_idx, right_idx));

        (left_idx, right_idx)
    }

    pub fn quantize(&self, color: &Color) -> Color {
        // Traverse the tree to find the leaf node that contains the color
        let mut root = 0;
        while let Some((left, right)) = self.nodes[root].children {
            let left_space = &self.nodes[left].space;
            let right_space = &self.nodes[right].space;

            if left_space.contains(color) {
                root = left;
            } else if right_space.contains(color) {
                root = right;
            } else {
                break;
            }
        }
        self.nodes[root].space.median()
    }
}

struct HeapItem<'a>(Bucket<'a>, usize);

impl PartialEq for HeapItem<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for HeapItem<'_> {}

impl PartialOrd for HeapItem<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapItem<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

/// Median cut algorithm
pub fn median_cut(mut colors: Vec<Color>, num_colors: usize) -> Quantizer {
    // The number of colors must be at least the number of unique colors in the image
    let mut set = HashSet::new();
    for color in &colors {
        set.insert(color);
        if set.len() >= num_colors {
            break;
        }
    }
    let num_colors = set.len();

    let start = Bucket::new(&mut colors);
    let mut quantizer = Quantizer::new(start.space.clone(), num_colors);
    let mut max_heap = BinaryHeap::from(vec![HeapItem(start, 0)]);

    // Split the bucket with the largest range until we have the desired number of colors
    while max_heap.len() < num_colors {
        let HeapItem(bucket, node) = max_heap.pop().unwrap();
        let (left, right) = bucket.split();

        let (left_idx, right_idx) = quantizer.split(node, left.space.clone(), right.space.clone());

        max_heap.push(HeapItem(left, left_idx));
        max_heap.push(HeapItem(right, right_idx));
    }

    quantizer
}
