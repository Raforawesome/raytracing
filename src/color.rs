use crate::Vec3;

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Color3(pub Vec3);

pub fn write_color(pixel_color: Color3) {
    let Color3(Vec3(r, g, b)) = pixel_color;

    let r_byte: i32 = (255.999_f64 * r) as i32;
    let g_byte: i32 = (255.999_f64 * g) as i32;
    let b_byte: i32 = (255.999_f64 * b) as i32;

    println!("{r_byte} {g_byte} {b_byte}");
}
