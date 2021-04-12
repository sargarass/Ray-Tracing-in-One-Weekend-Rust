use crate::vector::Vec3;
use std::ops::{Add, Neg, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3 {
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl Neg for Point3 {
    type Output = Point3;
    fn neg(self) -> Point3 {
        Point3::new(-self.x, -self.y, -self.z)
    }
}

// sum of points is vector
impl Add<Point3> for Point3 {
    type Output = Vec3;
    fn add(self, p: Point3) -> Vec3 {
        Vec3::new(self.x + p.x, self.y + p.y, self.z + p.z)
    }
}

// sum of points is vector
impl Add<Vec3> for Point3 {
    type Output = Point3;
    fn add(self, p: Vec3) -> Point3 {
        Point3::new(self.x + p.x, self.y + p.y, self.z + p.z)
    }
}

impl Add<Point3> for Vec3 {
    type Output = Point3;
    fn add(self, p: Point3) -> Point3 {
        Point3::new(self.x + p.x, self.y + p.y, self.z + p.z)
    }
}

impl Sub<Point3> for Point3 {
    type Output = Vec3;
    fn sub(self, p: Point3) -> Vec3 {
        Vec3::new(self.x - p.x, self.y - p.y, self.z - p.z)
    }
}

impl Sub<Vec3> for Point3 {
    type Output = Point3;
    fn sub(self, p: Vec3) -> Point3 {
        Point3::new(self.x - p.x, self.y - p.y, self.z - p.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point3_add() {
        let a = Point3::new(1.0, 1.0, 1.0);
        let b = Point3::new(2.0, 3.0, 4.0);

        assert_eq!(b + a, Vec3::new(3.0, 4.0, 5.0));
        assert_eq!(a + b, Vec3::new(3.0, 4.0, 5.0));

        let m = Vec3::new(1.0, 1.0, 1.0);

        assert_eq!(a + m, Point3::new(2.0, 2.0, 2.0));
        assert_eq!(m + a, Point3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn point3_sub() {
        let a = Point3::new(1.0, 1.0, 1.0);
        let b = Point3::new(2.0, 3.0, 4.0);

        assert_eq!(b - a, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(a - b, Vec3::new(-1.0, -2.0, -3.0));
    }
}
