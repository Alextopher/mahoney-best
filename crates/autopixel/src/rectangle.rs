#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Rectangle {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Rectangle {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn from_square(x: usize, y: usize, size: usize) -> Self {
        Self::new(x, y, size, size)
    }

    /// Crops the rectangle to fit within the picture
    pub fn crop(&self, picture: &Rectangle) -> Option<Rectangle> {
        if self.x >= picture.width || self.y >= picture.height {
            return None;
        }

        let width = self.width.min(picture.width - self.x);
        let height = self.height.min(picture.height - self.y);

        Some(Rectangle {
            x: self.x,
            y: self.y,
            width,
            height,
        })
    }

    /// Quad-tree decomposition
    pub fn split(&self) -> Option<[Rectangle; 4]> {
        debug_assert!(self.width.is_power_of_two() && self.width == self.height);

        if self.width == 1 {
            return None;
        }

        let size = self.width / 2;
        Some([
            Rectangle::from_square(self.x, self.y, size),
            Rectangle::from_square(self.x, self.y + size, size),
            Rectangle::from_square(self.x + size, self.y, size),
            Rectangle::from_square(self.x + size, self.y + size, size),
        ])
    }

    /// Merges 2 disjoint but neighboring rectangles
    pub fn merge(&self, other: &Rectangle) -> Option<Rectangle> {
        if self.x == other.x && self.width == other.width {
            if self.y + self.height == other.y {
                // 'self' is above
                return Some(Rectangle::new(
                    self.x,
                    self.y,
                    self.width,
                    self.height + other.height,
                ));
            } else if other.y + other.height == self.y {
                // 'other' is above
                return Some(Rectangle::new(
                    other.x,
                    other.y,
                    self.width,
                    self.height + other.height,
                ));
            }
        }

        if self.y == other.y && self.height == other.height {
            if self.x + self.width == other.x {
                // 'self' is left
                return Some(Rectangle::new(
                    self.x,
                    self.y,
                    self.width + other.width,
                    self.height,
                ));
            } else if other.x + other.width == self.x {
                // 'other' is left
                return Some(Rectangle::new(
                    other.x,
                    other.y,
                    self.width + other.width,
                    self.height,
                ));
            }
        }

        None
    }
}
