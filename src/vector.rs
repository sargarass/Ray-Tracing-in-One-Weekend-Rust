use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait Dot: Sized + Copy {
    fn dot(self, w: Self) -> f32;
}

pub trait Norm: Sized + Copy + Dot {
    fn norm(self) -> f32 {
        self.dot(self)
    }
}

pub trait Len: Sized + Copy + Norm + Dot {
    fn len(self) -> f32 {
        self.norm().sqrt()
    }
}

pub trait Normalize: Sized + Copy + Len + Norm + Dot + std::ops::Div<f32, Output = Self> {
    fn normalize(self) -> Self {
        self.div(self.len())
    }
}

trait Cross: Sized + Copy {
    fn cross(self, v: Self) -> Self;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

impl Dot for Vec3 {
    fn dot(self, w: Self) -> f32 {
        return self.x * w.x + self.y * w.y + self.z * w.z;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self * v.x, self * v.y, self * v.z)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, r: f32) -> Vec3 {
        r.recip() * self
    }
}

impl Norm for Vec3 {}

impl Len for Vec3 {}

impl Normalize for Vec3 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_add() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(-1.0, -2.0, -3.0);

        assert_eq!(a + b, Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn vector_sub() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(a - b, Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn vector_neg() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(-a, Vec3::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn vector_mul() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(5.0 * a, Vec3::new(5.0, 10.0, 15.0));
    }

    #[test]
    fn vector_norm() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(a.norm(), 14.0);
    }

    #[test]
    fn vector_len() {
        let a = Vec3::new(4.0, 4.0, 2.0);
        assert_eq!(a.len(), 6.0);
    }

    #[test]
    fn vector_dot() {
        let a = Vec3::new(4.0, 4.0, 2.0);
        assert_eq!(Vec3::dot(a, a), 36.0);
    }

    #[test]
    fn vector_normalize() {
        let a = Vec3::new(4.0, 4.0, 2.0);
        assert_eq!(a.normalize(), Vec3::new(4.0 / 6.0, 4.0 / 6.0, 2.0 / 6.0));
    }
}
