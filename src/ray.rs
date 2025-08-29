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

    pub fn color(&self) -> Color {
        const SKY: Color = Color::new(0.5, 0.7, 1.0);
        const WHITE: Color = Color::new(1.0, 1.0, 1.0);

        let t: f64 = self.hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5);
        if t > 0.0 {
            let normal: Vec3 = (self.at(t) - Vec3(0.0, 0.0, -1.0)).unit();
            Color(normal + 1.0) * 0.5
        } else {
            let unit_dir = self.dir.unit();
            let t: f64 = 0.5 * (unit_dir.y() + 1.0); // y ∈ [-1, 1] -> y ∈ [0, 1]
            crate::lerp(WHITE, SKY, t)
        }
    }
}
