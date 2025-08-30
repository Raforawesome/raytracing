use crate::{Point3, Vec3, ray::Ray};

#[derive(Default)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    /// Updates the record's `normal` and `front_face` properties.
    /// The parameter `outward_normal` is expected to have unit length.
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        // if ray and normal are opposite directions, the ray hit from outside
        self.front_face = ray.dir().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;

    fn boxed(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

/*
 * Hittable list
 */
#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest: f64 = ray_tmax;

        for obj in &self.objects {
            if let Some(record) = obj.hit(ray, ray_tmin, closest) {
                closest = record.t;
                temp_rec = Some(record);
            }
        }

        temp_rec
    }
}
