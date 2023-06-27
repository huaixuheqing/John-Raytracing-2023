const INFINITY: f64 = f64::INFINITY;

mod aabb;
mod aarect;
mod r#box;
mod bvh;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod moving_sphere;
mod perlin;
mod ray;
mod rtweekend;
mod sphere;
mod texture;
mod vec3;
mod constant_medium;

pub use crate::rtweekend::random_f64;
use crate::rtweekend::random_f64_1;
use crate::sphere::Sphere;
use color::write_color;

use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
pub use rtweekend::degrees_to_radians;

use std::fs::File;

use crate::aarect::{XyRect, XzRect, YzRect};
use crate::hittable::{RotateY, Translate};
use crate::material::DiffuseLight;
use crate::r#box::Box1;
use crate::texture::ImageTecture;
pub use camera::Camera;

pub use hittable::HitRecord;
pub use hittable::Hittable;
pub use hittable_list::HittableList;
pub use material::Dielectric;
pub use material::Lambertian;
pub use material::Material;
pub use material::Medal;
pub use moving_sphere::MovingSphere;
pub use ray::Ray;
pub use std::sync::Arc;
pub use std::vec;
use console::Alignment::Center;
pub use texture::CheckerTexture;
pub use texture::NoiseTexture;
pub use texture::Texture;
pub use vec3::Color1;
pub use vec3::Point3;
pub use vec3::Vec3;
use crate::bvh::BvhNode;
use crate::constant_medium::ConstantMedium;

const AUTHOR: &str = "Siyuan Huang";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn ray_color(r: &Ray, background: &Color1, world: &HittableList, depth: i32) -> Vec3 {
    let mut rec = HitRecord::new();
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if !world.hit(r, 0.001, INFINITY, &mut rec) {
        return *background;
    }
    let mut scattered = Ray::new1();
    let mut attenuation = Color1::new(0.0, 0.0, 0.0);
    let emitted = rec.mat_ptr.clone().unwrap().emitted(rec.u, rec.v, &rec.p);
    if !rec
        .mat_ptr
        .clone()
        .unwrap()
        .scatter(r, &mut rec, &mut attenuation, &mut scattered)
    {
        return emitted;
    }
    emitted
        + Vec3::elemul(
            attenuation,
            ray_color(&scattered, background, world, depth - 1),
        )
    //let mut target = rec.p.clone() + rec.normal.clone() + Vec3::random_in_hemisphere(&rec.normal);
    //return ray_color(&Ray::new(rec.p.clone(),target - rec.p.clone()), &world, depth - 1) * 0.5;

    //let unit_direction = r.dir.unit_vector();
    //let t = 0.5 * (unit_direction.y() + 1.0);
    //Color1::new(1.0, 1.0, 1.0) * (1.0 - t) + Color1::new(0.5, 0.7, 1.0) * t
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let checker: Option<Arc<dyn Texture>> = Some(Arc::new(CheckerTexture::new1(
        Color1::new(0.2, 0.3, 0.1),
        Color1::new(0.9, 0.9, 0.9),
    )));
    world.add(Some(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Some(Arc::new(Lambertian::new1(checker))),
    ))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Point3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Option<Arc<dyn Material>>;

                if choose_mat < 0.8 {
                    let albedo = Color1::random().elemul(Color1::random());
                    sphere_material = Some(Arc::new(Lambertian::new(&albedo)));
                    let center2 = center + Vec3::new(0.0, random_f64_1(0.0, 0.5), 0.0);
                    world.add(Some(Arc::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    ))));
                } else if choose_mat < 0.95 {
                    let albedo = Color1::random1(0.5, 1.0);
                    let fuzz = random_f64_1(0.0, 0.5);
                    sphere_material = Some(Arc::new(Medal::new(&albedo, fuzz)));
                    world.add(Some(Arc::new(Sphere::new(center, 0.2, sphere_material))));
                } else {
                    sphere_material = Some(Arc::new(Dielectric::new(1.5)));
                    world.add(Some(Arc::new(Sphere::new(center, 0.2, sphere_material))));
                }
            }
        }
    }
    let material1: Option<Arc<dyn Material>> = Some(Arc::new(Dielectric::new(1.5)));
    world.add(Some(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    ))));

    let material2: Option<Arc<dyn Material>> =
        Some(Arc::new(Lambertian::new(&Color1::new(0.4, 0.2, 0.1))));
    world.add(Some(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    ))));

    let material3: Option<Arc<dyn Material>> =
        Some(Arc::new(Medal::new(&Color1::new(0.7, 0.6, 0.5), 0.0)));
    world.add(Some(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    ))));

    world
}

