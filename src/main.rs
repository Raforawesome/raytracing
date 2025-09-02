use raytracing::{
    camera::Camera,
    hittable::{Hittable as _, HittableList},
    sphere::Sphere,
};

fn main() {
    // world setup
    let mut world: HittableList = HittableList::default();
    world.add(Sphere::new(0.0, 0.0, -1.0, 0.5).boxed());
    world.add(Sphere::new(0.0, -100.5, -1.0, 100.0).boxed());

    let cam: Camera = Camera::new(16.0 / 9.0, 400);
    cam.render(&world);
}
