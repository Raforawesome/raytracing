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

    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);

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