fn two_spheres() -> HittableList {
    let mut objects = HittableList::new();
    let checker: Option<Arc<dyn Texture>> = Some(Arc::new(CheckerTexture::new1(
        Color1::new(0.2, 0.3, 0.1),
        Color1::new(0.9, 0.9, 0.9),
    )));
    objects.add(Some(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Some(Arc::new(Lambertian::new1(checker.clone()))),
    ))));
    objects.add(Some(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Some(Arc::new(Lambertian::new1(checker))),
    ))));
    objects
}

fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new();

    let pertext: Option<Arc<dyn Texture>> = Some(Arc::new(NoiseTexture::new(4.0)));
    objects.add(Some(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Some(Arc::new(Lambertian::new1(pertext.clone()))),
    ))));
    objects.add(Some(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Some(Arc::new(Lambertian::new1(pertext))),
    ))));
    objects
}

fn earth() -> HittableList {
    let mut world = HittableList::new();
    let earth_texture: Option<Arc<dyn Texture>> = Some(Arc::new(ImageTecture::new("earthmap.jpg")));
    let earth_surface: Option<Arc<dyn Material>> = Some(Arc::new(Lambertian::new1(earth_texture)));
    let globe: Option<Arc<dyn Hittable>> = Some(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface,
    )));
    world.add(globe);

    world
}

fn simple_light() -> HittableList {
    let mut objects = HittableList::new();

    let pertext: Option<Arc<dyn Texture>> = Some(Arc::new(NoiseTexture::new(4.0)));
    objects.add(Some(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Some(Arc::new(Lambertian::new1(pertext.clone()))),
    ))));
    objects.add(Some(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Some(Arc::new(Lambertian::new1(pertext))),
    ))));

    let difflight: Option<Arc<dyn Material>> =
        Some(Arc::new(DiffuseLight::new1(Color1::new(4.0, 4.0, 4.0))));
    objects.add(Some(Arc::new(XyRect::new(
        3.0, 5.0, 1.0, 3.0, -2.0, difflight,
    ))));

    objects
}

fn cornell_box() -> HittableList {
    let mut objects = HittableList::new();

    let red: Option<Arc<dyn Material>> =
        Some(Arc::new(Lambertian::new(&Color1::new(0.65, 0.05, 0.05))));
    let white: Option<Arc<dyn Material>> =
        Some(Arc::new(Lambertian::new(&Color1::new(0.73, 0.73, 0.73))));
    let green: Option<Arc<dyn Material>> =
        Some(Arc::new(Lambertian::new(&Color1::new(0.12, 0.45, 0.15))));
    let light: Option<Arc<dyn Material>> =
        Some(Arc::new(DiffuseLight::new1(Color1::new(15.0, 15.0, 15.0))));

    objects.add(Some(Arc::new(YzRect::new(
        0.0, 555.0, 0.0, 555.0, 555.0, green,
    ))));
    objects.add(Some(Arc::new(YzRect::new(
        0.0, 555.0, 0.0, 555.0, 0.0, red,
    ))));
    objects.add(Some(Arc::new(XzRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    ))));
    objects.add(Some(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    ))));
    objects.add(Some(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    ))));
    objects.add(Some(Arc::new(XyRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    ))));

    let mut box1: Option<Arc<dyn Hittable>> = Some(Arc::new(Box1::new(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    )));
    box1 = Some(Arc::new(RotateY::new(box1, 15.0)));
    box1 = Some(Arc::new(Translate::new(
        box1,
        &Vec3::new(265.0, 0.0, 295.0),
    )));
    objects.add(box1);

    let mut box2: Option<Arc<dyn Hittable>> = Some(Arc::new(Box1::new(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    )));
    box2 = Some(Arc::new(RotateY::new(box2, -18.0)));
    box2 = Some(Arc::new(Translate::new(box2, &Vec3::new(130.0, 0.0, 65.0))));
    objects.add(box2);

    objects
}

