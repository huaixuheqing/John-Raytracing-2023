use crate::{perlin, vec3, Point3};
use perlin::Perlin;
use std::sync::Arc;
pub use vec3::Color1;

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
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color1 {
        return (*self).color_value;
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
            return (*self).odd.clone().unwrap().value(u, v, p);
        } else {
            return (*self).even.clone().unwrap().value(u, v, p);
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
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color1 {
        return Color1::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (*self).noise.noise(&(*p * (*self).scale)));
    }
}
