use crate::hittable::{HitRecord, HitRecordBuilder, Hittable};
use crate::ray::Ray;
use crate::vec3::{dot, unit_vector, Point3, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Sphere {
        Sphere {
            center: center,
            radius: radius.max(0.0)
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let ray_origin: Point3 = r.origin();
        let ray_dir: Vec3 = r.direction();
        let cq = self.center - ray_origin;

        let a = ray_dir.length_squared();
        let h = dot(&ray_dir, &cq);
        let c = cq.length_squared() - self.radius.powi(2);

        let discriminant = h.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        } 
        let sqrtd = discriminant.sqrt();
        let mut t = (h - sqrtd) / a;
        if t < t_min || t > t_max {
            t = (h + sqrtd) / a;
            if t < t_min || t > t_max {
                return None;
            }
        }
        let p = r.at(t);
        let outward_normal = unit_vector(&(p - self.center));
        let hit_record_builder = HitRecordBuilder {
            p: p,
            outward_normal: outward_normal,
            t: t,
            r: r.clone(),
        };
        Some(hit_record_builder.build())
    }
}