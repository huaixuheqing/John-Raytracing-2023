use crate::vec3::Point3;
use crate::Hittable;

use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use std::sync::Arc;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Option<Arc<dyn Material>>,
}

impl Sphere {
    pub fn new(cen: Point3, r: f64, m: Option<Arc<dyn Material>>) -> Self {
        Self {
            center: cen,
            radius: r,
            mat_ptr: m,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc * r.direction();
        let c = oc.clone().length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        //Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = self.mat_ptr.clone();

        true
    }
}
