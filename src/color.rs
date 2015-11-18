use std::ops;

/// The gamma value used for gamma correction.
const GAMMA_VALUE: f64 = 2.2;

fn is_in_unit_interval(x: f64) -> bool {
    0. <= x && x <= 1.
}

/// Represents an RGB color, each channel ranges between 0.0 and 1.0.
#[derive(Debug, Copy, Clone)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    /// Creates a new (opaque) Color given the red, green, blue values.
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        assert!(is_in_unit_interval(r) && is_in_unit_interval(g) && is_in_unit_interval(b));
        Color { r: r, g: g, b: b }
    }

    /// Creates a gray color with the given brightness. 0 -> black, 1 -> white.
    pub fn new_gray(b: f64) -> Color {
        Color::new(b, b, b)
    }

    /// Returns the red channel of the color.
    pub fn red(&self) -> f64 {
        self.r
    }

    /// Returns the green channel of the color.
    pub fn green(&self) -> f64 {
        self.g
    }

    /// Returns the blue channel of the color.
    pub fn blue(&self) -> f64 {
        self.b
    }

    /// Convert to an opaque color with alpha channel.
    pub fn with_alpha(self) -> AColor {
        AColor { c: self, a: 0.0 }
    }
}

impl ops::Add for Color {
    type Output = Color;

    fn add(self, c: Color) -> Color {
        Color::new(
            self.r + c.r,
            self.g + c.g,
            self.b + c.b,
        )
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, c: Color) -> Color {
        Color::new(
            self * c.r,
            self * c.g,
            self * c.b,
        )
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, c: Color) -> Color {
        Color::new(
            self.r * c.r,
            self.g * c.g,
            self.b * c.b,
        )
    }
}

pub fn black() -> Color {
    Color::new(0., 0., 0.)
}

pub fn white() -> Color {
    Color::new(1., 1., 1.)
}

/// Represents an RGB color with transparency.
/// For a background color b, the final color is `c + a * b`.
#[derive(Debug, Copy, Clone)]
pub struct AColor {
    c: Color,
    a: f64,
}

impl AColor {
    /// Creates a new (opaque) AColor given the red, green, blue values.
    pub fn new(r: f64, g: f64, b: f64) -> AColor {
        AColor::newa(r, g, b, 0.0)
    }

    fn newa(r: f64, g: f64, b: f64, a: f64) -> AColor {
        assert!(r + a <= 1.0 && g + a <= 1.0 && b + a <= 1.0);
        AColor { c: Color::new(r, g, b), a: a }
    }

    /// Returns the opaque part of the color.
    pub fn opaque(&self) -> Color {
        self.c
    }

    /// Returns the transparency of the color. 0 means opaque, 1 means transparent.
    pub fn transparency(&self) -> f64 {
        self.a
    }

    /// Creates a AColor that is completely transparent.
    pub fn transparent() -> AColor {
        AColor::newa(0.0, 0.0, 0.0, 1.0)
    }

    /// Converts the color to RGBA.
    pub fn to_rgba(&self) -> (u8,u8,u8,u8) {
        if self.a == 1. {
            return (0,0,0,0)
        }
        let c = (1. - self.a) * self.c;
        (to_u8(gamma_correct(c.r)),
         to_u8(gamma_correct(c.g)),
         to_u8(gamma_correct(c.b)),
         0xff - to_u8(self.a),
        )
    }
}

impl ops::Add for AColor {
    type Output = AColor;

    fn add(self, c: AColor) -> AColor {
        AColor {
            c: self.c + c.c,
            a: self.a + c.a,
        }
    }
}

impl ops::Mul<AColor> for f64 {
    type Output = AColor;

    fn mul(self, c: AColor) -> AColor {
        AColor {
            c: self * c.c,
            a: self * c.a,
        }
    }
}

/// Converts a floating point value between 0 and 1 to an integer between 0 and 255.
fn to_u8(x: f64) -> u8 {
    assert!(!x.is_nan());
    (x * 255.0) as u8
}

/// Applies standard gamma correction (2.2) to the AColor.
fn gamma_correct(x: f64) -> f64 {
    x.powf(1./GAMMA_VALUE)
}
