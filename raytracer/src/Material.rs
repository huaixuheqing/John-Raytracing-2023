use crate::vec3::Vec3;
use crate::Hittable::hit_record;
use crate::{random_f64, vec3, Ray};
use vec3::Color1;
pub use Ray::ray;

pub trait material {
    fn scatter(
        &self,
        r_in: &ray,
        rec: &mut hit_record,
        attenuation: &mut Color1,
        scattered: &mut ray,
    ) -> bool;
}

pub struct lambertian {
    albedo: Color1,
}

impl lambertian {
    pub fn new(a: &Color1) -> Self {
        Self { albedo: a.clone() }
    }
}

impl material for lambertian {
    fn scatter(
        &self,
        r_in: &ray,
        rec: &mut hit_record,
        attenuation: &mut Color1,
        scattered: &mut ray,
    ) -> bool {
        let mut scatter_direction = rec.normal.clone() + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }
        *scattered = ray::new(rec.p.clone(), scatter_direction);
        *attenuation = (*self).albedo.clone();
        return true;
    }
}

pub struct medal {
    albedo: Color1,
    fuzz: f64,
}

impl medal {
    pub fn new(a: &Color1, f: f64) -> Self {
        let mut b = 1.0;
        if f < 1.0 {
            b = f;
        }
        Self {
            albedo: a.clone(),
            fuzz: b,
        }
    }
}

impl material for medal {
    fn scatter(
        &self,
        r_in: &ray,
        rec: &mut hit_record,
        attenuation: &mut Color1,
        scattered: &mut ray,
    ) -> bool {
        let reflected = Vec3::reflect(&r_in.direction().unit_vector().clone(), &rec.normal.clone());
        *scattered = ray::new(
            rec.p.clone(),
            reflected + Vec3::random_in_unit_sphere() * (*self).fuzz,
        );
        *attenuation = (*self).albedo;
        return (scattered.direction() * rec.normal.clone()) > 0.0;
    }
}

pub struct dielectric {
    ir: f64,
}

impl dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }

    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
    }
}

impl material for dielectric {
    fn scatter(
        &self,
        r_in: &ray,
        rec: &mut hit_record,
        attenuation: &mut Color1,
        scattered: &mut ray,
    ) -> bool {
        *attenuation = Color1::new(1.0, 1.0, 1.0);
        let mut refraction_ratio = 0.0;
        if rec.front_face {
            refraction_ratio = 1.0 / (*self).ir;
        } else {
            refraction_ratio = (*self).ir;
        }

        let mut unit_direction = r_in.direction().unit_vector();

        let mut cos_theta = 1.0;
        if ((-unit_direction.clone()) * rec.normal.clone()) < 1.0 {
            cos_theta = ((-unit_direction.clone()) * rec.normal.clone());
        }
        let mut sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = (refraction_ratio * sin_theta) > 1.0;
        let mut direction = Vec3::new(0.0, 0.0, 0.0);

        if cannot_refract || (dielectric::reflectance(cos_theta, refraction_ratio) > random_f64()) {
            direction = Vec3::reflect(&unit_direction, &rec.normal);
        } else {
            direction = Vec3::refract(&unit_direction, &rec.normal, refraction_ratio);
        }

        *scattered = ray::new(rec.p, direction);
        return true;
    }
}
