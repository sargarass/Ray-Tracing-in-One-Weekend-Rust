use crate::point::Point3;
use crate::ray::Ray;
use crate::vector::{uniform_in_unit_disk, Cross, Normalize, Vec3};
use rand::Rng;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        vfov: f32,
        aspect_ration: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        let theta = f32::to_radians(vfov);
        let h = f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ration * viewport_height;

        let w = Vec3::normalize(look_from - look_at);
        let u = Vec3::normalize(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
        let lens_radius = aperture / 2.0;
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray<R: Rng>(&self, rng: &mut R, s: f32, t: f32) -> Ray {
        let (x, y) = uniform_in_unit_disk(rng);
        let offset = self.lens_radius * (x * self.u + y * self.v);
        let direction = Vec3::normalize(
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        );
        Ray::new(self.origin + offset, direction)
    }
}
