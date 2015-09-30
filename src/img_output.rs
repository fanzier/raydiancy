extern crate image;

use raytrace::*;
use std::path::Path;
use std::slice;
use self::image::*;

pub struct Image {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image { width: width, height: height, pixels: vec![Color::new(0.0,0.0,0.0,0.0); width * height] }
    }

    pub fn get(&self, x: usize, y: usize) -> Color {
        self.pixels[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, c: Color) -> &Image {
        self.pixels[y * self.width + x] = c;
        self
    }

    pub fn iter_mut(&mut self) -> ImageIterator {
        ImageIterator { pixels: self.pixels.iter_mut(), width: self.width, x: 0, y: 0 }
    }
}

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

pub fn write_pixels_to_file(image: Image, filepath: &Path) {
    let mut output: RgbaImage = ImageBuffer::new(image.width as u32, image.height as u32);
    for (x, y, pixel) in output.enumerate_pixels_mut() {
        let c = image.get(x as usize, y as usize);
        *pixel = Rgba([to_u8(c.r), to_u8(c.g), to_u8(c.b), to_u8(c.a)]);
    }
    output.save(filepath).unwrap()
}

fn to_u8(x: f64) -> u8 {
    (x * 255.0) as u8
}
