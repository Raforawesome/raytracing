use crate::{Point3, Vec3};

pub struct Ray {
    origin: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3) -> Self {
        Ray { origin, dir }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.dir * t
    }

    pub const fn origin(&self) -> Point3 {
        self.origin
    }

    pub const fn dir(&self) -> Vec3 {
        self.dir
    }
}

impl Ray {}
