mod camera;
mod color;
mod hittable;
mod hittable_vec;
mod material;
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
use crate::material::{Dielectric, Lambertian, Metal};
use crate::point::Point3;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector::{Len, Vec3};
use rand::distributions::{Distribution, Uniform};
use std::rc::Rc;

fn ray_color(world: &HittableVec, r: &Ray, depth: u32) -> Color {
    if depth == 0 {
        return Color::zero();
    }
    if let Some((hit, mat)) = world.hit(r, 1e-3, f32::MAX) {
        if let Some((attenuation, scattered)) = mat.scatter(r, &hit) {
            attenuation * ray_color(world, &scattered, depth - 1)
        } else {
            Color::zero()
        }
    } else {
        let t = 0.5 * (r.dir.y() + 1.0);
        Color::lerp(Color(1.0, 1.0, 1.0), Color(0.5, 0.7, 1.0), t)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // image
    let samples_per_pixel = 100;
    let depth = 100;
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    let look_from = Point3(3.0, 3.0, 2.0);
    let look_at = Point3(0.0, 0.0, -1.0);
    let vup = Vec3(0.0, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).len();
    let aperture = 2.0;
    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // render
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("image.ppm")?;

    file.write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())?;
    // from image_height - 1 to 0

    let material_ground = Rc::new(Lambertian::new(Color(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color(0.8, 0.6, 0.2), 0.0));

    let mut world = HittableVec::new();
    world.push(Box::new(Sphere::new(
        Point3(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.push(Box::new(Sphere::new(
        Point3(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.push(Box::new(Sphere::new(
        Point3(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.push(Box::new(Sphere::new(
        Point3(-1.0, 0.0, -1.0),
        -0.45,
        material_left,
    )));
    world.push(Box::new(Sphere::new(
        Point3(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

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

                pixel_color += ray_color(&world, &r, depth);
            }

            // Divide the color by the number of samples and gamma-correct for gamma=2.0.
            pixel_color /= samples_per_pixel as f32;
            let ir = (256.0 * f32::clamp(f32::sqrt(pixel_color.r()), 0.0, 0.999)) as u8;
            let ig = (256.0 * f32::clamp(f32::sqrt(pixel_color.g()), 0.0, 0.999)) as u8;
            let ib = (256.0 * f32::clamp(f32::sqrt(pixel_color.b()), 0.0, 0.999)) as u8;
            file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
        }
    }
    Ok(())
}
