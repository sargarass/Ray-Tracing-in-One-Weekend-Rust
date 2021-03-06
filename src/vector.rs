use rand::distributions::Distribution;
use rand::Rng;
use rand_distr::StandardNormal;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Dot: Sized + Copy {
    fn dot(self, w: Self) -> f32;
}

pub trait Len: Sized + Copy + Dot {
    fn len(self) -> f32 {
        self.len_squared().sqrt()
    }
    fn len_squared(self) -> f32 {
        self.dot(self)
    }
}

pub trait Normalize: Sized + Copy + Len + Dot + std::ops::Div<f32, Output = Self> {
    fn normalize(self) -> Self {
        self.div(self.len())
    }
}

pub trait Cross: Sized + Copy {
    fn cross(self, v: Self) -> Self;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl From<Vec3> for (f32, f32, f32) {
    fn from(v: Vec3) -> Self {
        (v.0, v.1, v.2)
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from(v: (f32, f32, f32)) -> Vec3 {
        Vec3(v.0, v.1, v.2)
    }
}

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

    pub fn almost_eq(lsh: Vec3, rhs: Vec3, eps: f32) -> bool {
        Vec3::len(lsh - rhs) < eps
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

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(mut self, v: Vec3) -> Vec3 {
        self += v;
        self
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(mut self, v: Vec3) -> Vec3 {
        self -= v;
        self
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, mut rhs: Vec3) -> Vec3 {
        rhs *= self;
        rhs
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(mut self, r: f32) -> Vec3 {
        self *= r.recip();
        self
    }
}

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

#[allow(clippy::many_single_char_names)]
pub fn uniform_in_unit_sphere<R: Rng>(rng: &mut R) -> (f32, f32, f32) {
    // Let d = 5
    // Compute d random numbers with the Normal Distribution
    let distribution = StandardNormal;
    let u: f32 = distribution.sample(rng);
    let v: f32 = distribution.sample(rng);
    let w: f32 = distribution.sample(rng);
    let s: f32 = distribution.sample(rng);
    let t: f32 = distribution.sample(rng);
    // According to Muller, Marsaglia there is a relationship between a d-ball and the Normal Distribution:
    // let u, ... t ~ N(0,1) aka has the Normal Distribution, and l = sqrt(u^2 + ... t^2)
    // then (u / l, ..., t / l) is uniformly distributed on a unit d-ball

    // inv_l = 1 / l
    let inv_l = f32::recip(f32::sqrt(u * u + v * v + w * w + s * s + t * t));

    // Now using method 9: Dropped Coordinates (proved by Voelker)
    // http://extremelearning.com.au/how-to-generate-uniformly-random-points-on-n-spheres-and-n-balls/
    // for uniformity inside (d-2) unit ball just drop last 2 coordinates
    (inv_l * Vec3(u, v, w)).into()
}

pub fn uniform_on_unit_sphere<R: Rng>(rng: &mut R) -> (f32, f32, f32) {
    // Use the same trick as in fn uniform_in_unit_sphere
    let distribution = StandardNormal;
    let u = distribution.sample(rng);
    let v = distribution.sample(rng);
    let w = distribution.sample(rng);
    let inv_l = f32::recip(f32::sqrt(u * u + v * v + w * w));

    // (u * inv_l, v * inv_l, w * inv_l) is uniformly distributed on the unit sphere
    (inv_l * Vec3(u, v, w)).into()
}

pub fn uniform_in_unit_disk<R: Rng>(rng: &mut R) -> (f32, f32) {
    // Use the same trick as in fn uniform_in_unit_sphere
    let distribution = StandardNormal;
    let u: f32 = distribution.sample(rng);
    let v: f32 = distribution.sample(rng);
    let w: f32 = distribution.sample(rng);
    let s: f32 = distribution.sample(rng);
    let inv_l = f32::recip(f32::sqrt(u * u + v * v + w * w + s * s));

    // (u * inv_l, v * inv_l) is uniformly distributed in the disk
    (inv_l * u, inv_l * v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_zero() {
        let a = Vec3::zero();
        assert_eq!(a.0, 0.0);
        assert_eq!(a.1, 0.0);
        assert_eq!(a.2, 0.0);
    }

    #[test]
    fn vec3_from_f32_tuple() {
        let a = Vec3::from((1.0, 2.0, 3.0));
        assert_eq!(a.0, 1.0);
        assert_eq!(a.1, 2.0);
        assert_eq!(a.2, 3.0);
    }

    #[test]
    fn vec3_into_f32_tuple() {
        let a: (f32, f32, f32) = Vec3(1.0, 2.0, 3.0).into();
        assert_eq!(a.0, 1.0);
        assert_eq!(a.1, 2.0);
        assert_eq!(a.2, 3.0);
    }

    #[test]
    fn vec3_xyz() {
        let a = Vec3(1.0, 2.0, 3.0);
        assert_eq!(a.x(), a.0);
        assert_eq!(a.y(), a.1);
        assert_eq!(a.z(), a.2);
    }

    #[test]
    fn vec3_add() {
        let a = Vec3(1.0, 2.0, 3.0);
        let b = Vec3(-1.0, -2.0, -3.0);

        assert_eq!(a + b, Vec3(0.0, 0.0, 0.0));
    }

    #[test]
    fn vec3_div() {
        let a = Vec3(4.0, 2.0, 6.0);

        assert_eq!(a / 2.0, Vec3(2.0, 1.0, 3.0));
    }

    #[test]
    fn vec3_sub() {
        let a = Vec3(1.0, 2.0, 3.0);
        let b = Vec3(1.0, 2.0, 3.0);

        assert_eq!(a - b, Vec3(0.0, 0.0, 0.0));
    }

    #[test]
    fn vec3_neg() {
        let a = Vec3(1.0, 2.0, 3.0);
        assert_eq!(-a, Vec3(-1.0, -2.0, -3.0));
    }

    #[test]
    fn vec3_mul() {
        let a = Vec3(1.0, 2.0, 3.0);
        assert_eq!(5.0 * a, Vec3(5.0, 10.0, 15.0));
    }

    #[test]
    fn vec3_len_squared() {
        let a = Vec3(1.0, 2.0, 3.0);
        assert_eq!(a.len_squared(), 14.0);
    }

    #[test]
    fn vec3_len() {
        let a = Vec3(4.0, 4.0, 2.0);
        assert_eq!(a.len(), 6.0);
    }

    #[test]
    fn vec3_dot() {
        let a = Vec3(4.0, 4.0, 2.0);
        assert_eq!(Vec3::dot(a, a), 36.0);
    }

    #[test]
    fn vec3_normalize() {
        let a = Vec3(4.0, 4.0, 2.0);
        assert_eq!(a.normalize(), Vec3(4.0 / 6.0, 4.0 / 6.0, 2.0 / 6.0));
    }

    #[test]
    fn vec3_cross() {
        let a = Vec3(1.0, 0.0, 0.0);
        let b = Vec3(0.0, 1.0, 0.0);
        let c = Vec3(0.0, 0.0, 1.0);

        assert_eq!(Vec3::cross(a, b), c);
        assert_eq!(Vec3::cross(b, c), a);
        assert_eq!(Vec3::cross(c, a), b);

        assert_eq!(Vec3::cross(b, a), -c);
        assert_eq!(Vec3::cross(a, c), -b);
        assert_eq!(Vec3::cross(c, b), -a);

    }
}
