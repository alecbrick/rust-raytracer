use crate::vec3::{Point3, Vec3};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Ray {
    origin: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3) -> Ray {
        Ray { origin, dir }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + (self.dir * t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_construction() {
        let p = Point3::new(1.0, 2.0, 3.0);
        let d = Vec3::new(4.0, 5.0, 6.0);
        let r = Ray::new(p, d);

        assert!(r.origin() == p);
        assert!(r.direction() == d);
    }

    #[test]
    fn test_at() {
        let p = Point3::new(1.0, 2.0, 3.0);
        let d = Vec3::new(4.0, 5.0, 6.0);
        let r = Ray::new(p, d);
        
        assert!(r.at(3.0) == Point3::new(13.0, 17.0, 21.0));
    }
}