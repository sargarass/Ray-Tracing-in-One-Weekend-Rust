mod camera;
mod color;
mod hittable;
mod hittable_vec;
mod point;
mod ray;
mod sphere;
mod vector;

use std::error::Error;
use std::fs;
use std::io::Write;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::Hittable;
use crate::hittable_vec::HittableVec;
use crate::point::Point3;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector::{Normalize, Vec3, random_in_unit_circle};
use rand::distributions::{Distribution, Uniform};

fn ray_color(world: &HittableVec, r: &Ray) -> Color {
    match world.hit(*r, 0.000001, f32::MAX) {
        Some(hit) => {
            let n = hit.n;
            0.5 * Color(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0)
        }
        None => {
            let unit_dir = Vec3::normalize(r.dir);
            let t = 0.5 * (unit_dir.y() + 1.0);
            Color::lerp(Color(1.0, 1.0, 1.0), Color(0.5, 0.7, 1.0), t)
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // image
    let samples_per_pixel = 25;
    let aspect_ratio = 21.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    let camera = Camera::new(
        Point3(0.0, 0.0, 0.0),
        Point3(0.0, 0.0, -1.0),
        Vec3(0.0, 1.0, 0.0),
        90.0,
        aspect_ratio,
    );

    // render
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("image.ppm")?;

    file.write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())?;
    // from image_height - 1 to 0

    let mut world = HittableVec::new();
    world.push(Box::new(Sphere::new(Point3(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3(0.0, -100.5, -1.0), 100.0)));

    let mut rng = rand::thread_rng();
    let distribution = Uniform::from(-0.5..=0.5);
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Color::zero();
            for _ in 0..samples_per_pixel {
                let di = distribution.sample(&mut rng);
                let dj = distribution.sample(&mut rng);
                let u = (i as f32 + di) / (image_width - 1) as f32;
                let v = (j as f32 + dj) / (image_height - 1) as f32;

                let r = camera.get_ray(u, v);

                pixel_color += ray_color(&world, &r);
            }

            pixel_color = pixel_color / samples_per_pixel as f32;

            let ir = (255.999 * f32::clamp(pixel_color.r(), 0.0, 0.999)) as u8;
            let ig = (255.999 * f32::clamp(pixel_color.g(), 0.0, 0.999)) as u8;
            let ib = (255.999 * f32::clamp(pixel_color.b(), 0.0, 0.999)) as u8;
            file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
        }
    }
    Ok(())
}
