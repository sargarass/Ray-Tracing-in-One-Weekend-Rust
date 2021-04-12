use crate::point::Point3;
use crate::vector::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub fn point_at_param(self, t: f32) -> Point3 {
        self.orig + t * self.dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_at_param() {
        let t = Ray::new(Point3::new(-1.0, 1.0, -1.0), Vec3::new(1.0, -1.0, 1.0));
        assert_eq!(t.point_at_param(1.0), Point3::new(0.0, 0.0, 0.0));
        assert_eq!(t.point_at_param(2.0), Point3::new(1.0, -1.0, 1.0));
    }
}