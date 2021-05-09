use crate::hittable::{Hit, Hittable};
use crate::material::Material;
use crate::point::Point3;
use crate::ray::Ray;
use crate::vector::{Dot, Len, Normalize, Vec3};

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(Hit, &Material)> {
        let oc = ray.orig - self.center;
        let a = Vec3::len_squared(ray.dir);
        let half_b = Vec3::dot(oc, ray.dir);
        let c = Vec3::len_squared(oc) - self.radius * self.radius;
        // computing a discriminant
        #[allow(clippy::suspicious_operation_groupings)]
        // suspend lint for the operation: was triggered by half_b * half_b
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // Find the nearest root that lies in the acceptable range.
        let sqrt_d = discriminant.sqrt();
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = ray.at(root);
        Some((
            Hit::new(
                ray.dir,
                p,
                Vec3::normalize((p - self.center) / self.radius),
                root,
            ),
            &self.material,
        ))
    }
}
