use raytracing::{
    Point3, Vec3,
    color::{Color, write_color},
    ray::Ray,
};

pub fn ray_color(ray: Ray) -> Color {
    Color(Vec3(0.0, 0.0, 0.0))
}

fn main() {
    // image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = ((image_width as f64 / aspect_ratio) as i32).max(1);

    // camera setup
    let focal_length: f64 = 1.0;
    let camera_center: Point3 = Point3::new(0.0, 0.0, 0.0);
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);

    // set up viewport vectors
    let viewport_u: Vec3 = Vec3(viewport_width, 0.0, 0.0);
    let viewport_v: Vec3 = Vec3(0.0, -viewport_height, 0.0);

    // calculate horizontal/vertical delta vectors between pixels
    let px_du = viewport_u / image_width as f64;
    let px_dv = viewport_v / image_height as f64;

    // upper left viewport & pixel coordinates
    let viewport_upper_left: Point3 =
        camera_center - Vec3(0.0, 0.0, focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);

    let px00_loc: Point3 = viewport_upper_left + 0.5 * (px_du + px_dv);

    // render
    println!("P3\n{image_width} {image_height}\n255");

    for j in 0..image_height {
        eprintln!("Lines remaining: {}", image_height - j);
        for i in 0..image_width {
            let px_center = px00_loc + (i as f64) * px_du + (j as f64) * px_dv;
            let ray_dir = px_center - camera_center;

            let ray: Ray = Ray::new(camera_center, ray_dir);

            let pixel_color: Color = ray.color();
            write_color(std::io::stdout(), pixel_color);
        }
    }
    eprintln!("Done.\n");
}
