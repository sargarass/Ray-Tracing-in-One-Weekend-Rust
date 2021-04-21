use std::ops::{Add, Div, Mul, Neg, Sub};
use rand::Rng;
use rand::distributions::{Distribution, Uniform};

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

pub trait Cross: Sized + Copy {
    fn cross(self, v: Self) -> Self;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn x(&self) -> f32 {
        self.0
    }
    pub fn y(&self) -> f32 {
        self.1
    }
    pub fn z(&self) -> f32 {
        self.2
    }

    pub fn zero() -> Self {
        Vec3(0.0, 0.0, 0.0)
    }
}

impl Dot for Vec3 {
    fn dot(self, w: Self) -> f32 {
        self.0 * w.0 + self.1 * w.1 + self.2 * w.2
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, v: Vec3) -> Vec3 {
        Vec3(self.0 + v.0, self.1 + v.1, self.2 + v.2)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, v: Vec3) -> Vec3 {
        Vec3(self.0 - v.0, self.1 - v.1, self.2 - v.2)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3(self * v.0, self * v.1, self * v.2)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, r: f32) -> Vec3 {
        r.recip() * self
    }
}

impl Norm for Vec3 {}

impl Len for Vec3 {}

impl Normalize for Vec3 {}

impl Cross for Vec3 {
    fn cross(self, v: Self) -> Self {
        Vec3(
            self.1 * v.2 - self.2 * v.1,
            -(self.0 * v.2 - self.2 * v.0),
            self.0 * v.1 - self.1 * v.0,
        )
    }
}

pub fn random_in_unit_circle<R: Rng>(rng: &mut R) -> (f32, f32) {
    let between = Uniform::from(0.0..=1.0);
    let u = between.sample(rng);
    let v = between.sample(rng);
    let w = between.sample(rng);
    let s = between.sample(rng);

    let norm = f32::sqrt(u*u + v*v + w*w + s*s);
    return (u / norm, v / norm);
}

pub fn random_in_unit_sphere<R: Rng>(rng: &mut R) -> (f32, f32, f32) {
    let between = Uniform::from(0.0..=1.0);
    let u = between.sample(rng);
    let v = between.sample(rng);
    let w = between.sample(rng);
    let s = between.sample(rng);
    let t = between.sample(rng);

    let norm = f32::sqrt(u*u + v*v + w*w + s*s + t*t);
    return (u / norm, v / norm, w / norm);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_add() {
        let a = Vec3(1.0, 2.0, 3.0);
        let b = Vec3(-1.0, -2.0, -3.0);

        assert_eq!(a + b, Vec3(0.0, 0.0, 0.0));
    }

    #[test]
    fn vector_sub() {
        let a = Vec3(1.0, 2.0, 3.0);
        let b = Vec3(1.0, 2.0, 3.0);

        assert_eq!(a - b, Vec3(0.0, 0.0, 0.0));
    }

    #[test]
    fn vector_neg() {
        let a = Vec3(1.0, 2.0, 3.0);
        assert_eq!(-a, Vec3(-1.0, -2.0, -3.0));
    }

    #[test]
    fn vector_mul() {
        let a = Vec3(1.0, 2.0, 3.0);
        assert_eq!(5.0 * a, Vec3(5.0, 10.0, 15.0));
    }

    #[test]
    fn vector_norm() {
        let a = Vec3(1.0, 2.0, 3.0);
        assert_eq!(a.norm(), 14.0);
    }

    #[test]
    fn vector_len() {
        let a = Vec3(4.0, 4.0, 2.0);
        assert_eq!(a.len(), 6.0);
    }

    #[test]
    fn vector_dot() {
        let a = Vec3(4.0, 4.0, 2.0);
        assert_eq!(Vec3::dot(a, a), 36.0);
    }

    #[test]
    fn vector_normalize() {
        let a = Vec3(4.0, 4.0, 2.0);
        assert_eq!(a.normalize(), Vec3(4.0 / 6.0, 4.0 / 6.0, 2.0 / 6.0));
    }
}
