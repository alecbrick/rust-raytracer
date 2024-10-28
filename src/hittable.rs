use crate::{interval::Interval, ray::Ray, vec3::{dot, Point3, Vec3}};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

pub struct HitRecordBuilder {
    pub p: Point3,
    pub outward_normal: Vec3,
    pub t: f32,
    pub r: Ray,
}

impl HitRecordBuilder {
    pub fn build(self) -> HitRecord {
        let front_face = dot(&self.r.direction(), &self.outward_normal) < 0.0;
        HitRecord {
            p: self.p,
            normal: if front_face {self.outward_normal.clone()} else {-self.outward_normal.clone()},
            t: self.t,
            front_face: front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}