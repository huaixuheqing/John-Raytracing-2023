pub mod basic;
mod color;
use basic::*;
use color::write_color;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::fs::File;

const AUTHOR: &str = "Kr.Cen";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn ray_color(r: &ray::Ray) -> vec3::Vec3 {
    let unit_direction = r.dir.to_unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    return vec3::Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + vec3::Vec3::new(0.5, 0.7, 1.0) * t;
}

fn main() {
    // get environment variable CI, which is true for GitHub Action
    let is_ci = is_ci();

    println!("CI: {}", is_ci);

    let aspect_ratio: f64 = 16.0 / 9.0;
    let width: usize = 800;
    let height: usize = ((width as f64) / aspect_ratio) as usize;
    let path = "output/book1_image2.jpg";
    let quality = 60; // From 0 to 100

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = vec3::Vec3::new(0.0, 0.0, 0.0);
    let horizontal = vec3::Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = vec3::Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - vec3::Vec3::new(0.0, 0.0, focal_length);

    // Create image data
    let mut img: RgbImage = ImageBuffer::new(width.try_into().unwrap(), height.try_into().unwrap());

    // Progress bar UI powered by library `indicatif`
    let bar = if is_ci {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    for j in 0..height {
        for i in 0..width {
            let u = i as f64 / (width as f64 - 1.0);
            let v = j as f64 / (height as f64 - 1.0);
            let r = ray::Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
                0.0,
            );
            let pixel_color = ray_color(&r);
            write_color(
                [
                    (pixel_color.x * 255.99) as u8,
                    (pixel_color.y * 255.99) as u8,
                    (pixel_color.z * 255.99) as u8,
                ],
                &mut img,
                i,
                height - j - 1,
            );
            bar.inc(1);
        }
    }

    bar.finish();
    // Output image to file
    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}