fn cornell_smoke() -> HittableList {
    let mut objects = HittableList::new();

    let red:Option<Arc<dyn Material>> = Some(Arc::new(Lambertian::new(&Color1::new(0.65,0.05,0.05))));
    let white:Option<Arc<dyn Material>> = Some(Arc::new(Lambertian::new(&Color1::new(0.73,0.73,0.73))));
    let green:Option<Arc<dyn Material>> = Some(Arc::new(Lambertian::new(&Color1::new(0.12,0.45,0.15))));
    let light:Option<Arc<dyn Material>> = Some(Arc::new(DiffuseLight::new1(Color1::new(7.0,7.0,7.0))));

    objects.add(Some(Arc::new(YzRect::new(0.0,555.0,0.0,555.0,555.0,green.clone()))));
    objects.add(Some(Arc::new(YzRect::new(0.0,555.0,0.0,555.0,0.0,red.clone()))));
    objects.add(Some(Arc::new(XzRect::new(113.0,443.0,127.0,432.0,554.0,light.clone()))));
    objects.add(Some(Arc::new(XzRect::new(0.0,555.0,0.0,555.0,555.0,white.clone()))));
    objects.add(Some(Arc::new(XzRect::new(0.0,555.0,0.0,555.0,0.0,white.clone()))));
    objects.add(Some(Arc::new(XyRect::new(0.0,555.0,0.0,555.0,555.0,white.clone()))));
    let mut box1: Option<Arc<dyn Hittable>> = Some(Arc::new(Box1::new(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    )));
    box1 = Some(Arc::new(RotateY::new(box1, 15.0)));
    box1 = Some(Arc::new(Translate::new(
        box1,
        &Vec3::new(265.0, 0.0, 295.0),
    )));

    let mut box2: Option<Arc<dyn Hittable>> = Some(Arc::new(Box1::new(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    )));
    box2 = Some(Arc::new(RotateY::new(box2, -18.0)));
    box2 = Some(Arc::new(Translate::new(box2, &Vec3::new(130.0, 0.0, 65.0))));

    objects.add(Some(Arc::new(ConstantMedium::new1(box1, 0.01,Color1::new(0.0,0.0,0.0)))));
    objects.add(Some(Arc::new(ConstantMedium::new1(box2, 0.01,Color1::new(1.0,1.0,1.0)))));

    objects
}

fn final_scene() -> HittableList {
    let mut boxes1 = HittableList::new();
    let ground:Option<Arc<dyn Material>> = Some(Arc::new(Lambertian::new(&Color1::new(0.48,0.83,0.53))));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side{
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_f64_1(1.0,101.0);
            let z1 = z0 + w;

            boxes1.add(Some(Arc::new(Box1::new(&Point3::new(x0,y0,z0),&Point3::new(x1,y1,z1),ground.clone()))));
        }
    }

    let mut objects = HittableList::new();

    objects.add(Some(Arc::new(BvhNode::new1(&mut boxes1, 0.0,1.0))));

    let light:Option<Arc<dyn Material>> = Some(Arc::new(DiffuseLight::new1(Color1::new(7.0,7.0,7.0))));
    objects.add(Some(Arc::new(XzRect::new(123.0,423.0,147.0,412.0,554.0,light.clone()))));

    let center1 = Point3::new(400.0,400.0,200.0);
    let center2 = center1 + Vec3::new(30.0,0.0,0.0);
    let moving_sphere_material:Option<Arc<dyn Material>> = Some(Arc::new(Lambertian::new(&Color1::new(0.7,0.3,0.1))));
    objects.add(Some(Arc::new(MovingSphere::new(center1,center2,0.0,1.0,50.0,moving_sphere_material))));

    objects.add(Some(Arc::new(Sphere::new(Point3::new(260.0,150.0,45.0),50.0,Some(Arc::new(Dielectric::new(1.5)))))));
    objects.add(Some(Arc::new(Sphere::new(Point3::new(0.0,150.0,145.0),50.0,Some(Arc::new(Medal::new(&Color1::new(0.8,0.8,0.9),1.0)))))));

    let mut boundary:Option<Arc<dyn Hittable>> = Some(Arc::new(Sphere::new(Point3::new(360.0,150.0,145.0),70.0,Some(Arc::new(Dielectric::new(1.5))))));
    objects.add(boundary.clone());
    objects.add(Some(Arc::new(ConstantMedium::new1(boundary,0.2,Color1::new(0.2,0.4,0.9)))));
    boundary = Some(Arc::new(Sphere::new(Point3::new(0.0,0.0,0.0),5000.0,Some(Arc::new(Dielectric::new(1.5))))));
    objects.add(Some(Arc::new(ConstantMedium::new1(boundary,0.0001,Color1::new(1.0,1.0,1.0)))));

    let emat:Option<Arc<dyn Material>> = Some(Arc::new(Lambertian::new1(Some(Arc::new(ImageTecture::new("earthmap.jpg"))))));
    objects.add(Some(Arc::new(Sphere::new(Point3::new(400.0,200.0,400.0),100.0,emat))));
    let pertext: Option<Arc<dyn Texture>> = Some(Arc::new(NoiseTexture::new(0.1)));
    objects.add(Some(Arc::new(Sphere::new(Point3::new(220.0,280.0,300.0),80.0, Some(Arc::new(Lambertian::new1(pertext)))))));

    let mut boxes2 = HittableList::new();
    let white:Option<Arc<dyn Material>> = Some(Arc::new(Lambertian::new(&Color1::new(0.73,0.73,0.73))));
    let ns = 1000;
    for j in 0..ns{
        boxes2.add(Some(Arc::new(Sphere::new(Point3::random1(0.0,165.0),10.0,white.clone()))));
    }

    objects.add(Some(Arc::new(Translate::new(Some(Arc::new(RotateY::new(Some(Arc::new(BvhNode::new1(&mut boxes2,0.0,1.0))),15.0))),&Vec3::new(-100.0,270.0,395.0)))));

    objects
}

