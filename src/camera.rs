use log::info;

use crate::{color::{write_color, Color}, hittable::Hittable, interval::Interval, ray::Ray, vec3::{unit_vector, Point3, Vec3}};

pub struct Camera {
    aspect_ratio: f32,
    image_width: i32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

pub struct CameraBuilder {
    pub aspect_ratio: f32,
    pub image_width: i32,
}

impl Camera {
    fn ray_color(&self, r: &Ray, world: &dyn Hittable) -> Color {
        if let Some(hit_record) = world.hit(r, Interval::new(0.0, f32::MAX)) {
            return 0.5 * (hit_record.normal + Color::new(1.0, 1.0, 1.0));
        }
        let direction: Vec3 = r.direction();
        let unit_direction = unit_vector(&direction);
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
    }
    
    pub fn render(&self, world: &dyn Hittable) {
        println!("P3\n{0} {1}\n255", self.image_width, self.image_height);
        for j in 0..self.image_height {
            let lines_remaining = self.image_height - j;
            info!("\nScanlines remaining: {lines_remaining}");
            for i in 0..self.image_width {
                let pixel_center: Vec3 = self.pixel00_loc + (self.pixel_delta_u * i as f32) + (self.pixel_delta_v * j as f32);
                let ray_direction: Vec3 = pixel_center - self.center;
                let pixel_ray: Ray = Ray::new(self.center, ray_direction);
                let pixel_color: Color = self.ray_color(&pixel_ray, world);
                write_color(&pixel_color);
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
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v
        }
    }
}