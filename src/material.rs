use crate::color::Color;
use crate::hittable::Hit;
use crate::ray::Ray;
use crate::vector::{uniform_in_unit_sphere, uniform_on_unit_sphere, Dot, Len, Normalize, Vec3};
use rand::thread_rng;

pub trait Scatterable: Sync + Send {
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
        let mut scatter_dir = hit.n() + uniform_on_unit_sphere(&mut thread_rng()).into();
        if scatter_dir.len() < 1e-7 {
            scatter_dir = hit.n();
        }

        let scattered = Ray::new(hit.p(), scatter_dir.normalize());
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

fn reflect(v: Vec3, un: Vec3) -> Vec3 {
    assert!(
        Vec3::almost_eq(un.normalize(), un, 1e-5),
        "un must be a unit vector"
    );
    assert!(Vec3::dot(v, un) <= 0.0, "v, un must be on a same side");

    v - 2.0 * Vec3::dot(v, un) * un
}

impl Scatterable for Metal {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        if Vec3::dot(r_in.dir, hit.n()) > 0.0 {
            return None;
        }

        let reflected = reflect(r_in.dir, hit.n());
        let scattered = Ray::new(
            hit.p(),
            Vec3::normalize(
                reflected + self.fuzz * Vec3::from(uniform_in_unit_sphere(&mut thread_rng())),
            ),
        );
        if Vec3::dot(scattered.dir, hit.n()) <= 0.0 {
            return None;
        }
        Some((self.albedo, scattered))
    }
}

pub struct Dielectric {
    index_of_refraction: f32,
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Self {
        Self {
            index_of_refraction,
        }
    }
}

// n - normalized normal
fn refract(uv: Vec3, un: Vec3, refraction_ratio: f32) -> Option<Vec3> {
    assert!(
        Vec3::almost_eq(un.normalize(), un, 1e-5),
        "un must be a unit vector"
    );
    assert!(Vec3::dot(uv, un) <= 0.0, "uv, un must be on same side");

    let cos_theta = f32::clamp(Vec3::dot(-uv, un), -1.0, 1.0);
    let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);
    if refraction_ratio * sin_theta > 1.0 {
        return None;
    }

    let r_out_perp = refraction_ratio * (uv + cos_theta * un);
    let r_out_parallel = -f32::sqrt(f32::abs(1.0 - r_out_perp.len_squared())) * un;
    Some(r_out_perp + r_out_parallel)
}

fn reflectance(cosine: f32, index_of_refraction: f32) -> f32 {
    // Use Schlick's approximation for reflectance.
    let mut r0 = (1.0 - index_of_refraction) / (1.0 + index_of_refraction);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * f32::powi(1.0 - cosine, 5)
}

impl Scatterable for Dielectric {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let refraction_ratio = if hit.front_face() {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let attenuation = Color(1.0, 1.0, 1.0);
        let cos_theta = f32::clamp(Vec3::dot(-r_in.dir, hit.n()), -1.0, 1.0);

        let direction =
            // Depending on the refraction ratio, the light might not be able to refract, and instead reflects
            // Uses Schlick's approximation as the reflection varies with the angle.
            if rand::random::<f32>() < reflectance(cos_theta, refraction_ratio) {
                reflect(r_in.dir, hit.n())
            } else if let Some(refracted) = refract(r_in.dir, hit.n(), refraction_ratio) {
                refracted.normalize()
            } else {
                reflect(r_in.dir, hit.n())
            };
        Some((attenuation, Ray::new(hit.p(), direction)))
    }
}
