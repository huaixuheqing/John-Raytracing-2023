use crate::rtweekend::clamp;
use crate::{perlin, vec3, Point3};

use image::GenericImageView;
use perlin::Perlin;
use std::sync::Arc;
pub use vec3::Color1;

static BYTES_PER_PIXEL: i32 = 3;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color1;
}

pub struct SolidColor {
    color_value: Color1,
}

impl SolidColor {
    pub fn new(c: Color1) -> Self {
        Self { color_value: c }
    }

    pub fn new1(red: f64, green: f64, blue: f64) -> Self {
        Self {
            color_value: Color1::new(red, green, blue),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color1 {
        self.color_value
    }
}

pub struct CheckerTexture {
    odd: Option<Arc<dyn Texture>>,
    even: Option<Arc<dyn Texture>>,
}

impl CheckerTexture {
    pub fn new(_even: Option<Arc<dyn Texture>>, _odd: Option<Arc<dyn Texture>>) -> Self {
        Self {
            even: _even,
            odd: _odd,
        }
    }

    pub fn new1(c1: Color1, c2: Color1) -> Self {
        Self {
            even: Some(Arc::new(SolidColor::new(c1))),
            odd: Some(Arc::new(SolidColor::new(c2))),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color1 {
        let sines = (p.x * 10.0).sin() * (p.y * 10.0).sin() * (p.z * 10.0).sin();
        if sines < 0.0 {
            self.odd.clone().unwrap().value(u, v, p)
        } else {
            self.even.clone().unwrap().value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(sc: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale: sc,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color1 {
        Color1::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (p.z * self.scale + 10.0 * self.noise.turb(p, 7)).sin())
    }
}

pub struct ImageTecture {
    data: Vec<u8>,
    width: i32,
    height: i32,
    bytes_per_scanline: i32,
}

impl ImageTecture {
    pub fn new(filename: &str) -> Self {
        let _components_per_pixel = BYTES_PER_PIXEL;
        let image = image::open(filename).expect("Failed to load image");
        let data1 = image.to_rgb8().into_vec();
        Self {
            data: data1,
            width: image.width() as i32,
            height: image.height() as i32,
            bytes_per_scanline: BYTES_PER_PIXEL * image.width() as i32,
        }
    }

    pub fn get_pixel(&self, mut i: i32, mut j: i32) -> Color1 {
        i = i.min(self.width - 1);
        j = j.min(self.height - 1);
        let index = (j * self.bytes_per_scanline + i * BYTES_PER_PIXEL) as usize;
        let color_scale = 1.0 / 255.0;
        let r = self.data[index] as f64 * color_scale;
        let g = self.data[index + 1] as f64 * color_scale;
        let b = self.data[index + 2] as f64 * color_scale;
        Color1::new(r, g, b)
    }
}

impl Texture for ImageTecture {
    fn value(&self, mut u: f64, mut v: f64, _p: &Point3) -> Color1 {
        u = clamp(u, 0.0, 1.0);
        v = 1.0 - clamp(v, 0.0, 1.0);

        let i = (u * self.width as f64) as i32;
        let j = (v * self.height as f64) as i32;

        self.get_pixel(i, j)
    }
}
