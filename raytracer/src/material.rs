use crate::hittable::HitRecord;
use crate::vec3::Vec3;
use crate::{random_f64, ray, texture, vec3, Point3};
pub use ray::Ray;

use std::sync::Arc;
pub use texture::SolidColor;
pub use texture::Texture;
use vec3::Color1;

pub trait Material {
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color1;

    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color1,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Option<Arc<dyn Texture>>,
}

impl Lambertian {
    pub fn new(a: &Color1) -> Self {
        Self {
            albedo: Some(Arc::new(SolidColor::new(*a))),
        }
    }
    pub fn new1(a: Option<Arc<dyn Texture>>) -> Self {
        Self { albedo: a }
    }
}

impl Material for Lambertian {
    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color1 {
        Color1::new(0.0, 0.0, 0.0)
    }

    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color1,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction, r_in.time());
        *attenuation = self.albedo.clone().unwrap().value(rec.u, rec.v, &rec.p);
        true
    }
}

pub struct Medal {
    albedo: Color1,
    fuzz: f64,
}

impl Medal {
    pub fn new(a: &Color1, f: f64) -> Self {
        let mut b = 1.0;
        if f < 1.0 {
            b = f;
        }
        Self {
            albedo: *a,
            fuzz: b,
        }
    }
}

impl Material for Medal {
    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color1 {
        Color1::new(0.0, 0.0, 0.0)
    }

    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color1,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(&r_in.direction().unit_vector().clone(), &rec.normal.clone());
        *scattered = Ray::new(
            rec.p,
            reflected + Vec3::random_in_unit_sphere() * self.fuzz,
            r_in.time(),
        );
        *attenuation = self.albedo;
        (scattered.direction() * rec.normal) > 0.0
    }
}

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }

    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color1 {
        Color1::new(0.0, 0.0, 0.0)
    }

    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color1,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color1::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction().unit_vector();

        let mut cos_theta = 1.0;
        if ((-unit_direction) * rec.normal) < 1.0 {
            cos_theta = (-unit_direction) * rec.normal;
        }
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = (refraction_ratio * sin_theta) > 1.0;
        let direction = if cannot_refract
            || (Dielectric::reflectance(cos_theta, refraction_ratio) > random_f64())
        {
            Vec3::reflect(&unit_direction, &rec.normal)
        } else {
            Vec3::refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        *scattered = Ray::new(rec.p, direction, r_in.time());
        true
    }
}

pub struct DiffuseLight {
    emit: Option<Arc<dyn Texture>>,
}

impl DiffuseLight {
    /*pub fn new(a: Option<Arc<dyn Texture>>) -> Self {
        Self { emit: a }
    }*/

    pub fn new1(c: Color1) -> Self {
        Self {
            emit: Some(Arc::new(SolidColor::new(c))),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &mut HitRecord,
        _attenuation: &mut Color1,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color1 {
        self.emit.clone().unwrap().value(u, v, p)
    }
}

pub struct Isotropic {
    albedo: Option<Arc<dyn Texture>>,
}

impl Isotropic {
    pub fn new(c: Color1) -> Self {
        Self {
            albedo: Some(Arc::new(SolidColor::new(c))),
        }
    }

    /*    pub fn new1(a:Option<Arc<dyn Texture>>) -> Self {
        Self{
            albedo:a,
        }
    }*/
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color1,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new(rec.p, Vec3::random_in_unit_sphere(), r_in.time());
        *attenuation = self.albedo.clone().unwrap().value(rec.u, rec.v, &rec.p);
        true
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color1 {
        Color1::new(0.0, 0.0, 0.0)
    }
}
