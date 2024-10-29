use std::rc::Rc;

use camera::CameraBuilder;
use color::Color;
use hittable_list::HittableList;
use log::info;
use material::{Lambertian, Metal};
use sphere::Sphere;
use vec3::{Point3};

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

fn main() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();
    info!("Starting up");

    // Image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let samples_per_pixel: i32 = 100;
    let image_width: i32 = 400;
    let max_depth: i32 = 10;

    // World
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center.clone())));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground.clone())));
    world.add(Rc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left.clone())));
    world.add(Rc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right.clone())));

    let camera = CameraBuilder {
        aspect_ratio,
        samples_per_pixel,
        image_width,
        max_depth,
    }.build();
    camera.render(&world);
}
