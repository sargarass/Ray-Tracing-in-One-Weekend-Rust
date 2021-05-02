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
use std::io::BufWriter;
use std::io::Write;
use std::{fs, thread};

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
use rand::{Rng, RngCore};
use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

fn ray_color(world: &HittableVec, mut ray: Ray, max_depth: u32) -> Color {
    let mut intensity = Color(1.0, 1.0, 1.0);
    for _depth in 0..max_depth {
        if let Some((hit, mat)) = world.hit(&ray, 1e-3, f32::MAX) {
            if let Some((attenuation, scattered)) = mat.scatter(&ray, &hit) {
                ray = scattered;
                intensity *= attenuation;
                continue;
            }
            break;
        }
        let t = 0.5 * (ray.dir.y() + 1.0);
        return intensity * Color::lerp(Color(1.0, 1.0, 1.0), Color(0.5, 0.7, 1.0), t);
    }
    Color::zero()
}

fn random_scene() -> hittable_vec::HittableVec {
    let mut world = HittableVec::new();
    world.push(Box::new(Sphere::new(
        Point3(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Color(0.5, 0.5, 0.5))),
    )));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let a = a as f32;
            let b = b as f32;
            let choose_mat: f32 = rng.gen();
            let (dx, dy): (f32, f32) = (rng.gen(), rng.gen());
            let center = Point3(a + 0.9 * dx, 0.2, b + 0.9 * dy);
            if (center - Point3(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(&mut rng) * Color::random(&mut rng);
                    let material = Arc::new(Lambertian::new(albedo));
                    world.push(Box::new(Sphere::new(center, 0.2, material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_minmax(&mut rng, 0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let material = Arc::new(Metal::new(albedo, fuzz));
                    world.push(Box::new(Sphere::new(center, 0.2, material)));
                } else {
                    // glass
                    let material = Arc::new(Dielectric::new(1.5));
                    world.push(Box::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    world.push(Box::new(Sphere::new(
        Point3(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.push(Box::new(Sphere::new(
        Point3(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(Color(0.4, 0.2, 0.1))),
    )));
    world.push(Box::new(Sphere::new(
        Point3(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Color(0.7, 0.6, 0.5), 0.0)),
    )));
    world
}

#[derive(Default)]
struct Metrics {
    pub pixels_count: AtomicU64,
    pub rays_count: AtomicU64,
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
    // from image_height - 1 to 0
    let world = random_scene();

    let metrics = Arc::new(Box::new(Metrics::default()));
    {
        let metrics = metrics.clone();
        let start = Instant::now();
        let mut prev_time_point = start;
        thread::spawn(move || {
            let mut prev_pixels_count = 0;
            let mut prev_rays_count = 0;

            loop {
                thread::sleep(Duration::from_secs(1));

                let time_point = Instant::now();
                let since_prev_time_point =
                    time_point.duration_since(prev_time_point).as_secs_f32();
                let since_start = time_point.duration_since(start).as_secs_f32();

                let pixels_count = metrics.pixels_count.load(Ordering::Relaxed);
                let rays_count = metrics.rays_count.load(Ordering::Relaxed);

                let pixels_per_sec =
                    (pixels_count - prev_pixels_count) as f32 / since_prev_time_point;
                let rays_per_sec = (rays_count - prev_rays_count) as f32 / since_prev_time_point;

                let complete_percentages = rays_count as f32
                    / (samples_per_pixel * image_width * image_height) as f32
                    * 100.0;
                println!(
                    "Rays/Sec {:.2} Pixels/Sec {:.2} Complete {:.2}% Took {:.2}s Estimated {:.2}s",
                    rays_per_sec,
                    pixels_per_sec,
                    complete_percentages,
                    since_start,
                    since_start / complete_percentages * 100.0 - since_start
                );

                prev_pixels_count = pixels_count;
                prev_rays_count = rays_count;
                prev_time_point = time_point;
            }
        });
    }

    let image: Vec<Vec<_>> = (0..image_height)
        .into_par_iter()
        .rev()
        .map(|j| {
            (0..image_width)
                .map(|i| {
                    let mut rng = rand::thread_rng();
                    let distribution = Uniform::from(-0.5..=0.5);
                    let mut pixel: Color = (0..samples_per_pixel)
                        .map(|_| {
                            let di = distribution.sample(&mut rng);
                            let dj = distribution.sample(&mut rng);
                            let u = (i as f32 + di) / (image_width - 1) as f32;
                            let v = (j as f32 + dj) / (image_height - 1) as f32;
                            let r = camera.get_ray(&mut rng, u, v);
                            let c = ray_color(&world, r, depth);
                            metrics.rays_count.fetch_add(1, Ordering::Relaxed);
                            c
                        })
                        .sum();
                    metrics.pixels_count.fetch_add(1, Ordering::Relaxed);
                    pixel /= samples_per_pixel as f32;
                    pixel
                })
                .collect()
        })
        .collect();

    let file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("image.ppm")?;
    let mut file = BufWriter::new(file);
    file.write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())?;
    for pixel in image.iter().flatten() {
        let ir = (256.0 * f32::clamp(f32::sqrt(pixel.r()), 0.0, 0.999)) as u8;
        let ig = (256.0 * f32::clamp(f32::sqrt(pixel.g()), 0.0, 0.999)) as u8;
        let ib = (256.0 * f32::clamp(f32::sqrt(pixel.b()), 0.0, 0.999)) as u8;
        file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())?
    }
    file.flush()?;
    Ok(())
}
