pub mod color;
pub mod ray;
pub mod vec3;

use std::ops::{Add, Mul, Sub};

pub use vec3::Vec3;

pub fn lerp<T>(a: T, b: T, t: f64) -> T
where
    T: Add<Output = T> + Sub<Output = T> + Mul<f64, Output = T> + Copy,
{
    a + (b - a) * t
}

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
/// Point3 is an newtype wrapper over Vec3 to represent a point
/// in 3D space. In contrast, Vec3 is typically used for raw directions.
pub struct Point3(pub Vec3);

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3(Vec3(x, y, z))
    }

    pub fn unit(&self) -> Vec3 {
        self.0.unit()
    }
}

impl Add<Vec3> for Point3 {
    type Output = Self;

    fn add(self, other: Vec3) -> Self::Output {
        Point3(Vec3(
            self.0.0 + other.0,
            self.0.1 + other.1,
            self.0.2 + other.2,
        ))
    }
}

impl Sub<Vec3> for Point3 {
    type Output = Self;

    fn sub(self, other: Vec3) -> Self::Output {
        Point3(Vec3(
            self.0.0 - other.0,
            self.0.1 - other.1,
            self.0.2 - other.2,
        ))
    }
}

impl Sub for Point3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self::Output {
        Vec3(
            self.0.0 - other.0.0,
            self.0.1 - other.0.1,
            self.0.2 - other.0.2,
        )
    }
}
