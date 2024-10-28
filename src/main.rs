use std::rc::Rc;

use color::Color;
use hittable::Hittable;
use hittable_list::HittableList;
use interval::Interval;
use log::info;
use ray::Ray;
use sphere::Sphere;
use vec3::{Point3, Vec3, unit_vector};

mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod utils;
mod vec3;

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    if let Some(hit_record) = world.hit(r, Interval::new(0.0, f32::MAX)) {
        return 0.5 * (hit_record.normal + Color::new(1.0, 1.0, 1.0));
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

    // World

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

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
            let pixel_color: Color = ray_color(&pixel_ray, &world);
            color::write_color(&pixel_color);
        }
    }
}
