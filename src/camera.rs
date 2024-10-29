use log::info;
use rand::random;

use crate::{color::{write_color, Color}, hittable::Hittable, interval::Interval, ray::Ray, vec3::{unit_vector, Point3, Vec3}};

pub struct Camera {
    aspect_ratio: f32,
    samples_per_pixel: i32,
    max_depth: i32,
    image_width: i32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

pub struct CameraBuilder {
    pub aspect_ratio: f32,
    pub samples_per_pixel: i32,
    pub image_width: i32,
    pub max_depth: i32,
}

impl Camera {
    fn ray_color(&self, r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        if let Some(hit_record) = world.hit(r, Interval::new(0.001, f32::MAX)) {
            let direction = hit_record.normal + Vec3::random_unit_vector();
            return 0.5 * self.ray_color(&Ray::new(hit_record.p, direction), depth - 1, world);
        }
        let direction: Vec3 = r.direction();
        let unit_direction = unit_vector(&direction);
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(random::<f32>() - 0.5, random::<f32>() - 0.5, 0.0)
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_center: Vec3 = self.pixel00_loc + (self.pixel_delta_u * (i as f32 + offset.x())) + (self.pixel_delta_v * ((j as f32) + offset.y()));
        let ray_direction: Vec3 = pixel_center - self.center;
        Ray::new(self.center, ray_direction)
    }
    
    pub fn render(&self, world: &dyn Hittable) {
        println!("P3\n{0} {1}\n255", self.image_width, self.image_height);
        for j in 0..self.image_height {
            let lines_remaining = self.image_height - j;
            info!("\nScanlines remaining: {lines_remaining}");
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r: Ray = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }
                let average_color = pixel_color / self.samples_per_pixel as f32;
                write_color(&average_color);
            }
        }
    }
}

impl CameraBuilder {
    pub fn build(&self) -> Camera {
        // image
        let aspect_ratio = self.aspect_ratio;
        let image_width = self.image_width;
        let mut image_height: i32 = ((image_width as f32) / aspect_ratio) as i32;
        if image_height < 1 {
            image_height = 1;
        }

        // Camera
        let focal_length: f32 = 1.0;
        let viewport_height: f32 = 2.0;
        let viewport_width: f32 = viewport_height * (image_width as f32 / image_height as f32);
        let center: Point3 = Point3::new(0.0, 0.0, 0.0);

        // viewport vectors
        let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);

        // Pixel deltas
        let pixel_delta_u: Vec3 = viewport_u / image_width as f32;
        let pixel_delta_v: Vec3 = viewport_v / image_height as f32;

        // Upper left pixel
        let viewport_upper_left: Vec3 = center - Vec3::new(0.0, 0.0, focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);
        let pixel00_loc: Vec3 = viewport_upper_left + (pixel_delta_u / 2.0) + (pixel_delta_v / 2.0);

        Camera {
            aspect_ratio,
            samples_per_pixel: self.samples_per_pixel,
            max_depth: self.max_depth,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v
        }
    }
}