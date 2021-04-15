use crate::hittable::{Hit, Hittable};
use crate::point::Point3;
use crate::ray::Ray;
use crate::vector::{Dot, Norm, Vec3};

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = ray.orig - self.center;
        let a = Vec3::norm(ray.dir);
        let half_b = Vec3::dot(oc, ray.dir);
        let c = Vec3::norm(oc) - self.radius * self.radius;
        // computing a discriminant
        #[allow(clippy::suspicious_operation_groupings)]
        // suspend lint for the operation: was triggered by half_b * half_b
        let d = half_b * half_b - a * c;
        if d < 0.0 {
            return None;
        }

        // Find the nearest root that lies in the acceptable range.
        let sqrt_d = d.sqrt();
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = ray.at(root);
        Some(Hit {
            t: root,
            p,
            n: (p - self.center) / self.radius,
        })
    }
}
