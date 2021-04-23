use crate::color::Color;
use crate::hittable::Hit;
use crate::ray::Ray;
use crate::vector::{uniform_on_unit_sphere, Dot, Len, Normalize, Vec3, uniform_in_unit_sphere};
use rand::thread_rng;

pub trait Scatterable {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, _: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let mut scatter_dir = hit.n + uniform_on_unit_sphere(&mut rand::thread_rng()).into();
        if scatter_dir.len() < 1e-7 {
            scatter_dir = hit.n;
        }

        let scattered = Ray::new(hit.p, scatter_dir);
        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    debug_assert!(n.len() < 1.0 + 1e-5, "n should be a unit vector");
    v - 2.0 * Vec3::dot(v, n) * n
}

impl Scatterable for Metal {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let reflected = reflect(Vec3::normalize(r_in.dir), hit.n);
        let scattered = Ray::new(hit.p, reflected + self.fuzz * Vec3::from(uniform_in_unit_sphere(&mut thread_rng())));
        if Vec3::dot(scattered.dir, hit.n) <= 0.0 {
            return None;
        }
        Some((self.albedo, scattered))
    }
}
