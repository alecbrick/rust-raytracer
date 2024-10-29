use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

pub fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn write_color(pixel_color: &Color) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Apply a linear to gamma transform for gamma 2
    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let intensity = Interval::new(0.0, 0.999);

    let ir = (256.0 * intensity.clamp(r)) as i32;
    let ig = (256.0 * intensity.clamp(g)) as i32;
    let ib = (256.0 * intensity.clamp(b)) as i32;

    println!("{ir} {ig} {ib}");
}