extern crate image;

pub use color::*;
use std::path::Path;
use std::slice;
use self::image::*;

// Stores an image and its dimensions.
pub struct Image {
    /// The width of the image in pixels.
    pub width: usize,
    /// The height of the image in pixels.
    pub height: usize,
    /// The colors of each pixel, stored line by line in a one-dimensional vector.
    pixels: Vec<AColor>
}

impl Image {
    /// Creates a new (transparent) image of the given dimensions.
    pub fn new(width: usize, height: usize) -> Image {
        Image { width: width, height: height, pixels: vec![AColor::transparent(); width * height] }
    }

    /// Returns the color of the pixel at (x,y).
    pub fn get(&self, x: usize, y: usize) -> AColor {
        self.pixels[y * self.width + x]
    }

    /// Changes the pixel at (x,y) to the given color and returns a reference to the new image.
    pub fn set(&mut self, x: usize, y: usize, c: AColor) -> &Image {
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
    ///     *col = AColor::new(x as f64/255.0,y as f64/255.0,0.0);
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
    pixels: slice::IterMut<'a, AColor>,
    x: usize,
    y: usize,
    width: usize
}

impl<'a> Iterator for ImageIterator<'a> {
    type Item = (usize, usize, &'a mut AColor);

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
        let (r,g,b,a) = c.to_rgba();
        *pixel = Rgba([r,g,b,a]);
    }
    output.save(filepath).unwrap()
}
