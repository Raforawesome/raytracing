use crate::Vec3;

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Color(pub Vec3);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color(Vec3(r, g, b))
    }
}

pub fn write_color(pixel_color: Color) {
    let Color(Vec3(r, g, b)) = pixel_color;

    let r_byte: i32 = (255.999_f64 * r) as i32;
    let g_byte: i32 = (255.999_f64 * g) as i32;
    let b_byte: i32 = (255.999_f64 * b) as i32;

    println!("{r_byte} {g_byte} {b_byte}");
}
