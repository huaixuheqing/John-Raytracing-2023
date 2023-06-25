const infinity: f64 = f64::INFINITY;
const pi: f64 = 3.1415926535897932385;

mod Camera;
mod Hittable;
mod Hittable_list;
mod Material;
mod Ray;
mod Sphere;
mod color;
mod rtweekend;
mod vec3;

pub use crate::rtweekend::random_f64;
use crate::rtweekend::random_f64_1;
use crate::Sphere::sphere;
use color::write_color;
use image::error::UnsupportedErrorKind::Color;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
pub use rtweekend::degrees_to_radians;
use std::borrow::Borrow;
use std::fs::File;
use std::ops::Mul;
pub use std::sync::Arc;
pub use std::vec;
pub use vec3::Color1;
pub use vec3::Point3;
pub use vec3::Vec3;
pub use Camera::camera;
pub use Hittable::hit_record;
pub use Hittable::hittable;
pub use Hittable_list::hittable_list;
pub use Material::dielectric;
pub use Material::lambertian;
pub use Material::material;
pub use Material::medal;
pub use Ray::ray;

const AUTHOR: &str = "Siyuan Huang";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn ray_color(r: &ray, world: &hittable_list, depth: i32) -> Vec3 {
    let mut rec = hit_record::new();
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if world.hit(r, 0.001, infinity, &mut rec) {
        let mut scattered = ray::new1();
        let mut attenuation = Color1::new(0.0, 0.0, 0.0);
        if rec
            .mat_ptr
            .clone()
            .unwrap()
            .scatter(r, &mut rec, &mut attenuation, &mut scattered)
        {
            return ray_color(&scattered, &world, depth - 1).elemul(attenuation);
        }
        return Color1::new(0.0, 0.0, 0.0);
        //let mut target = rec.p.clone() + rec.normal.clone() + Vec3::random_in_hemisphere(&rec.normal);
        //return ray_color(&ray::new(rec.p.clone(),target - rec.p.clone()), &world, depth - 1) * 0.5;
    }
    let mut unit_direction = r.dir.unit_vector();
    let mut t = 0.5 * (unit_direction.y() + 1.0);
    Color1::new(1.0, 1.0, 1.0) * (1.0 - t) + Color1::new(0.5, 0.7, 1.0) * t
}

fn random_scene() -> hittable_list {
    let mut world = hittable_list::new();

    let mut ground_material: Option<Arc<dyn material>> =
        Some(Arc::new(lambertian::new(&Color1::new(0.5, 0.5, 0.5))));
    world.add(Some(Arc::new(sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Point3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            if (center.clone() - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let mut sphere_material: Option<Arc<dyn material>>;

                if choose_mat < 0.8 {
                    let albedo = Color1::random().elemul(Color1::random());
                    sphere_material = Some(Arc::new(lambertian::new(&albedo)));
                    world.add(Some(Arc::new(sphere::new(center, 0.2, sphere_material))));
                } else if choose_mat < 0.95 {
                    let albedo = Color1::random1(0.5, 1.0);
                    let fuzz = random_f64_1(0.0, 0.5);
                    sphere_material = Some(Arc::new(medal::new(&albedo, fuzz)));
                    world.add(Some(Arc::new(sphere::new(center, 0.2, sphere_material))));
                } else {
                    sphere_material = Some(Arc::new(dielectric::new(1.5)));
                    world.add(Some(Arc::new(sphere::new(center, 0.2, sphere_material))));
                }
            }
        }
    }
    let material1: Option<Arc<dyn material>> = Some(Arc::new(dielectric::new(1.5)));
    world.add(Some(Arc::new(sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    ))));

    let material2: Option<Arc<dyn material>> =
        Some(Arc::new(lambertian::new(&Color1::new(0.4, 0.2, 0.1))));
    world.add(Some(Arc::new(sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    ))));

    let material3: Option<Arc<dyn material>> =
        Some(Arc::new(medal::new(&Color1::new(0.7, 0.6, 0.5), 0.0)));
    world.add(Some(Arc::new(sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    ))));

    return world;
}

fn main() {
    // get environment variable CI, which is true for GitHub Actions
    let is_ci = is_ci();

    println!("CI: {}", is_ci);

    // Image
    let aspect_ratio = 3.0 / 2.0;
    let width: usize = 1200;
    let height = (width as f64 / aspect_ratio) as usize;
    let path = "output/test.jpg";
    let quality = 60; // From 0 to 100, suggested value: 60
    let samples_per_pixel = 10;
    let max_depth = 20;

    // Create image data
    let mut img: RgbImage = ImageBuffer::new(width.try_into().unwrap(), height.try_into().unwrap());

    // Progress bar UI powered by library `indicatif`
    // You can use indicatif::ProgressStyle to make it more beautiful
    // You can also use indicatif::MultiProgress in multi-threading to show progress of each thread
    let bar = if is_ci {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    // World
    let mut world = random_scene();

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = camera::new(
        &lookfrom,
        &lookat,
        &vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    for j in 0..height {
        for i in 0..width {
            let mut pixel_color = Color1::new(0.0, 0.0, 0.0);
            for s in 0..samples_per_pixel {
                let mut u = (i as f64 + random_f64()) / (width - 1) as f64;
                let mut v = (j as f64 + random_f64()) / (height - 1) as f64;
                let mut r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            write_color(&pixel_color, &mut img, i, height - j - 1, samples_per_pixel);
            bar.inc(1);
        }
    }

    // Finish progress bar
    bar.finish();

    // Output image to file
    println!("Output image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}
