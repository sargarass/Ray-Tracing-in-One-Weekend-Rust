use std::fs;
use std::error::Error;
use std::io::Write;
use std::borrow::Borrow;

fn main() -> Result<(), Box<dyn Error>> {
    let image_width = 256;
    let image_height = 256;

    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("image.ppm")?;

    file.write(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes());
    // from image_height - 1 to 0
    for j in (0..image_height).rev() {
        for i in (0..image_height) {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.25;
            let ir = (255.999 * r) as i8;
            let ig = (255.999 * g) as i8;
            let ib = (255.999 * b) as i8;

            file.write(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
        }
    }
    Ok(())
}
