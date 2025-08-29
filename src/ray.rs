use crate::{Point3, Vec3, color::Color};

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
        let a: f64 = self.dir().length_squared();
        let h: f64 = oc.dot(&self.dir());
        let c: f64 = oc.length_squared() - radius * radius;
        let discrim: f64 = h * h - a * c;

        if discrim < 0.0 {
            -1.0
        } else {
            (h - discrim.sqrt()) / a
        }
    }

    pub fn color(&self) -> Color {
        let sky: Color = Color::new(0.5, 0.7, 1.0);
        let white: Color = Color::new(1.0, 1.0, 1.0);

        let t: f64 = self.hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5);
        if t > 0.0 {
            let normal: Vec3 = (self.at(t) - Vec3(0.0, 0.0, -1.0)).unit();
            Color(normal + 1.0) * 0.5
        } else {
            let unit_dir = self.dir.unit();
            let t: f64 = 0.5 * (unit_dir.y() + 1.0); // y ∈ [-1, 1] -> y ∈ [0, 1]
            crate::lerp(white, sky, t)
        }
    }
}
