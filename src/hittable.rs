use crate::point::Point3;
use crate::ray::Ray;
use crate::vector::Vec3;

pub struct Hit {
    pub p: Point3,
    pub n: Vec3,
    pub t: f32,
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}
