#![allow(unused, dead_code)]

pub mod color;
pub mod ray;
pub mod vec3;

use std::ops::Add;

pub use vec3::Vec3;

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Point3(pub Vec3);

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
