extern crate image;

use std::path::Path;
use std::slice;
use self::image::*;

/// Represents an RGBA color, each coordinate ranges between 0.0 and 1.0.
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64
}

impl Color {
    /// Creates a new color given the red, green, blue and alpha values.
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Color {
        Color { r: r, g: g, b: b, a: a }
    }
}

// TODO: Implement scalar multiplication, addition, subtraction for Color.

// Stores an image and its dimensions.
pub struct Image {
    /// The width of the image in pixels.
    pub width: usize,
    /// The height of the image in pixels.
    pub height: usize,
    /// The colors of each pixel, stored line by line in a one-dimensional vector.
    pixels: Vec<Color>
}

impl Image {
    /// Creates a new (transparent) image of the given dimensions.
    pub fn new(width: usize, height: usize) -> Image {
        Image { width: width, height: height, pixels: vec![Color::new(0.0,0.0,0.0,0.0); width * height] }
    }

    /// Returns the color of the pixel at (x,y).
    pub fn get(&self, x: usize, y: usize) -> Color {
        self.pixels[y * self.width + x]
    }

    /// Changes the pixel at (x,y) to the given color and returns a reference to the new image.
    pub fn set(&mut self, x: usize, y: usize, c: Color) -> &Image {
        self.pixels[y * self.width + x] = c;
        self
    }

    /// Returns a mutable iterator over the pixels of the image.
    ///
    /// ```
    /// use raydiancy::img_output::*;
    /// use std::path::*;
    /// let mut img = Image::new(255,255);
    /// for (x,y,col) in img.iter_mut() {
    ///     *col = Color::new(x as f64/255.0,y as f64/255.0,0.0,1.0);
    /// }
    /// // Now, img transitions is black at the top-left, green at the bottom-left,
    /// // red at the top-right and yellow at the bottom-right.
    /// // assert_eq!(img.get(255,255).r, 1.0);
    /// ```
    pub fn iter_mut(&mut self) -> ImageIterator {
        ImageIterator { pixels: self.pixels.iter_mut(), width: self.width, x: 0, y: 0 }
    }
}

/// Iterator over the pixels of an `Image`.
///
/// The documentation for `Image::iter_mut` includes an example of usage.
pub struct ImageIterator<'a> {
    pixels: slice::IterMut<'a, Color>,
    x: usize,
    y: usize,
    width: usize
}

impl<'a> Iterator for ImageIterator<'a> {
    type Item = (usize, usize, &'a mut Color);

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.pixels.next().map(|iter| (self.x, self.y, iter));
        self.x += 1;
        if self.x >= self.width {
            self.x = 0;
            self.y += 1;
        }
        result
    }
}

/// Writes a given image to the given file path.
///
/// The file type is determined by the file extension. Only ".png" was tested.
pub fn write_pixels_to_file(image: Image, filepath: &Path) {
    let mut output: RgbaImage = ImageBuffer::new(image.width as u32, image.height as u32);
    for (x, y, pixel) in output.enumerate_pixels_mut() {
        let c = image.get(x as usize, y as usize);
        *pixel = Rgba([to_u8(c.r), to_u8(c.g), to_u8(c.b), to_u8(c.a)]);
    }
    output.save(filepath).unwrap()
}

/// Converts a floating point value between 0 and 1 to an integer between 0 and 255.
fn to_u8(x: f64) -> u8 {
    (x * 255.0) as u8
}
