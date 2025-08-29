use crate::{
    Point3, Vec3,
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
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
        if root <= ray_tmin || root >= ray_tmax {
            root = (h + sqrtd) / a; // try positive root
            if root <= ray_tmin || root >= ray_tmax {
                return None;
            }
        }

        let point: Point3 = ray.at(root);
        Some(HitRecord {
            point,
            normal: (point - self.center) / self.radius,
            t: root,
        })
    }
}
