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

impl Ray {
    pub fn hit_sphere(&self, center: Point3, radius: f64) -> f64 {
        let oc: Vec3 = center - self.origin();
        let a: f64 = self.dir().dot(&self.dir());
        let b: f64 = -2.0 * oc.dot(&self.dir());
        let c: f64 = oc.dot(&oc) - radius * radius;
        let discrim: f64 = b * b - 4.0 * a * c;

        if discrim < 0.0 {
            -1.0
        } else {
            (-b - discrim.sqrt()) / (2.0 * a)
        }
    }
}
