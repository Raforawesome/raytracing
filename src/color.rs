use std::{
    io::Write,
    ops::{Add, Mul, Sub},
};

use crate::Vec3;

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
/// A newtype wrapper over Vec3 which represents a color. Each
/// parameter is in the range [0.0, 1.0] and gets converted to
/// an RGB parameter between 0-255 on write.
pub struct Color(pub Vec3);

impl Color {
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Color(Vec3(r, g, b))
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Color(Vec3(self.0.0 * rhs, self.0.1 * rhs, self.0.2 * rhs))
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color(Vec3(
            self.0.0 + rhs.0.0,
            self.0.1 + rhs.0.1,
            self.0.2 + rhs.0.2,
        ))
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Color(Vec3(
            self.0.0 - rhs.0.0,
            self.0.1 - rhs.0.1,
            self.0.2 - rhs.0.2,
        ))
    }
}

pub fn write_color(mut out: impl Write, pixel_color: Color) {
    let Color(Vec3(r, g, b)) = pixel_color;

    let r_byte: i32 = (255.999_f64 * r) as i32;
    let g_byte: i32 = (255.999_f64 * g) as i32;
    let b_byte: i32 = (255.999_f64 * b) as i32;

    writeln!(out, "{r_byte} {g_byte} {b_byte}").expect("Valid output file");
}
