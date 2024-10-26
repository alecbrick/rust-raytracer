use log::info;

mod color;
mod ray;
mod vec3;

use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3, dot, unit_vector};

fn hit_sphere(center: &Point3, radius: f32, r: &Ray) -> bool {
    let ray_origin: Point3 = r.origin();
    let ray_dir: Vec3 = r.direction();
    let cq = *center - ray_origin;

    let a = dot(&ray_dir, &ray_dir);
    let b = -2.0 * dot(&ray_dir, &cq);
    let c = dot(&cq, &cq) - radius.powi(2);

    let discriminant = b.powi(2) - 4.0 * a * c;
    return discriminant >= 0.0;
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let direction: Vec3 = r.direction();
    let unit_direction = unit_vector(&direction);
    let a = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    env_logger::init();
    info!("Starting up");

    // Image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let mut image_height: i32 = ((image_width as f32) / aspect_ratio) as i32;
    if image_height < 1 {
        image_height = 1;
    }

    // Camera
    let focal_length: f32 = 1.0;
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center: Point3 = Point3::new(0.0, 0.0, 0.0);

    // viewport vectors
    let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);

    // Pixel deltas
    let pixel_delta_u: Vec3 = viewport_u / image_width as f32;
    let pixel_delta_v: Vec3 = viewport_v / image_height as f32;

    // Upper left pixel
    let viewport_upper_left: Vec3 = camera_center - Vec3::new(0.0, 0.0, focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);
    let pixel_00_loc: Vec3 = viewport_upper_left + (pixel_delta_u / 2.0) + (pixel_delta_v / 2.0);

    println!("P3\n{image_width} {image_height}\n255");
    for j in 0..image_height {
        let lines_remaining = image_height - j;
        info!("\nScanlines remaining: {lines_remaining}");
        for i in 0..image_width {
            let pixel_center: Vec3 = pixel_00_loc + (pixel_delta_u * i as f32) + (pixel_delta_v * j as f32);
            let ray_direction: Vec3 = pixel_center - camera_center;
            let pixel_ray: Ray = Ray::new(camera_center, ray_direction);
            let pixel_color: Color = ray_color(&pixel_ray);
            color::write_color(&pixel_color);
        }
    }
}
