use std::ops;

use rand::{distributions::Uniform, random, thread_rng, Rng};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }
    
    pub fn from_array(e: [f32; 3]) -> Vec3 {
        Vec3 { e }
    }

    pub fn random() -> Vec3 {
        Vec3 { e: [
            random::<f32>(),
            random::<f32>(),
            random::<f32>(),
        ]}
    }

    pub fn random_range(min: f32, max: f32) -> Vec3 {
        let mut rng = thread_rng();
        let dist = Uniform::new(min, max);
        Vec3{ e: [
            rng.sample(dist),
            rng.sample(dist),
            rng.sample(dist),
        ]}
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            let lensq = p.length_squared();
            if 1e-40 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        };
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        if dot(normal, &on_unit_sphere) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        return self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s 
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let e = self.e;
        Vec3 { e: [-e[0], -e[1], -e[2]] }
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 { e: [self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2]]}
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 { e: [self.e[0] - rhs.e[0], self.e[1] - rhs.e[1], self.e[2] - rhs.e[2]]}
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 { e: [self.e[0] * rhs.e[0], self.e[1] * rhs.e[1], self.e[2] * rhs.e[2]]}
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 { e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs]}
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self *= 1.0 / rhs
    }
}

// Utility functions
pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
    u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {
        e: [
            u.y() * v.z() - u.z() * v.y(),
            u.z() * v.x() - u.x() * v.z(),
            u.x() * v.y() - u.y() * v.x()
        ]
    }
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    return *v - 2.0 * dot(v, n) * *n;
}

pub type Point3 = Vec3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_construction() {
        let v = Vec3 { e: [1.0, 2.0, 3.0] };
        assert!(v.x() == 1.0);
        assert!(v.y() == 2.0);
        assert!(v.z() == 3.0);
    }

    #[test]
    fn test_negative() {
        let v = Vec3 { e: [1.0, 2.0, 3.0] };
        assert!((-v).x() == -1.0)
    }

    #[test]
    fn test_index() {
        let v = Vec3 { e: [1.0, 2.0, 3.0] };
        assert!(v[0] == 1.0)
    }
    
    #[test]
    fn test_add() {
        let v = Vec3 { e: [1.0, 2.0, 3.0] };
        let w = Vec3 { e: [4.0, 5.0, 6.0] };
        assert!(v + w == Vec3 { e: [5.0, 7.0, 9.0] })
    }

    #[test]
    fn test_add_assign() {
        let mut v = Vec3 { e: [1.0, 2.0, 3.0] };
        let w = Vec3 { e: [4.0, 5.0, 6.0] };
        v += w;
        assert!(v == Vec3 { e: [5.0, 7.0, 9.0] })
    }

    #[test]
    fn test_mul() {
        let v = Vec3 { e: [1.0, 2.0, 3.0] };
        assert!(v * 4.0 == Vec3 { e: [4.0, 8.0, 12.0] });
        assert!(4.0 * v == Vec3 { e: [4.0, 8.0, 12.0] })
    }
    
    #[test]
    fn test_mul_assign() {
        let mut v = Vec3 { e: [1.0, 2.0, 3.0] };
        v *= 4.0;
        assert!(v == Vec3 { e: [4.0, 8.0, 12.0] })
    }

    #[test]
    fn test_div_assign() {
        let mut v = Vec3 { e: [4.0, 8.0, 12.0] };
        v /= 4.0;
        assert!(v == Vec3 { e: [1.0, 2.0, 3.0] })
    }

    #[test]
    fn test_length() {
        let v = Vec3 { e: [0.0, 3.0, 4.0] };
        assert!(v.length_squared() == 25.0);
        assert!(v.length() == 5.0)
    }

    #[test]
    fn test_unit_vector() {
        let v = Vec3 { e: [0.0, 3.0, 4.0] };
        assert!(unit_vector(&v) == Vec3 { e: [ 0.0, 0.6, 0.8 ]})
    }
}