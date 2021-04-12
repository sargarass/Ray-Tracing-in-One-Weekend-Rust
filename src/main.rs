mod color;
mod point;
mod ray;
mod vector;

use crate::color::Color;
use crate::point::Point3;
use crate::ray::Ray;
use crate::vector::{Normalize, Vec3};
use std::error::Error;
use std::fs;
use std::io::Write;

fn ray_color(r: &ray::Ray) -> Color {
    let unit_dir = Vec3::normalize(r.dir);
    let t = 0.5 * (unit_dir.y + 1.0);
    Color::lerp(Color::new(1.0, 1.0, 1.0), Color::new(0.5, 0.7, 1.0), t)
}

fn main() -> Result<(), Box<dyn Error>> {
    // image
    let aspect_ratio = 21.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    // camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("{} {} {}", lower_left_corner.x, lower_left_corner.y, lower_left_corner.z);

    // render
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("image.ppm")?;

    file.write(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())?;
    // from image_height - 1 to 0
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f32 / (image_width - 1) as f32;
            let v = j as f32 / (image_height - 1) as f32;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let col = ray_color(&r);
            let ir = (255.999 * col.r) as u8;
            let ig = (255.999 * col.g) as u8;
            let ib = (255.999 * col.b) as u8;
            file.write(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
        }
    }
    Ok(())
}
