use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::{dot, reflect, unit_vector, Vec3}};

pub trait Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

// Lambertian

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        return Some((self.albedo, scattered));
    }
}

// Metal

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Metal {
        Metal { albedo, fuzz: fuzz.min(1.0) }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected: Vec3 = reflect(&r.direction(), &rec.normal);
        let fuzzed: Vec3 = unit_vector(&reflected) + (self.fuzz * Vec3::random_unit_vector());
        let scattered: Ray = Ray::new(rec.p, fuzzed);
        if dot(&scattered.direction(), &rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}