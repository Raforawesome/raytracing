use crate::{
    Point3, Vec3,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(x: f64, y: f64, z: f64, r: f64) -> Self {
        Sphere {
            center: Point3::new(x, y, z),
            radius: r.max(0.0),
        }
    }

    pub fn from_comps(center: Point3, radius: f64) -> Self {
        Sphere {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc: Vec3 = self.center - ray.origin();
        let a: f64 = ray.dir().length_squared();
        let h: f64 = oc.dot(&ray.dir());
        let c: f64 = oc.length_squared() - self.radius * self.radius;

        let discrim: f64 = h * h - a * c;
        if discrim < 0.0 {
            return None;
        }

        let sqrtd: f64 = discrim.sqrt();

        // get nearest root within range
        let mut root = (h - sqrtd) / a; // try negative root first
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a; // try positive root
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let mut record: HitRecord = HitRecord::default();
        record.point = ray.at(root);
        record.t = root;
        let outward_normal: Vec3 = (record.point - self.center) / self.radius;
        record.set_face_normal(ray, outward_normal);
        Some(record)
    }
}
