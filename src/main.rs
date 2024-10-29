use std::rc::Rc;

use camera::CameraBuilder;
use color::Color;
use hittable_list::HittableList;
use log::info;
use ray::Ray;
use sphere::Sphere;
use vec3::{Point3, Vec3};

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod utils;
mod vec3;

fn main() {
    env_logger::init();
    info!("Starting up");

    // Image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let samples_per_pixel: i32 = 100;
    let image_width: i32 = 400;
    let max_depth: i32 = 10;

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let camera = CameraBuilder {
        aspect_ratio,
        samples_per_pixel,
        image_width,
        max_depth,
    }.build();
    camera.render(&world);
}
