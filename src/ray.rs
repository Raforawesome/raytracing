use std::f64::INFINITY;

use crate::{Point3, Vec3, color::Color, hittable::Hittable};

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

impl Ray {
    pub fn color(&self, world: &impl Hittable) -> Color {
        if let Some(record) = world.hit(&self, 0.0, INFINITY) {
            (record.normal + Color::new(1.0, 1.0, 1.0)) * 0.5
        } else {
            let sky: Color = Color::new(0.5, 0.7, 1.0);
            let white: Color = Color::new(1.0, 1.0, 1.0);
            let unit_dir = self.dir.unit();
            let t: f64 = 0.5 * (unit_dir.y() + 1.0); // y ∈ [-1, 1] -> y ∈ [0, 1]
            crate::lerp(white, sky, t)
        }
    }
}