fn main() {
    // get environment variable CI, which is true for GitHub Actions
    let is_ci = is_ci();

    println!("CI: {}", is_ci);

    // Image
    let mut aspect_ratio = 16.0 / 9.0;
    let mut width: usize = 400;
    let path = "output/test.jpg";
    let quality = 60; // From 0 to 100, suggested value: 60
    let mut samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let world;
    let lookfrom;
    let lookat;
    let vfov;
    let mut aperture = 0.0;
    let mut background = Vec3::new(0.0,0.0,0.0);

    match 0 {
        1 => {
            world = random_scene();
            background = Color1::new(0.70, 0.80, 1.00);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.1;
        }

        2 => {
            world = two_spheres();
            background = Color1::new(0.70, 0.80, 1.00);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }

        3 => {
            world = two_perlin_spheres();
            background = Color1::new(0.70, 0.80, 1.00);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }

        4 => {
            world = earth();
            background = Color1::new(0.70, 0.80, 1.00);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }

        5 => {
            world = simple_light();
            samples_per_pixel = 400;
            background = Color1::new(0.0, 0.0, 0.0);
            lookfrom = Point3::new(26.0, 3.0, 6.0);
            lookat = Point3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
        }

        6 => {
            world = cornell_box();
            aspect_ratio = 1.0;
            width = 600;
            samples_per_pixel = 200;
            background = Color1::new(0.0, 0.0, 0.0);
            lookfrom = Point3::new(278.0, 278.0, -800.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }

        7 => {
            world = cornell_smoke();
            aspect_ratio = 1.0;
            width = 600;
            samples_per_pixel = 200;
            lookfrom = Point3::new(278.0,278.0,-800.0);
            lookat = Point3::new(278.0,278.0,0.0);
            vfov = 40.0;
        }

        _ => {
            world = final_scene();
            aspect_ratio = 1.0;
            width = 800;
            samples_per_pixel = 10000;
            background = Color1::new(0.0,0.0,0.0);
            lookfrom = Point3::new(478.0,278.0,-600.0);
            lookat = Point3::new(278.0,278.0,0.0);
            vfov = 40.0;
        }
    }

    let height = (width as f64 / aspect_ratio) as usize;
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

    // Camera
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;

    let cam = Camera::new(
        &lookfrom,
        &lookat,
        &vup,
        vfov,
        aspect_ratio,
        aperture,
        (dist_to_focus, 0.0, 1.0),
    );

    for j in 0..height {
        for i in 0..width {
            let mut pixel_color = Color1::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u = (i as f64 + random_f64()) / (width - 1) as f64;
                let v = (j as f64 + random_f64()) / (height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &background, &world, max_depth);
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
