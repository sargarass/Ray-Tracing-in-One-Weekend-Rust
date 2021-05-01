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
use rand::Rng;
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

fn random_scene() -> hittable_vec::HittableVec {
    let mut world = HittableVec::new();

    world.push(Box::new(Sphere::new(
        Point3(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(Color(0.5, 0.5, 0.5))),
    )));

    let mut rng = rand::thread_rng();
    let distribution = rand::distributions::Standard;
    for a in -11..11 {
        for b in -11..11 {
            let a = a as f32;
            let b = b as f32;
            let choose_mat: f32 = distribution.sample(&mut rng);
            let (dx, dy): (f32, f32) =
                (distribution.sample(&mut rng), distribution.sample(&mut rng));
            let center = Point3(a + 0.9 * dx, 0.2, b + 0.9 * dy);
            if (center - Point3(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(&mut rng) * Color::random(&mut rng);
                    let material = Rc::new(Lambertian::new(albedo));
                    world.push(Box::new(Sphere::new(center, 0.2, material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_minmax(&mut rng, 0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let material = Rc::new(Metal::new(albedo, fuzz));
                    world.push(Box::new(Sphere::new(center, 0.2, material)));
                } else {
                    // glass
                    let material = Rc::new(Dielectric::new(1.5));
                    world.push(Box::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    world.push(Box::new(Sphere::new(
        Point3(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dielectric::new(1.5)),
    )));
    world.push(Box::new(Sphere::new(
        Point3(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian::new(Color(0.4, 0.2, 0.1))),
    )));
    world.push(Box::new(Sphere::new(
        Point3(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(Color(0.7, 0.6, 0.5), 0.0)),
    )));
    world
}

fn main() -> Result<(), Box<dyn Error>> {
    // image
    let samples_per_pixel = 500;
    let depth = 50;
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    let look_from = Point3(13.0, 2.0, 3.0);
    let look_at = Point3(0.0, 0.0, 0.0);
    let vup = Vec3(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
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

    let world = random_scene();
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

                let r = camera.get_ray(&mut rng,u, v);

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
