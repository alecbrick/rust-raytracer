use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

pub fn write_color(pixel_color: &Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let intensity = Interval::new(0.0, 0.999);

    let ir = (256.0 * intensity.clamp(r)) as i32;
    let ig = (256.0 * intensity.clamp(g)) as i32;
    let ib = (256.0 * intensity.clamp(b)) as i32;

    println!("{ir} {ig} {ib}");
}