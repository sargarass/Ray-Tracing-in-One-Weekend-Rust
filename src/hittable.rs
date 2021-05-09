use crate::material::Material;
use crate::point::Point3;
use crate::ray::Ray;
use crate::vector::{Dot, Normalize, Vec3};

pub struct Hit {
    p: Point3,
    n: Vec3,
    t: f32,
    front_face: bool,
}

impl Hit {
    pub fn new(ray_dir: Vec3, hit_point: Point3, outward_normal: Vec3, t: f32) -> Self {
        assert!(
            Vec3::almost_eq(outward_normal.normalize(), outward_normal, 1e-5),
            "n must be a unit vector"
        );

        let front_face = Vec3::dot(ray_dir, outward_normal) < 0.0;
        let n = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            p: hit_point,
            n,
            t,
            front_face,
        }
    }

    pub fn p(&self) -> Point3 {
        self.p
    }

    pub fn n(&self) -> Vec3 {
        self.n
    }

    pub fn t(&self) -> f32 {
        self.t
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(Hit, &Material)>;
}
