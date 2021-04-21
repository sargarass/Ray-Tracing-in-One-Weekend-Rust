use crate::vector::Vec3;
use std::ops::{Add, Neg, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point3(pub f32, pub f32, pub f32);

impl Point3 {
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
        Self(0.0, 0.0, 0.0)
    }
}

impl Neg for Point3 {
    type Output = Point3;
    fn neg(self) -> Point3 {
        Point3(-self.0, -self.1, -self.2)
    }
}

impl Add<Vec3> for Point3 {
    type Output = Point3;
    fn add(self, p: Vec3) -> Point3 {
        Point3(self.0 + p.0, self.1 + p.1, self.2 + p.2)
    }
}

impl Add<Point3> for Vec3 {
    type Output = Point3;
    fn add(self, p: Point3) -> Point3 {
        Point3(self.0 + p.0, self.1 + p.1, self.2 + p.2)
    }
}

impl Sub<Point3> for Point3 {
    type Output = Vec3;
    fn sub(self, p: Point3) -> Vec3 {
        Vec3(self.0 - p.0, self.1 - p.1, self.2 - p.2)
    }
}

impl Sub<Vec3> for Point3 {
    type Output = Point3;
    fn sub(self, p: Vec3) -> Point3 {
        Point3(self.0 - p.0, self.1 - p.1, self.2 - p.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point3_add() {
        let a = Point3(1.0, 1.0, 1.0);
        let m = Vec3(1.0, 1.0, 1.0);
        assert_eq!(a + m, Point3(2.0, 2.0, 2.0));
        assert_eq!(m + a, Point3(2.0, 2.0, 2.0));
    }

    #[test]
    fn point3_sub() {
        let a = Point3(1.0, 1.0, 1.0);
        let b = Point3(2.0, 3.0, 4.0);

        assert_eq!(b - a, Vec3(1.0, 2.0, 3.0));
        assert_eq!(a - b, Vec3(-1.0, -2.0, -3.0));
    }
}
